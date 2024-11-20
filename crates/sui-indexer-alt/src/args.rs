// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::benchmark::BenchmarkConfig;
use crate::db::DbConfig;
use crate::pipeline::sequential::config::SequentialPipelineConfig;
use crate::IndexerConfig;
use clap::Subcommand;

#[derive(clap::Parser, Debug, Clone)]
pub struct Args {
    #[command(flatten)]
    pub db_config: DbConfig,

    #[command(subcommand)]
    pub command: Command,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    /// Run the indexer.
    Indexer {
        #[command(flatten)]
        indexer: IndexerConfig,

        #[command(flatten)]
        sequential_pipeline_config: SequentialPipelineConfig,
    },

    /// Wipe the database of its contents
    ResetDatabase {
        /// If true, only drop all tables but do not run the migrations.
        /// That is, no tables will exist in the DB after the reset.
        #[clap(long, default_value_t = false)]
        skip_migrations: bool,
    },

    Benchmark {
        #[command(flatten)]
        config: BenchmarkConfig,
    },
}
