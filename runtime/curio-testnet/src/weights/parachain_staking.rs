// Curio Parachain

// Copyright (С) 2022 Curio AG (Company Number FL-0002.594.728-9)
// Incorporated and registered in Liechtenstein.

// Copyright (С) 2022 Curio Capital AG (Company Number CHE-211.446.654)
// Incorporated and registered in Zug, Switzerland.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0


//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-24, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/curio-parachain-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=parachain-staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/curio-devnet/src/weights/parachain_staking.rs
// --template=templates/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for parachain_staking.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> parachain_staking::WeightInfo for WeightInfo<T> {
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn on_initialize_no_action() -> Weight {
		// Minimum execution time: 47_758 nanoseconds.
		Weight::from_parts(78_408_000, 0)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn on_initialize_round_update() -> Weight {
		// Minimum execution time: 51_271 nanoseconds.
		Weight::from_parts(52_098_000, 0)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn on_initialize_network_rewards() -> Weight {
		// Minimum execution time: 45_532 nanoseconds.
		Weight::from_parts(77_956_000, 0)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking ForceNewRound (r:0 w:1)
	fn force_new_round() -> Weight {
		// Minimum execution time: 7_648 nanoseconds.
		Weight::from_parts(7_981_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking CandidatePool (r:3 w:0)
	// Storage: ParachainStaking BlocksAuthored (r:2 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:142 w:142)
	// Storage: ParachainStaking Rewards (r:142 w:142)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:0)
	/// The range of component `n` is `[0, 150]`.
	/// The range of component `m` is `[0, 70]`.
	fn set_inflation(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 1_499_975 nanoseconds.
		Weight::from_parts(1_534_508_000, 0)
			// Standard Error: 12_031_024
			.saturating_add(Weight::from_parts(350_541_352, 0).saturating_mul(n.into()))
			// Standard Error: 25_804_362
			.saturating_add(Weight::from_parts(703_258_431, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(292))
			.saturating_add(T::DbWeight::get().reads((52_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().reads((105_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(285))
			.saturating_add(T::DbWeight::get().writes((50_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((105_u64).saturating_mul(m.into())))
	}
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:3 w:0)
	/// The range of component `n` is `[2, 150]`.
	/// The range of component `m` is `[0, 70]`.
	fn set_max_selected_candidates(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 90_542 nanoseconds.
		Weight::from_parts(91_579_000, 0)
			// Standard Error: 722_663
			.saturating_add(Weight::from_parts(18_206_068, 0).saturating_mul(n.into()))
			// Standard Error: 1_552_099
			.saturating_add(Weight::from_parts(31_832_486, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		// Minimum execution time: 32_193 nanoseconds.
		Weight::from_parts(36_346_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking CandidatePool (r:3 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:1)
	// Storage: ParachainStaking BlocksRewarded (r:71 w:71)
	// Storage: ParachainStaking Rewards (r:71 w:71)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:71 w:71)
	// Storage: ParachainStaking DelegatorState (r:70 w:70)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	/// The range of component `n` is `[3, 150]`.
	/// The range of component `m` is `[0, 70]`.
	fn force_remove_candidate(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 156_129 nanoseconds.
		Weight::from_parts(158_224_000, 0)
			// Standard Error: 742_790
			.saturating_add(Weight::from_parts(15_467_160, 0).saturating_mul(n.into()))
			// Standard Error: 1_599_194
			.saturating_add(Weight::from_parts(59_347_166, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(10))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(m.into())))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	/// The range of component `n` is `[1, 149]`.
	/// The range of component `m` is `[0, 70]`.
	fn join_candidates(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 135_623 nanoseconds.
		Weight::from_parts(137_262_000, 0)
			// Standard Error: 677_724
			.saturating_add(Weight::from_parts(14_500_849, 0).saturating_mul(n.into()))
			// Standard Error: 1_444_676
			.saturating_add(Weight::from_parts(30_778_733, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: ParachainStaking CandidatePool (r:3 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	/// The range of component `n` is `[3, 149]`.
	/// The range of component `m` is `[0, 70]`.
	fn init_leave_candidates(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 115_859 nanoseconds.
		Weight::from_parts(116_084_000, 0)
			// Standard Error: 704_646
			.saturating_add(Weight::from_parts(14_658_128, 0).saturating_mul(n.into()))
			// Standard Error: 1_505_714
			.saturating_add(Weight::from_parts(31_676_987, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	/// The range of component `n` is `[3, 149]`.
	/// The range of component `m` is `[0, 70]`.
	fn cancel_leave_candidates(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 110_022 nanoseconds.
		Weight::from_parts(110_801_000, 0)
			// Standard Error: 706_898
			.saturating_add(Weight::from_parts(14_868_282, 0).saturating_mul(n.into()))
			// Standard Error: 1_510_526
			.saturating_add(Weight::from_parts(31_812_199, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:1)
	// Storage: ParachainStaking BlocksRewarded (r:71 w:71)
	// Storage: ParachainStaking Rewards (r:71 w:71)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:71 w:71)
	// Storage: ParachainStaking DelegatorState (r:70 w:70)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	/// The range of component `n` is `[3, 149]`.
	/// The range of component `m` is `[0, 70]`.
	fn execute_leave_candidates(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 137_826 nanoseconds.
		Weight::from_parts(139_139_000, 0)
			// Standard Error: 753_588
			.saturating_add(Weight::from_parts(16_347_445, 0).saturating_mul(n.into()))
			// Standard Error: 1_610_294
			.saturating_add(Weight::from_parts(60_454_871, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(8))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(m.into())))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	/// The range of component `n` is `[1, 149]`.
	/// The range of component `m` is `[0, 70]`.
	/// The range of component `u` is `[0, 9]`.
	fn candidate_stake_more(n: u32, m: u32, _u: u32, ) -> Weight {
		// Minimum execution time: 158_634 nanoseconds.
		Weight::from_parts(159_426_000, 0)
			// Standard Error: 478_109
			.saturating_add(Weight::from_parts(31_249_652, 0).saturating_mul(n.into()))
			// Standard Error: 1_012_783
			.saturating_add(Weight::from_parts(65_651_083, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	/// The range of component `n` is `[1, 149]`.
	/// The range of component `m` is `[0, 70]`.
	fn candidate_stake_less(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 126_565 nanoseconds.
		Weight::from_parts(128_378_000, 0)
			// Standard Error: 704_733
			.saturating_add(Weight::from_parts(15_534_252, 0).saturating_mul(n.into()))
			// Standard Error: 1_502_250
			.saturating_add(Weight::from_parts(33_151_401, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking LastDelegation (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:0 w:1)
	/// The range of component `n` is `[1, 150]`.
	/// The range of component `m` is `[1, 69]`.
	fn join_delegators(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 163_997 nanoseconds.
		Weight::from_parts(165_291_000, 0)
			// Standard Error: 700_812
			.saturating_add(Weight::from_parts(15_759_258, 0).saturating_mul(n.into()))
			// Standard Error: 1_523_875
			.saturating_add(Weight::from_parts(33_136_258, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	/// The range of component `n` is `[1, 150]`.
	/// The range of component `m` is `[1, 69]`.
	/// The range of component `u` is `[1, 9]`.
	fn delegator_stake_more(n: u32, m: u32, _u: u32, ) -> Weight {
		// Minimum execution time: 171_850 nanoseconds.
		Weight::from_parts(172_538_000, 0)
			// Standard Error: 459_785
			.saturating_add(Weight::from_parts(32_373_423, 0).saturating_mul(n.into()))
			// Standard Error: 1_000_289
			.saturating_add(Weight::from_parts(68_972_020, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	/// The range of component `n` is `[1, 150]`.
	/// The range of component `m` is `[1, 69]`.
	fn delegator_stake_less(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 150_203 nanoseconds.
		Weight::from_parts(151_303_000, 0)
			// Standard Error: 716_585
			.saturating_add(Weight::from_parts(16_298_430, 0).saturating_mul(n.into()))
			// Standard Error: 1_558_172
			.saturating_add(Weight::from_parts(33_676_170, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	/// The range of component `n` is `[1, 150]`.
	/// The range of component `m` is `[1, 69]`.
	fn leave_delegators(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 160_251 nanoseconds.
		Weight::from_parts(162_683_000, 0)
			// Standard Error: 721_198
			.saturating_add(Weight::from_parts(16_387_280, 0).saturating_mul(n.into()))
			// Standard Error: 1_568_203
			.saturating_add(Weight::from_parts(34_517_822, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `u` is `[1, 9]`.
	fn unlock_unstaked(u: u32, ) -> Weight {
		// Minimum execution time: 47_019 nanoseconds.
		Weight::from_parts(50_888_651, 0)
			// Standard Error: 22_988
			.saturating_add(Weight::from_parts(602_213, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:0 w:1)
	fn set_max_candidate_stake() -> Weight {
		// Minimum execution time: 24_312 nanoseconds.
		Weight::from_parts(31_906_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	fn increment_delegator_rewards() -> Weight {
		// Minimum execution time: 46_756 nanoseconds.
		Weight::from_parts(47_461_000, 0)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	fn increment_collator_rewards() -> Weight {
		// Minimum execution time: 43_048 nanoseconds.
		Weight::from_parts(64_178_000, 0)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_rewards() -> Weight {
		// Minimum execution time: 58_251 nanoseconds.
		Weight::from_parts(74_883_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking LastRewardReduction (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:3 w:0)
	// Storage: ParachainStaking BlocksAuthored (r:2 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:142 w:142)
	// Storage: ParachainStaking Rewards (r:142 w:142)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:0)
	/// The range of component `n` is `[0, 150]`.
	/// The range of component `m` is `[0, 70]`.
	fn execute_scheduled_reward_change(n: u32, m: u32, ) -> Weight {
		// Minimum execution time: 1_507_261 nanoseconds.
		Weight::from_parts(1_517_973_000, 0)
			// Standard Error: 11_926_415
			.saturating_add(Weight::from_parts(349_538_730, 0).saturating_mul(n.into()))
			// Standard Error: 25_579_994
			.saturating_add(Weight::from_parts(711_352_766, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(293))
			.saturating_add(T::DbWeight::get().reads((52_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().reads((105_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(286))
			.saturating_add(T::DbWeight::get().writes((50_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((105_u64).saturating_mul(m.into())))
	}
}