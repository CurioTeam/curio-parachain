import { ApiPromise, WsProvider } from '@polkadot/api';
import { defaultTxOptions as txOptions, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success_promise } from '../util/txResult';
import { AccountId } from '@polkadot/types/interfaces';
import { u128, U128 } from '@polkadot/types';
import { createType } from '@polkadot/types';

describe("Refungible::createItem(s)", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('createItem documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            // make a collection creator a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // make bob a whitelisted investor
            // as only collection admins, collection owner and whitelisted investors are
            // allowed to have security tokens
            await state.setWhitelistInvestors([bob.address], alice);
            // create a collection an get a collectionId from CollectionCreated event
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });

            const data = {
                balances: [
                    // for the first account
                    [alice.address, 100n], 
                    // for the second account
                    [bob.address, 50n]
                    // for the third account could be...
                ],
                properties: [{key: 'Key', value: 'Value'}]
            };
            // create an item
            const unsub: CompleteCallback = await api.tx.refungible
                .createItem(collectionId, data)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('createMultipleItems documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            // make a collection creator a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // make bob a whitelisted investor
            // as only collection admins, collection owner and whitelisted investors are
            // allowed to have security tokens
            await state.setWhitelistInvestors([bob.address], alice);
            // create a collection an get a collectionId from CollectionCreated event
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });

            // tokens data
            const data = [
                // for the first token 
                {
                    balances: [[alice.address, 100n], [bob.address, 50n]],
                    properties: [{key: 'Key1', value: 'Value1'}]
                },
                // for the second token
                {
                    balances: [[alice.address, 30n], [bob.address, 20n]],
                    properties: [{key: 'Key2', value: 'Value2'}]
                }
                // for the third token could be...
            ];
        
            // create items
            const unsub: CompleteCallback = await api.tx.refungible
                .createMultipleItems(collectionId, data)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('createItem works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            await state.setWhitelistInvestors([bob.address], alice);
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            
            const data = {
                balances: [
                    [alice.address, 100n], 
                    [bob.address, 50n]
                ],
                properties: [{key: 'Key', value: 'Value'}]
            };

            const unsub: CompleteCallback = await api.tx.refungible
                .createItem(collectionId, data)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('createMultipleItems works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            await state.setWhitelistInvestors([bob.address], alice);
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });

            const data = [
                {
                    balances: [[alice.address, 100n], [bob.address, 50n]],
                    properties: [{key: 'Key1', value: 'Value1'}]
                },
                {
                    balances: [[alice.address, 30n], [bob.address, 20n]],
                    properties: [{key: 'Key2', value: 'Value2'}]
                }
            ];
        
            const unsub: CompleteCallback = await api.tx.refungible
                .createMultipleItems(collectionId, data)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});