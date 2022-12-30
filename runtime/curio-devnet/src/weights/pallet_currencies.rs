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


//! Autogenerated weights for pallet_currencies
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
// --pallet=pallet-currencies
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/curio-devnet/src/weights/pallet_currencies.rs
// --template=templates/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_currencies.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_currencies::WeightInfo for WeightInfo<T> {
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_non_native_currency() -> Weight {
		// Minimum execution time: 65_555 nanoseconds.
		Weight::from_ref_time(69_046_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_native_currency() -> Weight {
		// Minimum execution time: 55_470 nanoseconds.
		Weight::from_ref_time(57_268_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn update_balance_non_native_currency() -> Weight {
		// Minimum execution time: 49_832 nanoseconds.
		Weight::from_ref_time(51_176_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: System Account (r:1 w:1)
	fn update_balance_native_currency_creating() -> Weight {
		// Minimum execution time: 47_214 nanoseconds.
		Weight::from_ref_time(63_736_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	fn update_balance_native_currency_killing() -> Weight {
		// Minimum execution time: 42_244 nanoseconds.
		Weight::from_ref_time(42_973_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Tokens Accounts (r:1 w:0)
	/// The range of component `c` is `[1, 3]`.
	fn sweep_dust(c: u32, ) -> Weight {
		// Minimum execution time: 18_792 nanoseconds.
		Weight::from_ref_time(14_481_074)
			// Standard Error: 12_390
			.saturating_add(Weight::from_ref_time(5_341_632).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
	}
}