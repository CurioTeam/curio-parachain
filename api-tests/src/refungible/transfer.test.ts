import { ApiPromise, WsProvider } from '@polkadot/api';
import { defaultTxOptions as txOptions, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success, ensure_success_promise } from '../util/txResult';

describe("Refungible::transfer", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('transfer documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            // make alice a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // make future token holders are whitelisted investors
            await state.setWhitelistInvestors([bob.address], alice);
            // create a collection an get a collectionId from CollectionCreated event
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            // create a token and get a tokenId from ItemCreated event
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            // transfer
            const unsub: CompleteCallback = await api.tx.refungible
                .transfer(collectionId, tokenId, bob.address, 70n)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('transferFrom documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // get accounts
            const [alice, bob, eve] = await accounts.deriveNewAccounts([1000n, 1000n, 0n]);
            // make alice a whitelisted admin
            await state.setWhitelistAdmins([alice.address], accounts.root);
            // make future token holders are whitelisted investors
            await state.setWhitelistInvestors([bob.address, eve.address], alice);
            // create a collection an get a collectionId from CollectionCreated event
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            // create a token and get a tokenId from ItemCreated event
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            // set allowance to call transferFrom
            const approveUnSub: CompleteCallback = await api.tx.refungible
                .setAllowance(collectionId, tokenId, bob.address, 70n)
                .signAndSend(alice, txOptions, (result) => ensure_success(result, approveUnSub));

            // transfer
            const unsub: CompleteCallback = await api.tx.refungible
                .transferFrom(collectionId, tokenId, alice.address, eve.address, 70n)
                .signAndSend(bob, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('transfer works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice, bob] = await accounts.deriveNewAccounts([1000n, 0n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            await state.setWhitelistInvestors([bob.address], alice);
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            const unsub: CompleteCallback = await api.tx.refungible
                .transfer(collectionId, tokenId, bob.address, 70n)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('transferFrom works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice, bob, eve] = await accounts.deriveNewAccounts([1000n, 1000n, 0n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);
            await state.setWhitelistInvestors([bob.address, eve.address], alice);
            const collectionId = await state.createCollection(alice, { mode: 'refungible' });
            const tokenId = await state.createItem(alice, collectionId, [[alice.address, 100n]]);

            const approveUnSub: CompleteCallback = await api.tx.refungible
                .setAllowance(collectionId, tokenId, bob.address, 70n)
                .signAndSend(alice, txOptions, (result) => ensure_success(result, approveUnSub));

            const unsub: CompleteCallback = await api.tx.refungible
                .transferFrom(collectionId, tokenId, alice.address, eve.address, 70n)
                .signAndSend(bob, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});