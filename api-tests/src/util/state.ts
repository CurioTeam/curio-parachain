import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { defaultTxOptions } from '.';
import { CreateCollectionData, get_create_collection_data } from './collections';
import { CompleteCallback, ensure_success, ensure_success_promise, get_event_data } from './txResult';

export class StateInitializer {
    private api: ApiPromise;

    constructor(api: ApiPromise) {
        this.api = api;
    }

    async setWhitelistAdmins(admins: string[], root: KeyringPair) {
        let nonce = (await this.api.rpc.system.accountNextIndex(root.address)).toBigInt();

        return Promise.all(admins.map(async adminAddress => {
            const addAdminUnSub: CompleteCallback = await this.api.tx.whitelist
                .addAdmin(adminAddress)
                .signAndSend(root, {nonce: nonce++}, result => ensure_success(result, addAdminUnSub));
        }));
    }

    async setWhitelistManagers(managers: string[], admin: KeyringPair) {
        let nonce = (await this.api.rpc.system.accountNextIndex(admin.address)).toBigInt();

        return Promise.all(managers.map(async managerAddress => {
            const addManagerUnSub: CompleteCallback = await this.api.tx.whitelist
                .addManager(managerAddress)
                .signAndSend(admin, {nonce: nonce++}, result => ensure_success(result, addManagerUnSub));
        }));
    }

    async setWhitelistInvestors(investors: string[], root: KeyringPair) {
        const investorsData: ([string, { account?: any; isActive?: any }])[] = investors.map(investorAddress => {
            return [
                investorAddress.substring(0, 32),
                {
                    account: investorAddress,
                    isActive: true
                }
            ]
        });

        const addInvestorsUnSub: CompleteCallback = await this.api.tx.whitelist
            .addInvestors(investorsData)
            .signAndSend(root, defaultTxOptions, result => ensure_success(result, addInvestorsUnSub));
    }

    async createCollection(owner: KeyringPair, data: CreateCollectionData): Promise<bigint> {
        const result: SubmittableResult = await new Promise(async (resolve, reject) => {
            const collectionData = get_create_collection_data(data);
            const initCollectionUnSub: CompleteCallback = await this.api.tx.refungible
                .initCollection(collectionData)
                .signAndSend(owner, defaultTxOptions, result => ensure_success_promise(result, initCollectionUnSub, {resolve, reject}));
        });

        const eventData = get_event_data(result, 'common.CollectionCreated');
        return BigInt(eventData[0].toString());
    }

    async createItem(owner: KeyringPair, collectionId: bigint, balances: any[], properties: any[] = []): Promise<bigint> {
        const result: SubmittableResult = await new Promise(async (resolve, reject) => {
            const createItemUnSub: CompleteCallback = await this.api.tx.refungible
                .createItem(collectionId, {balances, properties})
                .signAndSend(owner, defaultTxOptions, result => ensure_success_promise(result, createItemUnSub, {resolve, reject}));
        });
        const eventData = get_event_data(result, 'common.ItemCreated');
        return BigInt(eventData[1].toString());
    }
}