import { ApiPromise, WsProvider } from '@polkadot/api';
import { defaultTxOptions as txOptions, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success_promise } from '../util/txResult';

describe("Refungible::deleteTokenProperty(ies)", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);
        accounts = new AccountsManager(api);
    });

    it('deleteTokenProperty documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make the account a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // create a collection an get a collectionId from CollectionCreated event
            // mutableProperties explained in setCollectionProperty
            // property permissions are common for a collection and its tokens so passed here
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key'] });
            // prepare a propety
            const property = {key: 'Key', value: 'Value'};
            // create a token with some property an get a tokenId from ItemCreated event
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]], [property]);

            // delete token property
            const unsub: CompleteCallback = await api.tx.refungible
                .deleteTokenProperty(collectionId, tokenId, property.key)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('deleteTokenProperties documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make the account a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // create a collection an get a collectionId from CollectionCreated event
            // mutableProperties explained in setCollectionProperty
            // property permissions are common for a collection and its tokens so passed here
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key1', 'Key2'] });
            // prepare a propety
            const properties = [{key: 'Key1', value: 'Value1'}, {key: 'Key2', value: 'Value2'}];
            // create a token with some property an get a tokenId from ItemCreated event
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]], properties);
            // delete token properties
            const unsub: CompleteCallback = await api.tx.refungible
                .deleteTokenProperties(collectionId, tokenId, properties.map(p => p.key))
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('deleteTokenProperty works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key'] });
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]], [{key: 'Key', value: 'Value'}]);

            const unsub: CompleteCallback = await api.tx.refungible
                .deleteTokenProperty(collectionId, tokenId, 'Key')
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('deleteTokenProperties works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key1', 'Key2'] });
            const properties = [{key: 'Key1', value: 'Value1'}, {key: 'Key2', value: 'Value2'}];
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]], properties);

            const unsub: CompleteCallback = await api.tx.refungible
                .deleteTokenProperties(collectionId, tokenId, properties.map(p => p.key))
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});