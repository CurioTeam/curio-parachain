import { ApiPromise, WsProvider } from '@polkadot/api';
import { defaultTxOptions as txOptions, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success_promise } from '../util/txResult';

describe("Refungible::setPropertyPermission", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('setPropertyPermission documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make the account a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // create a collection an get a collectionId from CollectionCreated event
            // mutable_properties explained in setCollectionProperty
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key'] });

            // prepare a PropertyKeyPermission
            const propertyPermission = {
                key: 'Key',
                permission: {
                    mutable: false
                }
            };
            // set property permission
            const unsub: CompleteCallback = await api.tx.refungible
                .setPropertyPermission(collectionId, propertyPermission)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('setPropertyPermission works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const collectionId = await state.createCollection(alice, { mode: 'refungible', mutableProperties: ['Key'] });

            const propertyPermission = {
                key: 'Key',
                permission: {
                    mutable: false
                }
            };
            const unsub: CompleteCallback = await api.tx.refungible
                .setPropertyPermission(collectionId, propertyPermission)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});