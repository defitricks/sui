// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use std::{collections::BTreeMap, sync::Arc};

use mysten_metrics::monitored_mpsc::{channel, Receiver, Sender};
use parking_lot::Mutex;
use tap::tap::TapFallible;
use thiserror::Error;
use tokio::sync::oneshot;
use tracing::{error, warn};

use crate::{
    block::{BlockRef, Transaction, TransactionIndex},
    context::Context,
    Round,
};

/// The maximum number of transactions pending to the queue to be pulled for block proposal
const MAX_PENDING_TRANSACTIONS: usize = 2_000;

/// The guard acts as an acknowledgment mechanism for the inclusion of the transactions to a block.
/// When its last transaction is included to a block then `included_in_block_ack` will be signalled.
/// If the guard is dropped without getting acknowledged that means the transactions have not been
/// included to a block and the consensus is shutting down.
pub(crate) struct TransactionsGuard {
    // Holds a list of transactions to be included in the block.
    // A TransactionsGuard may be partially consumed by `TransactionConsumer`, in which case, this holds the remaining transactions.
    transactions: Vec<Transaction>,

    included_in_block_ack: oneshot::Sender<(BlockRef, oneshot::Receiver<BlockStatus>)>,
}

/// The TransactionConsumer is responsible for fetching the next transactions to be included for the block proposals.
/// The transactions are submitted to a channel which is shared between the TransactionConsumer and the TransactionClient
/// and are pulled every time the `next` method is called.
pub(crate) struct TransactionConsumer {
    context: Arc<Context>,
    tx_receiver: Receiver<TransactionsGuard>,
    max_consumed_bytes_per_request: u64,
    max_consumed_transactions_per_request: u64,
    pending_transactions: Option<TransactionsGuard>,
    block_status_subscribers: Arc<Mutex<BTreeMap<BlockRef, Vec<oneshot::Sender<BlockStatus>>>>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(unused)]
pub enum BlockStatus {
    /// The block has been sequenced as part of a committed sub dag. That means that any transaction that has been included in the block
    /// has been committed as well.
    Sequenced(BlockRef),
    /// The block has been garbage collected and will never be committed. Any transactions that have been included in the block should also
    /// be considered as impossible to be committed as part of this block and might need to be retried
    GarbageCollected(BlockRef),
}

