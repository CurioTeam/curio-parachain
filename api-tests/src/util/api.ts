import * as dotenv from 'dotenv';
import {WsProvider, ApiPromise} from '@polkadot/api';

export async function getApiPromise() {
    dotenv.config();
    if(!process.env.WS_ENDPOINT) throw Error('WS_ENDPOINT ENV must be set');
    const wsProvider = new WsProvider(process.env.WS_ENDPOINT);
    return ApiPromise.create({ provider: wsProvider, noInitWarn: true });
}