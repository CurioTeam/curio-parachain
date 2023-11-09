import { ApiPromise, WsProvider } from '@polkadot/api';
import { defaultTxOptions as txOptions, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success_promise } from '../util/txResult';

describe("Refungible::repartition", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('repartition documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make alice a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // create a collection an get a collectionId from CollectionCreated event
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            // create a token and get a tokenId from ItemCreated event
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            // repartition
            const unsub: CompleteCallback = await api.tx.refungible
                .repartition(collectionId, tokenId, 50n)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('repartition works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            const unsub: CompleteCallback = await api.tx.refungible
                .repartition(collectionId, tokenId, 50n)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});