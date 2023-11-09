#!/bin/bash

# Runs all benchmarks for all pallets, for a given runtime, provided by $1
# Should be run on a reference machine to gain accurate benchmarks
# current Substrate reference machine: https://github.com/paritytech/substrate/pull/5848

help="Usage: run-benchmarks.sh --chain <chain> --runtime <runtime>\nExample: run-benchmarks.sh --chain dev --runtime curio-devnet-runtime"

# defaults
chain=dev
runtime=curio-devnet-runtime

while [ -n "$1" ]
do
    case $1 in
        --chain) 
            if [ -n "$2" ]; then 
                chain=$2
                shift
            else
                echo -e $help
                exit 1
            fi
        ;;
        --runtime) 
            if [ -n "$2" ]; then 
                runtime=$2
                shift
            else
                echo -e $help
                exit 1
            fi
        ;;
        *)
            echo -e $help
            exit 1
        ;;
    esac
    shift
done

pallets=(
    frame_system
    pallet_balances
    pallet_bounties
    pallet_bridge
    pallet_collective
    pallet_democracy
    pallet_elections_phragmen
    pallet_identity
    pallet_indices
    pallet_membership
    pallet_multisig
    pallet_preimage
    pallet_proxy
    pallet_scheduler
    pallet_session
    pallet_timestamp
    pallet_tips
    pallet_treasury
    pallet_utility
    pallet_vesting
    cumulus_pallet_xcmp_queue
    orml_tokens
    pallet_dex
    pallet_currencies
    pallet_whitelist
    pallet_refungible
)

echo "[+] Running all runtime benchmarks for --chain=$chain --runtime=$runtime"


cargo build --release --features "$runtime runtime-benchmarks"

for pallet in "${pallets[@]}"; do
    echo "Pallet: $pallet"
    echo "./runtime/${runtime/-runtime/}/src/weights/${pallet//-/_}.rs"
    ./target/release/curio-parachain-node benchmark pallet \
    --chain="${chain}" \
    --steps=50 \
    --repeat=20 \
    --pallet="$pallet" \
    --extrinsic="*" \
    --execution=wasm \
    --wasm-execution=compiled \
    --heap-pages=4096 \
    --output="./runtime/${runtime/-runtime/}/src/weights/${pallet//-/_}.rs"
done