impl TransactionConsumer {
    pub(crate) fn new(tx_receiver: Receiver<TransactionsGuard>, context: Arc<Context>) -> Self {
        Self {
            tx_receiver,
            max_consumed_bytes_per_request: context
                .protocol_config
                .max_transactions_in_block_bytes(),
            max_consumed_transactions_per_request: context
                .protocol_config
                .max_num_transactions_in_block(),
            context,
            pending_transactions: None,
            block_status_subscribers: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    // Attempts to fetch the next transactions that have been submitted for sequence. Also a `max_consumed_bytes_per_request` parameter
    // is given in order to ensure up to `max_consumed_bytes_per_request` bytes of transactions are retrieved.
    // This returns one or more transactions to be included in the block and a callback to acknowledge the inclusion of those transactions.
    // Note that a TransactionsGuard may be partially consumed and the rest saved for the next pull, in which case its `included_in_block_ack`
    // will not be signalled in the callback.
    pub(crate) fn next(&mut self) -> (Vec<Transaction>, Box<dyn FnOnce(BlockRef)>) {
        let mut transactions = Vec::new();
        let mut acks = Vec::new();
        let mut total_size: usize = 0;

        // Handle one batch of incoming transactions from TransactionGuard.
        // Returns the remaining txs as a new TransactionGuard, if the batch breaks any limit.
        let mut handle_txs = |t: TransactionsGuard| -> Option<TransactionsGuard> {
            let remaining_txs: Vec<_> = t
                .transactions
                .into_iter()
                .filter_map(|tx| {
                    if (total_size + tx.data().len()) as u64 > self.max_consumed_bytes_per_request
                        || transactions.len() as u64 >= self.max_consumed_transactions_per_request
                    {
                        // Adding this tx would exceed the size limit or the number of txs limit, cache it for the next pull.
                        Some(tx)
                    } else {
                        total_size += tx.data().len();
                        transactions.push(tx);
                        None
                    }
                })
                .collect();

            if remaining_txs.is_empty() {
                // The batch has been fully consumed, register its ack.
                // In case a batch gets split, ack shall only be sent when the last transaction is included in the block.
                acks.push(t.included_in_block_ack);
                None
            } else {
                // If we went over the any limit while processing the batch, return the remainings.
                // It is the caller's responsibility to cache it for the next pull.
                Some(TransactionsGuard {
                    transactions: remaining_txs,
                    included_in_block_ack: t.included_in_block_ack,
                })
            }
        };

        if let Some(t) = self.pending_transactions.take() {
            self.pending_transactions = handle_txs(t);
        }

        // Until we have reached the limit for the pull.
        // We may have already reached limit in the first iteration above, in which case we stop immediately.
        while self.pending_transactions.is_none() {
            if let Ok(t) = self.tx_receiver.try_recv() {
                self.pending_transactions = handle_txs(t);
            } else {
                break;
            }
        }

        let block_status_subscribers = self.block_status_subscribers.clone();
        let gc_enabled = self.context.protocol_config.gc_depth() > 0;
        (
            transactions,
            Box::new(move |block_ref: BlockRef| {
                let mut block_status_subscribers = block_status_subscribers.lock();

                for ack in acks {
                    let (status_tx, status_rx) = oneshot::channel();

                    if gc_enabled {
                        block_status_subscribers
                            .entry(block_ref)
                            .or_default()
                            .push(status_tx);
                    } else {
                        // When gc is not enabled, then report directly the block as sequenced while tx is acknowledged for inclusion.
                        // As blocks can never get garbage collected it is there is actually no meaning to do otherwise and also is safer for edge cases.
                        status_tx.send(BlockStatus::Sequenced(block_ref)).ok();
                    }

                    let _ = ack.send((block_ref, status_rx));
                }
            }),
        )
    }

    /// Notifies all the transaction submitters who are waiting to receive an update on the status of the block.
    /// The `committed_blocks` are the blocks that have been committed and the `gc_round` is the round up to which the blocks have been garbage collected.
    /// First we'll notify for all the committed blocks, and then for all the blocks that have been garbage collected.
    pub(crate) fn notify_own_blocks_status(
        &self,
        committed_blocks: Vec<BlockRef>,
        gc_round: Round,
    ) {
        // Notify for all the committed blocks first
        let mut block_status_subscribers = self.block_status_subscribers.lock();
        for block_ref in committed_blocks {
            if let Some(subscribers) = block_status_subscribers.remove(&block_ref) {
                subscribers.into_iter().for_each(|s| {
                    let _ = s.send(BlockStatus::Sequenced(block_ref));
                });
            }
        }

        // Now notify everyone <= gc_round that their block has been garbage collected and clean up the entries
        while let Some((block_ref, subscribers)) = block_status_subscribers.pop_first() {
            if block_ref.round <= gc_round {
                subscribers.into_iter().for_each(|s| {
                    let _ = s.send(BlockStatus::GarbageCollected(block_ref));
                });
            } else {
                block_status_subscribers.insert(block_ref, subscribers);
                break;
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn subscribe_for_block_status_testing(
        &self,
        block_ref: BlockRef,
    ) -> oneshot::Receiver<BlockStatus> {
        let (tx, rx) = oneshot::channel();
        let mut block_status_subscribers = self.block_status_subscribers.lock();
        block_status_subscribers
            .entry(block_ref)
            .or_default()
            .push(tx);
        rx
    }

    #[cfg(test)]
    fn is_empty(&mut self) -> bool {
        if self.pending_transactions.is_some() {
            return false;
        }
        if let Ok(t) = self.tx_receiver.try_recv() {
            self.pending_transactions = Some(t);
            return false;
        }
        true
    }
}

#[derive(Clone)]
pub struct TransactionClient {
    sender: Sender<TransactionsGuard>,
    max_transaction_size: u64,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Failed to submit transaction, consensus is shutting down: {0}")]
    ConsensusShuttingDown(String),

    #[error("Transaction size ({0}B) is over limit ({1}B)")]
    OversizedTransaction(u64, u64),
}

impl TransactionClient {
    pub(crate) fn new(context: Arc<Context>) -> (Self, Receiver<TransactionsGuard>) {
        let (sender, receiver) = channel("consensus_input", MAX_PENDING_TRANSACTIONS);

        (
            Self {
                sender,
                max_transaction_size: context.protocol_config.max_transaction_size_bytes(),
            },
            receiver,
        )
    }

    /// Submits a list of transactions to be sequenced. The method returns when all the transactions have been successfully included
    /// to next proposed blocks.
    pub async fn submit(
        &self,
        transactions: Vec<Vec<u8>>,
    ) -> Result<(BlockRef, oneshot::Receiver<BlockStatus>), ClientError> {
        // TODO: Support returning the block refs for transactions that span multiple blocks
        let included_in_block = self.submit_no_wait(transactions).await?;
        included_in_block
            .await
            .tap_err(|e| warn!("Transaction acknowledge failed with {:?}", e))
            .map_err(|e| ClientError::ConsensusShuttingDown(e.to_string()))
    }

    /// Submits a list of transactions to be sequenced.
    /// If any transaction's length exceeds `max_transaction_size`, no transaction will be submitted.
    /// That shouldn't be the common case as sizes should be aligned between consensus and client. The method returns
    /// a receiver to wait on until the transactions has been included in the next block to get proposed. The consumer should
    /// wait on it to consider as inclusion acknowledgement. If the receiver errors then consensus is shutting down and transaction
    /// has not been included to any block.
    /// If multiple transactions are submitted, the receiver will be signalled when the last transaction is included in the block.
    pub(crate) async fn submit_no_wait(
        &self,
        transactions: Vec<Vec<u8>>,
    ) -> Result<oneshot::Receiver<(BlockRef, oneshot::Receiver<BlockStatus>)>, ClientError> {
        let (included_in_block_ack_send, included_in_block_ack_receive) = oneshot::channel();
        for transaction in &transactions {
            if transaction.len() as u64 > self.max_transaction_size {
                return Err(ClientError::OversizedTransaction(
                    transaction.len() as u64,
                    self.max_transaction_size,
                ));
            }
        }

        let t = TransactionsGuard {
            transactions: transactions.into_iter().map(Transaction::new).collect(),
            included_in_block_ack: included_in_block_ack_send,
        };
        self.sender
            .send(t)
            .await
            .tap_err(|e| error!("Submit transactions failed with {:?}", e))
            .map_err(|e| ClientError::ConsensusShuttingDown(e.to_string()))?;
        Ok(included_in_block_ack_receive)
    }
}

/// `TransactionVerifier` implementation is supplied by Sui to validate transactions in a block,
/// before acceptance of the block.
#[async_trait::async_trait]
pub trait TransactionVerifier: Send + Sync + 'static {
    /// Determines if this batch of transactions is valid.
    /// Fails if any one of the transactions is invalid.
    fn verify_batch(&self, batch: &[&[u8]]) -> Result<(), ValidationError>;

    /// Returns indices of transactions to reject, validator error over transactions.
    /// Currently only uncertified user transactions can be rejected. The rest of transactions
    /// are implicitly voted to be accepted.
    /// When the result is an error, the whole block should be rejected from local DAG instead.
    async fn verify_and_vote_batch(
        &self,
        batch: &[&[u8]],
    ) -> Result<Vec<TransactionIndex>, ValidationError>;
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
}

/// `NoopTransactionVerifier` accepts all transactions.
#[cfg(test)]
pub(crate) struct NoopTransactionVerifier;

#[cfg(test)]
#[async_trait::async_trait]
impl TransactionVerifier for NoopTransactionVerifier {
    fn verify_batch(&self, _batch: &[&[u8]]) -> Result<(), ValidationError> {
        Ok(())
    }

    async fn verify_and_vote_batch(
        &self,
        _batch: &[&[u8]],
    ) -> Result<Vec<TransactionIndex>, ValidationError> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use consensus_config::AuthorityIndex;
    use futures::{stream::FuturesUnordered, StreamExt};
    use sui_protocol_config::ProtocolConfig;
    use tokio::time::timeout;

    use crate::{
        block::{BlockDigest, BlockRef},
        context::Context,
        transaction::{BlockStatus, TransactionClient, TransactionConsumer},
    };

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn basic_submit_and_consume() {
        let _guard = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
            config.set_consensus_max_transaction_size_bytes_for_testing(2_000); // 2KB
            config.set_consensus_max_transactions_in_block_bytes_for_testing(2_000);
            config
        });

        let context = Arc::new(Context::new_for_test(4).0);
        let (client, tx_receiver) = TransactionClient::new(context.clone());
        let mut consumer = TransactionConsumer::new(tx_receiver, context.clone());

        // submit asynchronously the transactions and keep the waiters
        let mut included_in_block_waiters = FuturesUnordered::new();
        for i in 0..3 {
            let transaction =
                bcs::to_bytes(&format!("transaction {i}")).expect("Serialization should not fail.");
            let w = client
                .submit_no_wait(vec![transaction])
                .await
                .expect("Shouldn't submit successfully transaction");
            included_in_block_waiters.push(w);
        }

        // now pull the transactions from the consumer
        let (transactions, ack_transactions) = consumer.next();
        assert_eq!(transactions.len(), 3);

        for (i, t) in transactions.iter().enumerate() {
            let t: String = bcs::from_bytes(t.data()).unwrap();
            assert_eq!(format!("transaction {i}").to_string(), t);
        }

        assert!(
            timeout(Duration::from_secs(1), included_in_block_waiters.next())
                .await
                .is_err(),
            "We should expect to timeout as none of the transactions have been acknowledged yet"
        );

        // Now acknowledge the inclusion of transactions
        ack_transactions(BlockRef::MIN);

        // Now make sure that all the waiters have returned
        while let Some(result) = included_in_block_waiters.next().await {
            assert!(result.is_ok());
        }

        // try to pull again transactions, result should be empty
        assert!(consumer.is_empty());
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn block_status_update_gc_enabled() {
        let _guard = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
            config.set_consensus_max_transaction_size_bytes_for_testing(2_000); // 2KB
            config.set_consensus_max_transactions_in_block_bytes_for_testing(2_000);
            config.set_consensus_gc_depth_for_testing(10);
            config
        });

        let context = Arc::new(Context::new_for_test(4).0);
        let (client, tx_receiver) = TransactionClient::new(context.clone());
        let mut consumer = TransactionConsumer::new(tx_receiver, context.clone());

        // submit the transactions and include 2 of each on a new block
        let mut included_in_block_waiters = FuturesUnordered::new();
        for i in 1..=10 {
            let transaction =
                bcs::to_bytes(&format!("transaction {i}")).expect("Serialization should not fail.");
            let w = client
                .submit_no_wait(vec![transaction])
                .await
                .expect("Shouldn't submit successfully transaction");
            included_in_block_waiters.push(w);

            // Every 2 transactions simulate the creation of a new block and acknowledge the inclusion of the transactions
            if i % 2 == 0 {
                let (transactions, ack_transactions) = consumer.next();
                assert_eq!(transactions.len(), 2);
                ack_transactions(BlockRef::new(
                    i,
                    AuthorityIndex::new_for_test(0),
                    BlockDigest::MIN,
                ));
            }
        }

        // Now iterate over all the waiters. Everyone should have been acknowledged.
        let mut block_status_waiters = Vec::new();
        while let Some(result) = included_in_block_waiters.next().await {
            let (block_ref, block_status_waiter) =
                result.expect("Block inclusion waiter shouldn't fail");
            block_status_waiters.push((block_ref, block_status_waiter));
        }

        // Now acknowledge the commit of the blocks 6, 8, 10 and set gc_round = 5, which should trigger the garbage collection of blocks 1..=5
        let gc_round = 5;
        consumer.notify_own_blocks_status(
            vec![
                BlockRef::new(6, AuthorityIndex::new_for_test(0), BlockDigest::MIN),
                BlockRef::new(8, AuthorityIndex::new_for_test(0), BlockDigest::MIN),
                BlockRef::new(10, AuthorityIndex::new_for_test(0), BlockDigest::MIN),
            ],
            gc_round,
        );

        // Now iterate over all the block status waiters. Everyone should have been notified.
        for (block_ref, waiter) in block_status_waiters {
            let block_status = waiter.await.expect("Block status waiter shouldn't fail");

            if block_ref.round <= gc_round {
                assert!(matches!(block_status, BlockStatus::GarbageCollected(_)))
            } else {
                assert!(matches!(block_status, BlockStatus::Sequenced(_)));
            }
        }

        // Ensure internal structure is clear
        assert!(consumer.block_status_subscribers.lock().is_empty());
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn block_status_update_gc_disabled() {
        let _guard = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
            config.set_consensus_max_transaction_size_bytes_for_testing(2_000); // 2KB
            config.set_consensus_max_transactions_in_block_bytes_for_testing(2_000);
            config
        });

        let (mut context, _keys) = Context::new_for_test(4);
        context.protocol_config.set_consensus_gc_depth_for_testing(0);
        let context = Arc::new(context);
        let (client, tx_receiver) = TransactionClient::new(context.clone());
        let mut consumer = TransactionConsumer::new(tx_receiver, context.clone());

        // submit the transactions and include 2 of each on a new block
        let mut included_in_block_waiters = FuturesUnordered::new();
        for i in 1..=10 {
            let transaction =
                bcs::to_bytes(&format!("transaction {i}")).expect("Serialization should not fail.");
            let w = client
                .submit_no_wait(vec![transaction])
                .await
                .expect("Shouldn't submit successfully transaction");
            included_in_block_waiters.push(w);

            // Every 2 transactions simulate the creation of a new block and acknowledge the inclusion of the transactions
            if i % 2 == 0 {
                let (transactions, ack_transactions) = consumer.next();
                assert_eq!(transactions.len(), 2);
                ack_transactions(BlockRef::new(
                    i,
                    AuthorityIndex::new_for_test(0),
                    BlockDigest::MIN,
                ));
            }
        }

        // Now iterate over all the waiters. Everyone should have been acknowledged.
        let mut block_status_waiters = Vec::new();
        while let Some(result) = included_in_block_waiters.next().await {
            let (block_ref, block_status_waiter) =
                result.expect("Block inclusion waiter shouldn't fail");
            block_status_waiters.push((block_ref, block_status_waiter));
        }

        // Now iterate over all the block status waiters. Everyone should have been notified and everyone should be considered sequenced.
        for (_block_ref, waiter) in block_status_waiters {
            let block_status = waiter.await.expect("Block status waiter shouldn't fail");
            assert!(matches!(block_status, BlockStatus::Sequenced(_)));
        }

        // Ensure internal structure is clear
        assert!(consumer.block_status_subscribers.lock().is_empty());
    }

