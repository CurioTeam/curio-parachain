
//! Autogenerated weights for `pallet_bridge`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `alex-ubuntu`, CPU: `12th Gen Intel(R) Core(TM) i7-12700K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/curio-parachain-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/curio-mainnet/src/weights/pallet_bridge.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bridge`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge::WeightInfo for WeightInfo<T> {
	/// Storage: Bridge Managers (r:1 w:0)
	/// Proof: Bridge Managers (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Bridge Paused (r:1 w:0)
	/// Proof: Bridge Paused (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Bridge InRequestStatus (r:1 w:1)
	/// Proof: Bridge InRequestStatus (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Bridge SubBlacklisted (r:1 w:0)
	/// Proof: Bridge SubBlacklisted (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Bridge TokenMap (r:1 w:0)
	/// Proof: Bridge TokenMap (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: Bridge CurrencyMap (r:1 w:0)
	/// Proof: Bridge CurrencyMap (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: Bridge NativeBridgedSupply (r:1 w:1)
	/// Proof: Bridge NativeBridgedSupply (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `16199`
		// Minimum execution time: 39_219 nanoseconds.
		Weight::from_parts(40_098_000, 16199)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Bridge Managers (r:1 w:0)
	/// Proof: Bridge Managers (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Bridge Paused (r:1 w:0)
	/// Proof: Bridge Paused (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Bridge InRequestStatus (r:100 w:100)
	/// Proof: Bridge InRequestStatus (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Bridge SubBlacklisted (r:1 w:0)
	/// Proof: Bridge SubBlacklisted (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Bridge TokenMap (r:1 w:0)
	/// Proof: Bridge TokenMap (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: Bridge CurrencyMap (r:1 w:0)
	/// Proof: Bridge CurrencyMap (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: Bridge NativeBridgedSupply (r:1 w:1)
	/// Proof: Bridge NativeBridgedSupply (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 100]`.
	fn batch_mint(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `13691 + l * (2508 ±0)`
		// Minimum execution time: 39_406 nanoseconds.
		Weight::from_parts(41_115_176, 13691)
			// Standard Error: 22_687
			.saturating_add(Weight::from_parts(17_095_827, 2508).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(l.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(l.into())))
	}
	/// Storage: Bridge Paused (r:1 w:0)
	/// Proof: Bridge Paused (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Bridge CurrencyMap (r:1 w:0)
	/// Proof: Bridge CurrencyMap (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: Bridge SubBlacklisted (r:1 w:0)
	/// Proof: Bridge SubBlacklisted (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Bridge EthBlacklisted (r:1 w:0)
	/// Proof: Bridge EthBlacklisted (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	/// Storage: Bridge NativeBridgedSupply (r:1 w:1)
	/// Proof: Bridge NativeBridgedSupply (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Bridge OutRequestId (r:1 w:1)
	/// Proof: Bridge OutRequestId (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `521`
		//  Estimated: `11674`
		// Minimum execution time: 33_703 nanoseconds.
		Weight::from_parts(34_730_000, 11674)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Bridge Managers (r:1 w:0)
	/// Proof: Bridge Managers (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Bridge Paused (r:1 w:1)
	/// Proof: Bridge Paused (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	fn set_full_pause() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208`
		//  Estimated: `3020`
		// Minimum execution time: 11_536 nanoseconds.
		Weight::from_parts(11_973_000, 3020)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Bridge Managers (r:1 w:0)
	/// Proof: Bridge Managers (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Bridge CurrencyMap (r:1 w:1)
	/// Proof: Bridge CurrencyMap (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	fn set_currency_pause() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `234`
		//  Estimated: `5041`
		// Minimum execution time: 13_234 nanoseconds.
		Weight::from_parts(13_664_000, 5041)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Bridge Managers (r:1 w:0)
	/// Proof: Bridge Managers (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Bridge EthBlacklisted (r:1 w:1)
	/// Proof: Bridge EthBlacklisted (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	fn set_eth_blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208`
		//  Estimated: `5036`
		// Minimum execution time: 12_086 nanoseconds.
		Weight::from_parts(12_622_000, 5036)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Bridge Managers (r:1 w:0)
	/// Proof: Bridge Managers (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Bridge SubBlacklisted (r:1 w:1)
	/// Proof: Bridge SubBlacklisted (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn set_sub_blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208`
		//  Estimated: `5048`
		// Minimum execution time: 12_107 nanoseconds.
		Weight::from_parts(14_856_000, 5048)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Bridge Managers (r:1 w:1)
	/// Proof: Bridge Managers (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn set_manager() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `160`
		//  Estimated: `2524`
		// Minimum execution time: 9_518 nanoseconds.
		Weight::from_parts(10_086_000, 2524)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}