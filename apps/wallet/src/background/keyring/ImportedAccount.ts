// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    normalizeSuiAddress,
    type Keypair,
    type SuiAddress,
} from '@mysten/sui.js';

import { type Account, AccountType } from './Account';
import { AccountKeypair } from './AccountKeypair';

export type SerializedImportedAccount = {
    type: AccountType.IMPORTED;
    address: SuiAddress;
    derivationPath: null;
};

export class ImportedAccount implements Account {
    accountKeypair: AccountKeypair;
    type: AccountType;
    address: string;

    constructor({ keypair }: { keypair: Keypair }) {
        this.type = AccountType.IMPORTED;
        this.accountKeypair = new AccountKeypair(keypair);
        this.address = normalizeSuiAddress(
            this.accountKeypair.publicKey.toSuiAddress()
        );
    }

    toJSON(): SerializedImportedAccount {
        return {
            type: AccountType.IMPORTED,
            address: this.address,
            derivationPath: null,
        };
    }
}
