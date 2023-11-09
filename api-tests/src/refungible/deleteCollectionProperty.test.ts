import { ApiPromise } from '@polkadot/api';
import { defaultTxOptions as txOptions, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success_promise } from '../util/txResult';

describe("Refungible::deleteCollectionProperty(ies)", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('deleteCollectionProperty documentaion example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make the account a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // prepare a propery
            const property = {key: 'Key', value: 'Value'};
            // and init a collection with this property
            const collectionId = await state.createCollection(alice, { mode: 'refungible', properties: [property] });

            // delete the property
            const unsub: CompleteCallback = await api.tx.refungible
                .deleteCollectionProperty(collectionId, property.key)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('deleteCollectionProperty works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const property = {key: 'Key', value: 'Value'};
            const collectionId = await state.createCollection(alice, { mode: 'refungible', properties: [property] });

            
            const unsub: CompleteCallback = await api.tx.refungible
                .deleteCollectionProperty(collectionId, property.key)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('deleteCollectionProperties documentaion example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make the account a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // prepare a properties array
            const properties = [{key: 'Key1', value: 'Value'}, {key: 'Key2', value: 'Value'}];
            // and init a collection with these properties
            const collectionId = await state.createCollection(alice, { mode: 'refungible', properties });

            // delete collection properties
            const unsub: CompleteCallback = await api.tx.refungible
            .deleteCollectionProperties(collectionId, properties.map(p => p.key))
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('deleteCollectionProperties works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const properties = [{key: 'Key1', value: 'Value'}, {key: 'Key2', value: 'Value'}];
            const collectionId = await state.createCollection(alice, { mode: 'refungible', properties });

            const unsub: CompleteCallback = await api.tx.refungible
                .deleteCollectionProperties(collectionId, properties.map(p => p.key))
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});