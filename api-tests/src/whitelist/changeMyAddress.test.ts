import { ApiPromise } from '@polkadot/api';
import { 
    AccountsManager, CompleteCallback, defaultTxOptions as txOptions, 
    ensure_success_promise, StateInitializer, getApiPromise, ensure_success
} from '../util';

describe("Whitelist::changeMyAddress", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('changeMyAddress documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice, bob, newBob] = await accounts.deriveNewAccounts([1000n, 1000n, 0n]);
            // make alice an admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // make bob an investor
            await state.setWhitelistInvestors([bob.address], alice);

            // bob is changing his address to a new one
            const unsub: CompleteCallback = await api.tx.whitelist
                .changeMyAddress(newBob.address)
                .signAndSend(bob, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('changeMyAddress works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice, bob, eve] = await accounts.deriveNewAccounts([1000n, 1000n, 0n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            await state.setWhitelistInvestors([bob.address], alice);

            const unsub: CompleteCallback = await api.tx.whitelist
                .changeMyAddress(eve.address)
                .signAndSend(bob, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});