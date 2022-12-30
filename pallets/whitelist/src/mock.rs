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

use crate::{self as pallet_whitelist, InvestorKey, Investor, MAX_NEW_INVESTORS};
use frame_support::{parameter_types, ord_parameter_types, traits::Everything};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use system::{EnsureSignedBy};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u64;

fn too_many_investors() -> Vec<(InvestorKey, Investor<AccountId>)> {
	let mut investors = Vec::<(InvestorKey, Investor<AccountId>)>::with_capacity(MAX_NEW_INVESTORS.into());

	for i in 0..=MAX_NEW_INVESTORS {
		investors.push(
			(
				[i; 32],
				Investor {
					account: i as AccountId,
					is_active: false
				}
			)
		)
	}

	investors
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Whitelist: pallet_whitelist::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

ord_parameter_types! {
	pub const RolesRoot: AccountId = 1;
}

pub const ROLES_ROOT: AccountId = 1;
pub const ALICE: AccountId = 2;
pub const BOB: AccountId = 3;
pub const EVE: AccountId = 4;
pub const CHARLIE: AccountId = 5;
pub const DAVE: AccountId = 6;

parameter_types! {
	pub SampleInvestors2: Vec<(InvestorKey, Investor<AccountId>)> = vec![
		(
			[0u8; 32],
			Investor {
				account: BOB,
				is_active: true
			}
		),
		(
			[1u8; 32],
			Investor {
				account: EVE,
				is_active: false
			}
		)
	];

	pub SampleInvestors4: Vec<(InvestorKey, Investor<AccountId>)> = vec![
		(
			[0u8; 32],
			Investor {
				account: BOB,
				is_active: true
			}
		),
		(
			[1u8; 32],
			Investor {
				account: EVE,
				is_active: false
			}
		),
		(
			[2u8; 32],
			Investor {
				account: CHARLIE,
				is_active: true
			}
		),
		(
			[3u8; 32],
			Investor {
				account: DAVE,
				is_active: false
			}
		)
	];

	pub SampleInvestorsDoubleAccount: Vec<(InvestorKey, Investor<AccountId>)> = vec![
		(
			[0u8; 32],
			Investor {
				account: BOB,
				is_active: true
			}
		),
		(
			[1u8; 32],
			Investor {
				account: EVE,
				is_active: false
			}
		),
		(
			[2u8; 32],
			Investor {
				account: BOB,
				is_active: false
			}
		)
	];

	pub SampleInvestorsDoubleKey: Vec<(InvestorKey, Investor<AccountId>)> = vec![
		(
			[0u8; 32],
			Investor {
				account: BOB,
				is_active: true
			}
		),
		(
			[1u8; 32],
			Investor {
				account: EVE,
				is_active: false
			}
		),
		(
			[1u8; 32],
			Investor {
				account: CHARLIE,
				is_active: false
			}
		)
	];

	pub SampleInvestorsTooMany: Vec<(InvestorKey, Investor<AccountId>)> = too_many_investors();
}

impl pallet_whitelist::Config for Test {
    type RolesRoot = EnsureSignedBy<RolesRoot, Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
