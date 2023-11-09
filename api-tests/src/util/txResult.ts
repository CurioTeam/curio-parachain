import { SubmittableResult } from '@polkadot/api';
import { AnyTuple } from '@polkadot/types/types';

export type CompleteCallback = () => void;
export type ResolveCallback = (value: any) => any;
export type RejectCallback = (reason?: any) => void;
export type OuterPromise = {resolve: ResolveCallback, reject: RejectCallback};

export function ensure_success(result: SubmittableResult, unsub: CompleteCallback) {
    proceed_tx_result(result, unsub);
}

export function ensure_success_promise(result: SubmittableResult, unsub: CompleteCallback, outerPromise: OuterPromise) {
    const onSuccess = () => {outerPromise.resolve(result)};
    const onFailed = (e: Error) => {outerPromise.reject(e)};
    proceed_tx_result(result, unsub, onSuccess, onFailed);
}

function proceed_tx_result (
    result: SubmittableResult, 
    unsub: CompleteCallback, 
    onSuccess: () => any = () => {},
    onFailed: (error: Error) => void = (e: Error) => {throw e}
) {
    const txStatus = get_tx_status(result);

    if(txStatus === TxStatus.Pending) return;
    else if (txStatus === TxStatus.Success) {
        unsub();
        return onSuccess();
    } else {
        unsub();
        const error = parse_error(result);
        return onFailed(error);
    }
}

enum TxStatus {
    Pending,
    Success,
    Failed
}

function get_tx_status(result: SubmittableResult): TxStatus {
    if(is_tx_completed(result)) {
        if(result.isError || extrinsic_failed_event_emitted(result)) return TxStatus.Failed;
        else if(extrinsic_success_event_emitted(result)) return TxStatus.Success;
        else throw Error(`Unexpected transaction result: status = ${result.status.type}`);
    } else {
        return TxStatus.Pending;
    }
}

function is_tx_completed(result: SubmittableResult): boolean {
    if(result.isInBlock || result.isFinalized) return true;
    else return false;
}

function extrinsic_success_event_emitted(result: SubmittableResult) {
    return result.events.filter(e => e.event.method === 'ExtrinsicSuccess').length > 0;
}

function extrinsic_failed_event_emitted(result: SubmittableResult) {
    return result.events.filter(e => e.event.method === 'ExtrinsicFailed').length > 0;
}

export function get_event_data(result: SubmittableResult, eventSignature: string): AnyTuple {
    const parts = eventSignature.trim().split('.');
    if(parts.length != 2) throw Error('Incorrect event signature. A signature is palletName.eventName.');
    const [section, method] = parts;

    const record = result.findRecord(section, method);

    if(record) {
        const {event} = record;
        return event.data;
    } else {
        throw Error('Target event was not found');
    };
}

function parse_error(result: SubmittableResult): Error {
    const dispatchError = result.dispatchError;
    if(dispatchError) {
        if (dispatchError.isModule) {
            const modErr = dispatchError.asModule;
            const errorMeta = dispatchError.registry.findMetaError(modErr);

            return Error(`${errorMeta.section}.${errorMeta.name}`);
        }
    }

    throw Error(`Unexpected transaction error or not error`);
}
