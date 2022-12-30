#!/bin/bash

# Runs all benchmarks for all pallets, for a given runtime, provided by $1
# Should be run on a reference machine to gain accurate benchmarks
# current Substrate reference machine: https://github.com/paritytech/substrate/pull/5848

runtime=${1-"curio-devnet"}
chain=$([ "$1" == "curio-devnet" ] && echo "local-testnet" || echo "dev")
standard_args="--release --features=runtime-benchmarks"

pallets=(
    frame-system
    pallet-balances
    pallet-bounties
    pallet-collective
    pallet-democracy
	pallet-elections-phragmen
    pallet-identity
    pallet-indices
    pallet-membership
    pallet-multisig
    pallet-preimage
	pallet-proxy
    pallet-scheduler
    pallet-session
    pallet-timestamp
	pallet-tips
    pallet-treasury
    pallet-utility
    pallet-vesting
    parachain-staking
    cumulus-pallet-xcmp-queue
    orml-tokens
    pallet-dex
    pallet-currencies
    pallet-whitelist
    pallet-refungible
)

echo "[+] Running all runtime benchmarks for $runtime --chain=$chain"

cargo build $standard_args

for pallet in "${pallets[@]}"; do
    echo "Runtime: $runtime. Pallet: $pallet";
    # shellcheck disable=SC2086
    ./target/release/curio-parachain-node benchmark pallet \
    --chain="${chain}" \
    --steps=50 \
    --repeat=20 \
    --pallet="$pallet" \
    --extrinsic="*" \
    --execution=wasm \
    --wasm-execution=compiled \
    --heap-pages=4096 \
    --output="./runtime/${runtime}/src/weights/${pallet//-/_}.rs" \
    --template="templates/runtime-weight-template.hbs"
done