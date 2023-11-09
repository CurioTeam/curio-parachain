import { ApiPromise, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { CompleteCallback, ensure_success_promise } from '.';

export class AccountsManager {
    private api: ApiPromise
    private seed: string
    private iteration: number
    private keyring: Keyring
    private nominal: bigint

    public root: KeyringPair

    constructor(api: ApiPromise, seed?: string) {
        this.api = api;
        this.seed = seed ?? Date.now().toString();
        
        this.iteration = 0
        this.keyring = new Keyring({ type: 'sr25519' });
        this.root = this.keyring.addFromUri('//Alice');

        const chainProperties = api.registry.getChainProperties();
        if(!chainProperties) throw Error('Failed to get chain properties');
        const decimals = chainProperties.tokenDecimals.unwrap()[0].toBigInt() ?? BigInt(18);
        this.nominal = BigInt(10) ** decimals;
    }

    async deriveNewAccounts(balances: bigint[]) {
        this.iteration += 1;
        const uriPrefix = `${this.seed}${this.iteration}`;

        const accounts = Array.from(Array(balances.length).keys(), id => this.keyring.addFromUri(uriPrefix + id));
        let nonce = (await this.api.rpc.system.accountNextIndex(this.root.address)).toBigInt();
        
        await Promise.all(balances.map((balance, index) => { 
            if(balance == 0n) return;

            return new Promise(async (resolve, reject) => {
                const unsub: CompleteCallback = await this.api.tx.sudo
                    .sudo(this.api.tx.balances.setBalance(accounts[index].address, this.nominal * balance, 0))
                    .signAndSend(this.root, {nonce: nonce++}, result => ensure_success_promise(result, unsub, {resolve, reject}));
            });
        }));

        return accounts;
    }
}