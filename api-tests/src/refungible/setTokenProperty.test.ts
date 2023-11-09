import { ApiPromise, WsProvider } from '@polkadot/api';
import { defaultTxOptions as txOptions, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success_promise } from '../util/txResult';

describe("Refungible::setTokenProperty(ies)", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('setTokenProperty documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make the account a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // create a collection an get a collectionId from CollectionCreated event
            // mutableProperties explained in setCollectionProperty
            // property permissions are common for a collection and its tokens so passed here
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key'] });
            // create a token an get a tokenId from ItemCreated event
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            // prepare a Property
            const property = {key: 'Key', value: 'Value'};
            // set token property
            const unsub: CompleteCallback = await api.tx.refungible
                .setTokenProperty(collectionId, tokenId, property)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('setTokenProperties documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make the account a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // create a collection an get a collectionId from CollectionCreated event
            // mutableProperties explained in setCollectionProperty
            // property permissions are common for a collection and its tokens so passed here
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key1', 'Key2'] });
            // create a token an get a tokenId from ItemCreated event
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            // prepare a Properties
            const properties = [{key: 'Key1', value: 'Value1'}, {key: 'Key2', value: 'Value2'}];
            // set token properties
            const unsub: CompleteCallback = await api.tx.refungible
                .setTokenProperties(collectionId, tokenId, properties)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('setTokenProperty works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key'] });
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            const unsub: CompleteCallback = await api.tx.refungible
                .setTokenProperty(collectionId, tokenId, {key: 'Key', value: 'Value'})
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('setTokenProperties works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key1', 'Key2'] });
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            const properties = [{key: 'Key1', value: 'Value1'}, {key: 'Key2', value: 'Value2'}];
            const unsub: CompleteCallback = await api.tx.refungible
                .setTokenProperties(collectionId, tokenId, properties)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});