    #[tokio::test]
    async fn submit_over_max_fetch_size_and_consume() {
        let _guard = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
            config.set_consensus_max_transaction_size_bytes_for_testing(100);
            config.set_consensus_max_transactions_in_block_bytes_for_testing(100);
            config
        });

        let context = Arc::new(Context::new_for_test(4).0);
        let (client, tx_receiver) = TransactionClient::new(context.clone());
        let mut consumer = TransactionConsumer::new(tx_receiver, context.clone());

        // submit some transactions
        for i in 0..10 {
            let transaction =
                bcs::to_bytes(&format!("transaction {i}")).expect("Serialization should not fail.");
            let _w = client
                .submit_no_wait(vec![transaction])
                .await
                .expect("Shouldn't submit successfully transaction");
        }

        // now pull the transactions from the consumer
        let mut all_transactions = Vec::new();
        let (transactions, _ack_transactions) = consumer.next();
        assert_eq!(transactions.len(), 7);

        // ensure their total size is less than `max_bytes_to_fetch`
        let total_size: u64 = transactions.iter().map(|t| t.data().len() as u64).sum();
        assert!(
            total_size <= context.protocol_config.max_transactions_in_block_bytes(),
            "Should have fetched transactions up to {}",
            context.protocol_config.max_transactions_in_block_bytes()
        );
        all_transactions.extend(transactions);

        // try to pull again transactions, next should be provided
        let (transactions, _ack_transactions) = consumer.next();
        assert_eq!(transactions.len(), 3);

        // ensure their total size is less than `max_bytes_to_fetch`
        let total_size: u64 = transactions.iter().map(|t| t.data().len() as u64).sum();
        assert!(
            total_size <= context.protocol_config.max_transactions_in_block_bytes(),
            "Should have fetched transactions up to {}",
            context.protocol_config.max_transactions_in_block_bytes()
        );
        all_transactions.extend(transactions);

        // try to pull again transactions, result should be empty
        assert!(consumer.is_empty());

        for (i, t) in all_transactions.iter().enumerate() {
            let t: String = bcs::from_bytes(t.data()).unwrap();
            assert_eq!(format!("transaction {i}").to_string(), t);
        }
    }

    #[tokio::test]
    async fn submit_large_batch_and_ack() {
        let _guard = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
            config.set_consensus_max_transaction_size_bytes_for_testing(100);
            config.set_consensus_max_transactions_in_block_bytes_for_testing(100);
            config
        });

        let context = Arc::new(Context::new_for_test(4).0);
        let (client, tx_receiver) = TransactionClient::new(context.clone());
        let mut consumer = TransactionConsumer::new(tx_receiver, context.clone());
        let mut all_receivers = Vec::new();
        // submit a few transactions individually.
        for i in 0..10 {
            let transaction =
                bcs::to_bytes(&format!("transaction {i}")).expect("Serialization should not fail.");
            let w = client
                .submit_no_wait(vec![transaction])
                .await
                .expect("Shouldn't submit successfully transaction");
            all_receivers.push(w);
        }

        // construct a over-size-limit batch and submit, which should get broken into smaller ones.
        {
            let transactions: Vec<_> = (10..32)
                .map(|i| {
                    bcs::to_bytes(&format!("transaction {i}"))
                        .expect("Serialization should not fail.")
                })
                .collect();
            let w = client
                .submit_no_wait(transactions)
                .await
                .expect("Shouldn't submit successfully transaction");
            all_receivers.push(w);
        }

        // submit another individual transaction.
        {
            let i = 32;
            let transaction =
                bcs::to_bytes(&format!("transaction {i}")).expect("Serialization should not fail.");
            let w = client
                .submit_no_wait(vec![transaction])
                .await
                .expect("Shouldn't submit successfully transaction");
            all_receivers.push(w);
        }

        // now pull the transactions from the consumer.
        // we expect all transactions are fetched in order, not missing any, and not exceeding the size limit.
        let mut all_transactions = Vec::new();
        let mut all_acks: Vec<Box<dyn FnOnce(BlockRef)>> = Vec::new();
        while !consumer.is_empty() {
            let (transactions, ack_transactions) = consumer.next();

            let total_size: u64 = transactions.iter().map(|t| t.data().len() as u64).sum();
            assert!(
                total_size <= context.protocol_config.max_transactions_in_block_bytes(),
                "Should have fetched transactions up to {}",
                context.protocol_config.max_transactions_in_block_bytes()
            );

            all_transactions.extend(transactions);
            all_acks.push(ack_transactions);
        }

        // verify the number of transactions as well as the content.
        assert_eq!(all_transactions.len(), 33);
        for (i, t) in all_transactions.iter().enumerate() {
            let t: String = bcs::from_bytes(t.data()).unwrap();
            assert_eq!(format!("transaction {i}").to_string(), t);
        }

        // now acknowledge the inclusion of all transactions.
        for ack in all_acks {
            ack(BlockRef::MIN);
        }

        // expect all receivers to be resolved.
        for w in all_receivers {
            let r = w.await;
            assert!(r.is_ok());
        }
    }
}
