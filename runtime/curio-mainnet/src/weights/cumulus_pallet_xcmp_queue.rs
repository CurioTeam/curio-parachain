
//! Autogenerated weights for `cumulus_pallet_xcmp_queue`
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
// --pallet=cumulus_pallet_xcmp_queue
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/curio-mainnet/src/weights/cumulus_pallet_xcmp_queue.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `cumulus_pallet_xcmp_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> cumulus_pallet_xcmp_queue::WeightInfo for WeightInfo<T> {
	/// Storage: XcmpQueue QueueConfig (r:1 w:1)
	/// Proof Skipped: XcmpQueue QueueConfig (max_values: Some(1), max_size: None, mode: Measured)
	fn set_config_with_u32() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `604`
		// Minimum execution time: 4_417 nanoseconds.
		Weight::from_parts(4_733_000, 604)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: XcmpQueue QueueConfig (r:1 w:1)
	/// Proof Skipped: XcmpQueue QueueConfig (max_values: Some(1), max_size: None, mode: Measured)
	fn set_config_with_weight() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `604`
		// Minimum execution time: 4_485 nanoseconds.
		Weight::from_parts(4_860_000, 604)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
