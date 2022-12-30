# defaults
PARACHAIN_BIN_DEFAULT="./target/release/curio-parachain-node"
CHAIN_SPEC_DEFAULT="dev"
OUT_DIR_DEFAULT="./"

generate_validation_wasm() {
    eval "$PARACHAIN_BIN export-genesis-wasm --chain $CHAIN_SPEC $OUT_DIR/parachain-wasm"
}

generate_genesis() {
    eval "$PARACHAIN_BIN export-genesis-state --chain $CHAIN_SPEC $OUT_DIR/parachain-genesis"
}

set_if_not_exists() {
    if [[ -z "$(printenv $1)" ]]; then
        echo "$1 env is not set. Using default"
        eval "export $1=\"$2\""
    fi

    echo "$1 = $(printenv $1)"
}

out_dir_exists() {
    if [ ! -d $OUT_DIR ] 
    then
        echo "$OUT_DIR not found!"
        exit
    fi
}

trim_dir_path() {
    if [ "${OUT_DIR: -1}" = "/" ] 
    then
        let length=(${#OUT_DIR} - 1)
        OUT_DIR="${OUT_DIR:0:$length}"
    fi
}

set_if_not_exists "PARACHAIN_BIN" "$PARACHAIN_BIN_DEFAULT"
set_if_not_exists "CHAIN_SPEC" "$CHAIN_SPEC_DEFAULT"
set_if_not_exists "OUT_DIR" "$OUT_DIR_DEFAULT"
out_dir_exists
trim_dir_path
generate_validation_wasm
generate_genesis

echo "Done"
