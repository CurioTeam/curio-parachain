import { ApiPromise, WsProvider } from '@polkadot/api';
import { defaultTxOptions as txOptions, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success_promise } from '../util/txResult';

describe("Refungible::setCollectionProperty(ies)", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('setCollectionProperty documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make the account a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // create a collection an get a collectionId from CollectionCreated event
            // mutable_properties field here let us add a property under 'Key' key later
            // under the hood it is passed to `propertyPermissions` from `CreateCollectionData` (see InitCollection)
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key'] });

            // prepare the a property
            const property = {key: 'Key', value: "Value"};
            // set collection property
            const unsub: CompleteCallback = await api.tx.refungible
                .setCollectionProperty(collectionId, property)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('setCollectionProperty works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key'] });

            const property = {key: 'Key', value: "Value"};
            const unsub: CompleteCallback = await api.tx.refungible
                .setCollectionProperty(collectionId, property)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('setCollectionProperties documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make the account a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // create a collection an get a collectionId from CollectionCreated event
            // mutable_properties field here let us add a property under passed keys later
            // under the hood it is passed to `propertyPermissions` from `CreateCollectionData` (see InitCollection)
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key1', 'Key2'] });

            // prepare the a properties array
            const properties = [{key: 'Key1', value: 'Value'}, {key: 'Key2', value: 'Value'}];
            // set collection properties
            const unsub: CompleteCallback = await api.tx.refungible
                .setCollectionProperties(collectionId, properties)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('setCollectionProperties works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key1', 'Key2'] });

            const properties = [{key: 'Key1', value: 'Value'}, {key: 'Key2', value: 'Value'}];
            const unsub: CompleteCallback = await api.tx.refungible
                .setCollectionProperties(collectionId, properties)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});