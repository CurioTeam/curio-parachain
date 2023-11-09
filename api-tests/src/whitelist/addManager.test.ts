import { ApiPromise, WsProvider } from '@polkadot/api';
import { 
    AccountsManager, CompleteCallback, defaultTxOptions as txOptions, 
    ensure_success_promise, StateInitializer, getApiPromise
} from '../util';

describe("Whitelist::addManager", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('addMananger documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            // make alice an admin
            await state.setWhitelistAdmins([alice.address], accounts.root);

            // add bob to manager list
            const unsub: CompleteCallback = await api.tx.whitelist
                .addManager(bob.address)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('addMananger works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);

            const unsub: CompleteCallback = await api.tx.whitelist
                .addManager(bob.address)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});