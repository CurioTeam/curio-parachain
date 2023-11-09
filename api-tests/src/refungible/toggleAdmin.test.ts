import { ApiPromise, WsProvider } from '@polkadot/api';
import { defaultTxOptions as txOptions, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success_promise } from '../util/txResult';

describe("Refungible::toggleAdmin", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('toggleAdmin documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            // make alice a whitelisted admin to create collection
            // also make bob a whitelisted admin as only whitelisted admins could be collection admins
            await state.setWhitelistAdmins([alice.address, bob.address], accounts.root);
            // create a collection an get a collectionId from CollectionCreated event
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });

            // add an admin
            const unsub: CompleteCallback = await api.tx.refungible
                .toggleAdmin(collectionId, bob.address, true)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('toggleAdmin works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            await state.setWhitelistAdmins([alice.address, bob.address], accounts.root);
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });

            const unsub: CompleteCallback = await api.tx.refungible
                .toggleAdmin(collectionId, bob.address, true)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});