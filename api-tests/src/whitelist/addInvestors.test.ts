import { ApiPromise, WsProvider } from '@polkadot/api';
import { 
    AccountsManager, CompleteCallback, defaultTxOptions as txOptions, 
    ensure_success_promise, StateInitializer, getApiPromise
} from '../util';

describe("Whitelist::addInvestors", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('addInvestors documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice, bob, eve] = await accounts.deriveNewAccounts([1000n, 0n, 0n,]);
            // make alice an admin
            await state.setWhitelistAdmins([alice.address], accounts.root);

            // prepate investors data
            const investors: [string, {account: string, isActive: boolean}][] = [
                [
                    'B'.repeat(32),
                    {
                        account: bob.address,
                        isActive: true
                    }
                ],
                [
                    'E'.repeat(32),
                    {
                        account: eve.address,
                        isActive: true
                    }
                ]
            ]

            // add bob and eve to investors list
            const addInvestorsUnSub: CompleteCallback = await api.tx.whitelist
                .addInvestors(investors)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, addInvestorsUnSub, {resolve, reject}));
        });
    });

    it('addInvestors works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice, bob, eve] = await accounts.deriveNewAccounts([1000n, 0n, 0n, 0n, 0n]);

            await state.setWhitelistAdmins([alice.address], accounts.root);

            const investors: [string, {account: string, isActive: boolean}][] = [
                [
                    bob.address.substring(0, 32),
                    {
                        account: bob.address,
                        isActive: true
                    }
                ],
                [
                    eve.address.substring(0, 32),
                    {
                        account: eve.address,
                        isActive: true
                    }
                ]
            ]
            const addInvestorsUnSub: CompleteCallback = await api.tx.whitelist
                .addInvestors(investors)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, addInvestorsUnSub, {resolve, reject}));
        });
    });
});