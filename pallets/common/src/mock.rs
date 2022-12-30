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

#![cfg(test)]

use frame_support::{
    construct_runtime, parameter_types, ord_parameter_types,
	traits::{
        Everything
    },
};
use frame_system::{EnsureSignedBy};
pub use sp_runtime::{
    traits::{
        AccountIdLookup, BlakeTwo256, ConstU32,
    }
};

use pallet_whitelist::{
	Investor, InvestorKey
};

use primitives::{
	Index, BlockNumber, Hash, Header, BlockHashCount, Balance
};
pub use mock_support::primitives::*;
pub use mock_support::accounts::*;
pub use mock_support::consts::*;
pub use mock_support::collections::*;

pub use collection_primitives::{
	CreateCollectionData, CollectionMode, CollectionName,
	CollectionDescription, CollectionTokenPrefix, CollectionPropertiesPermissionsVec,
	CollectionPropertiesVec, CollectionFlags, Property, PropertyKeyPermission,
	CollectionId, Collection, CollectionLimits, CollectionPermissions,
	PropertyValue, PropertyKey, SponsorshipState, PropertyPermission
};

pub use pallet_common::collection_initializer::CollectionInitializer;
mod pallet_common {
    pub use super::super::*;
}

// Mock accounts
pub const ROLES_ROOT: AccountId = 777;
pub const ADMIN_1: AccountId = 101;
pub const ADMIN_2: AccountId = 102;
pub const ADMIN_3: AccountId = 103;
pub const POOR_ADMIN: AccountId = 110;

impl frame_system::Config for MockRuntime {
	type AccountId = AccountId;
	type RuntimeCall = RuntimeCall;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 0;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for MockRuntime {
	type MaxLocks = MaxLocks;
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

ord_parameter_types! {
	pub const RolesRootAccountId: AccountId = ROLES_ROOT;
}

impl pallet_whitelist::Config for MockRuntime {
	type RuntimeEvent = RuntimeEvent;
	type RolesRoot = EnsureSignedBy<RolesRootAccountId, AccountId>;
	type WeightInfo = ();
}

parameter_types! {
	pub const CollectionCreationPrice: Balance = 1_000 * DOLLARS;
    pub const TreasuryAccountId: AccountId = 666;
}

impl pallet_common::Config for MockRuntime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type CollectionCreationPrice = CollectionCreationPrice; 
    type TreasuryAccountId = TreasuryAccountId;
	type Whitelist = Whitelist;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MockRuntime>;
type Block = frame_system::mocking::MockBlock<MockRuntime>;

construct_runtime! {
    pub enum MockRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Whitelist: pallet_whitelist::{Pallet, Storage, Config<T>, Event<T>},
        Common: pallet_common::{Pallet, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Storage, Config<T>, Event<T>},
    }
}

pub struct ExtBuilder {
	investors: Vec<(InvestorKey, Investor<AccountId>)>,
	admins: Vec<AccountId>,
	balances: Vec<(AccountId, Balance)>
}

impl ExtBuilder {
	pub fn new() -> Self {
		Self {
			investors: vec![],
			admins: vec![],
			balances: vec![]
		}
	}

	pub fn investors(self, investors: Vec<AccountId>) -> Self {
		let investors = investors
			.iter()
			.enumerate()
			.map(|(i, account)| {
				(
					[i as u8; 32],
					Investor {
						account: account.clone(),
						is_active: true
					}
				)
			})
			.collect();

		Self {
			investors,
			..self
		}
	}

	pub fn wl_admins(self, admins: Vec<AccountId>) -> Self {
		Self {
			admins,
			..self
		}
	}

	pub fn balances(self, balances: Vec<(AccountId, Balance)>) -> Self {
		Self {
			balances,
			..self
		}
	}

    pub fn build(self) -> sp_io::TestExternalities {
		// TODO: WTF
		use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild;

		let mut t = frame_system::GenesisConfig::default()
            .build_storage::<MockRuntime>()
            .unwrap();

		WhitelistConfig {
			investors: self.investors,
			admins: self.admins,
			..Default::default()
		}.assimilate_storage(&mut t).unwrap();

		BalancesConfig {
			balances: self.balances
		}.assimilate_storage(&mut t).unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
    }
}