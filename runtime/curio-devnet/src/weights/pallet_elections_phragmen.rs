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


//! Autogenerated weights for pallet_elections_phragmen
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/curio-parachain-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-elections-phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/curio-devnet/src/weights/pallet_elections_phragmen.rs
// --template=templates/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_elections_phragmen.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Minimum execution time: 48_866 nanoseconds.
		Weight::from_ref_time(49_993_673)
			// Standard Error: 4_668
			.saturating_add(Weight::from_ref_time(173_040).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Minimum execution time: 65_365 nanoseconds.
		Weight::from_ref_time(66_833_328)
			// Standard Error: 5_314
			.saturating_add(Weight::from_ref_time(98_405).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Minimum execution time: 64_249 nanoseconds.
		Weight::from_ref_time(65_821_266)
			// Standard Error: 6_159
			.saturating_add(Weight::from_ref_time(169_970).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn remove_voter() -> Weight {
		// Minimum execution time: 62_518 nanoseconds.
		Weight::from_ref_time(63_091_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Minimum execution time: 52_858 nanoseconds.
		Weight::from_ref_time(61_334_605)
			// Standard Error: 1_061
			.saturating_add(Weight::from_ref_time(65_396).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Elections Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 49_775 nanoseconds.
		Weight::from_ref_time(62_542_552)
			// Standard Error: 1_072
			.saturating_add(Weight::from_ref_time(44_841).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn renounce_candidacy_members() -> Weight {
		// Minimum execution time: 66_237 nanoseconds.
		Weight::from_ref_time(66_879_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Elections RunnersUp (r:1 w:1)
	fn renounce_candidacy_runners_up() -> Weight {
		// Minimum execution time: 53_580 nanoseconds.
		Weight::from_ref_time(54_451_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn remove_member_without_replacement() -> Weight {
		// Minimum execution time: 500_000_000 nanoseconds.
		Weight::from_ref_time(500_000_000_000)
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn remove_member_with_replacement() -> Weight {
		// Minimum execution time: 78_277 nanoseconds.
		Weight::from_ref_time(79_558_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Elections Voting (r:5001 w:5000)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Balances Locks (r:5000 w:5000)
	// Storage: System Account (r:5000 w:5000)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Minimum execution time: 370_612_229 nanoseconds.
		Weight::from_ref_time(372_250_382_000)
			// Standard Error: 315_759
			.saturating_add(Weight::from_ref_time(46_326_902).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Elections Voting (r:10001 w:0)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Elections ElectionRounds (r:1 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Minimum execution time: 26_277_979 nanoseconds.
		Weight::from_ref_time(26_370_227_000)
			// Standard Error: 217_266
			.saturating_add(Weight::from_ref_time(24_925_161).saturating_mul(v.into()))
			// Standard Error: 13_942
			.saturating_add(Weight::from_ref_time(966_029).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(280))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}