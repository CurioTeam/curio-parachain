
//! Autogenerated weights for `pallet_collective`
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
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/curio-mainnet/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: Council Members (r:1 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:100 w:100)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3233 ±0) + p * (3227 ±0)`
		//  Estimated: `16198 + m * (7809 ±23) + p * (10254 ±23)`
		// Minimum execution time: 13_060 nanoseconds.
		Weight::from_parts(13_273_000, 16198)
			// Standard Error: 45_572
			.saturating_add(Weight::from_parts(3_639_288, 7809).saturating_mul(m.into()))
			// Standard Error: 45_572
			.saturating_add(Weight::from_parts(5_822_806, 10254).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `135 + m * (32 ±0)`
		//  Estimated: `631 + m * (32 ±0)`
		// Minimum execution time: 12_117 nanoseconds.
		Weight::from_parts(10_873_465, 631)
			// Standard Error: 1_013
			.saturating_add(Weight::from_parts(6_071, 0).saturating_mul(b.into()))
			// Standard Error: 10_443
			.saturating_add(Weight::from_parts(42_694, 32).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:0)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(_b: u32, _m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `135 + m * (32 ±0)`
		//  Estimated: `3242 + m * (64 ±0)`
		// Minimum execution time: 14_840 nanoseconds.
		Weight::from_parts(40_524_840, 3242)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalCount (r:1 w:1)
	/// Proof Skipped: Council ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `457 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `5860 + m * (165 ±0) + p * (180 ±0)`
		// Minimum execution time: 19_285 nanoseconds.
		Weight::from_parts(25_619_919, 5860)
			// Standard Error: 263
			.saturating_add(Weight::from_parts(1_202, 0).saturating_mul(b.into()))
			// Standard Error: 2_749
			.saturating_add(Weight::from_parts(4_130, 165).saturating_mul(m.into()))
			// Standard Error: 2_714
			.saturating_add(Weight::from_parts(105_807, 180).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `911 + m * (64 ±0)`
		//  Estimated: `4790 + m * (128 ±0)`
		// Minimum execution time: 19_016 nanoseconds.
		Weight::from_parts(19_652_680, 4790)
			// Standard Error: 533
			.saturating_add(Weight::from_parts(20_916, 128).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `531 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `5513 + m * (260 ±0) + p * (144 ±0)`
		// Minimum execution time: 21_773 nanoseconds.
		Weight::from_parts(23_059_650, 5513)
			// Standard Error: 1_004
			.saturating_add(Weight::from_parts(17_787, 260).saturating_mul(m.into()))
			// Standard Error: 979
			.saturating_add(Weight::from_parts(143_907, 144).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `867 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `8784 + b * (4 ±0) + m * (264 ±0) + p * (160 ±0)`
		// Minimum execution time: 31_078 nanoseconds.
		Weight::from_parts(33_057_834, 8784)
			// Standard Error: 145
			.saturating_add(Weight::from_parts(1_805, 4).saturating_mul(b.into()))
			// Standard Error: 1_540
			.saturating_add(Weight::from_parts(3_369, 264).saturating_mul(m.into()))
			// Standard Error: 1_501
			.saturating_add(Weight::from_parts(171_808, 160).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:0)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `551 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `6620 + m * (325 ±0) + p * (180 ±0)`
		// Minimum execution time: 23_887 nanoseconds.
		Weight::from_parts(25_119_517, 6620)
			// Standard Error: 818
			.saturating_add(Weight::from_parts(19_265, 325).saturating_mul(m.into()))
			// Standard Error: 798
			.saturating_add(Weight::from_parts(145_538, 180).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:0)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `887 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `10090 + b * (5 ±0) + m * (330 ±0) + p * (200 ±0)`
		// Minimum execution time: 32_974 nanoseconds.
		Weight::from_parts(35_186_900, 10090)
			// Standard Error: 118
			.saturating_add(Weight::from_parts(1_989, 5).saturating_mul(b.into()))
			// Standard Error: 1_249
			.saturating_add(Weight::from_parts(16_504, 330).saturating_mul(m.into()))
			// Standard Error: 1_218
			.saturating_add(Weight::from_parts(167_418, 200).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292 + p * (32 ±0)`
		//  Estimated: `1371 + p * (96 ±0)`
		// Minimum execution time: 12_471 nanoseconds.
		Weight::from_parts(14_996_860, 1371)
			// Standard Error: 2_595
			.saturating_add(Weight::from_parts(110_854, 96).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
