
// This file is part of Curio.

// Copyright (C) 2020-2022 Curio Capital AG.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

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

//! Autogenerated weights for pallet_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/curio-parachain-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_vesting
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/src/weights/vesting.rs
// --execution
// wasm
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --template
// templates/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_vesting.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		(80_553_000 as Weight)
			// Standard Error: 66_000
			.saturating_add((86_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 136_000
			.saturating_add((952_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		(37_336_000 as Weight)
			// Standard Error: 81_000
			.saturating_add((1_260_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 167_000
			.saturating_add((1_077_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		(138_281_000 as Weight)
			// Standard Error: 103_000
			.saturating_add((513_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 211_000
			.saturating_add((712_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		(135_368_000 as Weight)
			// Standard Error: 99_000
			.saturating_add((354_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 202_000
			.saturating_add((812_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		(173_804_000 as Weight)
			// Standard Error: 171_000
			.saturating_add((516_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 348_000
			.saturating_add((2_067_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	fn force_vested_transfer(l: u32, _s: u32, ) -> Weight {
		(225_929_000 as Weight)
			// Standard Error: 152_000
			.saturating_add((744_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(136_381_000 as Weight)
			// Standard Error: 104_000
			.saturating_add((317_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 222_000
			.saturating_add((967_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(162_291_000 as Weight)
			// Standard Error: 106_000
			.saturating_add((163_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 226_000
			.saturating_add((42_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
