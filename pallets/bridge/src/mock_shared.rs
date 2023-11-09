// Shared between tests and benchmarks so placed into separate module
// to avoid conflicts caused by the fact that runtime-benchmarks use
// no_std environviment but tests are using std

#![cfg(any(test, feature = "runtime-benchmarks"))]

use crate::EvmAddress;

#[allow(non_snake_case)]
pub fn CGT_ERC20() -> EvmAddress { EvmAddress::repeat_byte(1u8) }
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn DAI_ERC20() -> EvmAddress { EvmAddress::repeat_byte(2u8) }
#[allow(non_snake_case)]
pub fn ETH_ADDRESS_1() -> EvmAddress { EvmAddress::repeat_byte(3u8) }