import { ApiPromise } from '@polkadot/api';
import { 
    AccountsManager, CompleteCallback, defaultTxOptions as txOptions, 
    ensure_success_promise, StateInitializer, getApiPromise, ensure_success
} from '../util';

describe("Whitelist::removeManager", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('removeManager documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            // make alice an admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // make bob a manager to remove later
            await state.setWhitelistManagers([bob.address], alice);

            // remove bob from managers list
            const unsub: CompleteCallback = await api.tx.whitelist
                .removeManager(bob.address)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('removeManager works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            await state.setWhitelistManagers([bob.address], alice);

            const unsub: CompleteCallback = await api.tx.whitelist
                .removeManager(bob.address)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});