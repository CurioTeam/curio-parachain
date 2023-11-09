import { ApiPromise, WsProvider } from '@polkadot/api';
import { defaultTxOptions as txOptions, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success, ensure_success_promise } from '../util/txResult';

describe("Refungible::burn", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('burn documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get an account
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            // make alice a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // create a collection an get a collectionId from CollectionCreated event
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            // create a token and get a tokenId from ItemCreated event
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            // burn
            const unsub: CompleteCallback = await api.tx.refungible
                .burn(collectionId, tokenId, 100n)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('burnFrom documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 1000n]);
            // make alice a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // make future token holders are whitelisted investors
            await state.setWhitelistInvestors([bob.address], alice);
            // create a collection an get a collectionId from CollectionCreated event
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            // create a token and get a tokenId from ItemCreated event
            const tokenId = await state.createItem(alice, collectionId, [[bob.address, 100n]]);

            // set an allowance
            const approveUnSub: CompleteCallback = await api.tx.refungible
                .setAllowance(collectionId, tokenId, alice.address, 70n)
                .signAndSend(bob, txOptions, (result) => ensure_success(result, approveUnSub));
            // burn
            const unsub: CompleteCallback = await api.tx.refungible
                .burnFrom(collectionId, tokenId, bob.address, 70n)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('burn works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            // send a transaction
            const unsub: CompleteCallback = await api.tx.refungible
                .burn(collectionId, tokenId, 100n)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('burnFrom works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            await state.setWhitelistInvestors([bob.address], alice);
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            const tokenId = await state.createItem(alice, collectionId, [[bob.address, 100n]]);

            // send a transaction
            const approveUnSub: CompleteCallback = await api.tx.refungible
                .setAllowance(collectionId, tokenId, alice.address, 70n)
                .signAndSend(bob, txOptions, (result) => ensure_success(result, approveUnSub));

            const unsub: CompleteCallback = await api.tx.refungible
                .burnFrom(collectionId, tokenId, bob.address, 70n)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});