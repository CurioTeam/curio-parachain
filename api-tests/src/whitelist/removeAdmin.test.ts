import { ApiPromise, WsProvider } from '@polkadot/api';
import { 
    AccountsManager, CompleteCallback, defaultTxOptions as txOptions, 
    ensure_success_promise, StateInitializer, getApiPromise
} from '../util';

describe("Whitelist::removeAdmin", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('removeAdmin documentation works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([0n]);
            // make alice a whitelist admin to remove later
            await state.setWhitelistAdmins([alice.address], accounts.root);

            // accounts.root is a RolesRoot of the whitelist pallet
            // remove alice from admin list
            const removeAdminUnSub: CompleteCallback = await api.tx.whitelist
                .removeAdmin(alice.address)
                .signAndSend(accounts.root, txOptions, (result) => ensure_success_promise(result, removeAdminUnSub, {resolve, reject}));
        });
    });

    it('removeAdmin works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([0n]);

            await state.setWhitelistAdmins([alice.address], accounts.root);

            const removeAdminUnSub: CompleteCallback = await api.tx.whitelist
                .removeAdmin(alice.address)
                .signAndSend(accounts.root, txOptions, (result) => ensure_success_promise(result, removeAdminUnSub, {resolve, reject}));
        });
    });
});