import { ApiPromise, WsProvider } from '@polkadot/api';
import { defaultTxOptions as txOptions, get_create_collection_data, getApiPromise } from '../util';
import { AccountsManager } from '../util/accounts';
import { StateInitializer } from '../util/state';
import { CompleteCallback, ensure_success_promise } from '../util/txResult';
import { stringToUnicodeArr } from '../util';

describe("Refungible::initCollection", function () {
    let api: ApiPromise;
    let state: StateInitializer;
    let accounts: AccountsManager;

    before(async function () {
        api = await getApiPromise();
        state = new StateInitializer(api);

        accounts = new AccountsManager(api);
    });

    it('documentation example works', function () {
        return new Promise(async function(resolve, reject) {
            // We are using some testing helper objects and functions such as accounts, state, stringToUnicodeArr...
            // They are not a part of API and used here to explicitly prepare required state without extra details

            // just creating an account with some tokens to pay the creation and transaction fees
            const [alice] = await accounts.deriveNewAccounts([1000n]);

            // only whitelisted admins are allowed to create collections
            // explicitly making alice an admin for example
            await state.setWhitelistAdmins([alice.address], accounts.root);

            const collectionData = {
                // for this pallet must be 'refungible'
                mode: 'refungible',
                name: stringToUnicodeArr('Collection name'),
                description: stringToUnicodeArr('Collection description'),
                // Yes, token prefix could be passed as a string unlike name and description
                // See CreateCollectionData docs
                tokenPrefix: 'ECT',
                properties: [
                    {
                        key: 'Key', 
                        value: 'Value'
                    }
                ],
                propertyPermissions: [
                    // let mutate the 'Key' property specified above
                    {
                        key: 'Key',
                        permission: {
                            mutable: true,
                        }
                    },
                    // let add this property later
                    {
                        key: 'NewKey',
                        permission: {
                            mutable: true,
                        }
                    }
                ]
            }
            
            // init a collection
            const unsub: CompleteCallback = await api.tx.refungible
                .initCollection(collectionData)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });

    it('init_collection works', function () {
        return new Promise(async function(resolve, reject) {
            const [alice] = await accounts.deriveNewAccounts([1000n]);
            await state.setWhitelistAdmins([alice.address], accounts.root);

            const collectionData = get_create_collection_data({ mode: 'refungible' });
            const unsub: CompleteCallback = await api.tx.refungible
                .initCollection(collectionData)
                .signAndSend(alice, txOptions, (result) => ensure_success_promise(result, unsub, {resolve, reject}));
        });
    });
});