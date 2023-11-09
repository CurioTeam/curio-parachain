import { ApiPromise, WsProvider } from '@polkadot/api';
import { 
    AccountsManager, CompleteCallback, defaultTxOptions as txOptions, 
    ensure_success_promise, StateInitializer, getApiPromise
} from '../util';

describe("Whitelist::addAdmin", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('addAdmin documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([0n]);
            
            // accounts.root is a RolesRoot of the whitelist pallet
            // add alice to admin list
            const unsub: CompleteCallback = await api.tx.whitelist
                .addAdmin(alice.address)
                .signAndSend(accounts.root, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('addAdmin works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([0n]);

            const unsub: CompleteCallback = await api.tx.whitelist
                .addAdmin(alice.address)
                .signAndSend(accounts.root, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});