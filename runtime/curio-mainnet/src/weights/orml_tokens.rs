
//! Autogenerated weights for `orml_tokens`
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
// --pallet=orml_tokens
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/curio-devnet/src/weights/orml_tokens.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for `orml_tokens`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_tokens::WeightInfo for WeightInfo<T> {
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(109), added: 2584, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1733`
		//  Estimated: `7771`
		// Minimum execution time: 34_827 nanoseconds.
		Weight::from_parts(36_114_000, 7771)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(109), added: 2584, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1733`
		//  Estimated: `7771`
		// Minimum execution time: 39_222 nanoseconds.
		Weight::from_parts(53_017_000, 7771)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(109), added: 2584, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1561`
		//  Estimated: `7771`
		// Minimum execution time: 31_722 nanoseconds.
		Weight::from_parts(32_774_000, 7771)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(109), added: 2584, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1733`
		//  Estimated: `10374`
		// Minimum execution time: 35_708 nanoseconds.
		Weight::from_parts(37_100_000, 10374)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(109), added: 2584, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn set_balance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1363`
		//  Estimated: `7691`
		// Minimum execution time: 25_974 nanoseconds.
		Weight::from_parts(34_000_000, 0)
		.saturating_add(RocksDbWeight::get().reads(3 as u64))
		.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
}