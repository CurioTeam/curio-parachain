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
	dispatch::{
		DispatchError
	}
};
use frame_system::{EnsureSignedBy};
pub use sp_runtime::{
    traits::{
        AccountIdLookup, BlakeTwo256, ConstU32,
    }
};

use pallet_whitelist::{Investor, InvestorKey};
pub use pallet_common::{
	Event as CommonEvent,
	Error as CommonError,
	CollectionHandle
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
	PropertyValue, PropertyKey, TokenId, PropertyPermission,

	MAX_REFUNGIBLE_PIECES
};

pub use pallet_common::collection_initializer::CollectionInitializer;

mod pallet_refungible {
    pub use super::super::*;
}
use pallet_refungible::TokenBalance;

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

impl pallet_refungible::Config for MockRuntime {
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MockRuntime>;
type Block = frame_system::mocking::MockBlock<MockRuntime>;

construct_runtime! {
    pub enum MockRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
		Whitelist: pallet_whitelist,
        Common: pallet_common,
        Balances: pallet_balances,
		Refungible: pallet_refungible,
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

pub fn create_default_token(collection_id: CollectionId, owner: AccountId) -> Result<TokenId, DispatchError> {
	let balances = default_token_balances();
	let properties = default_token_properties();

	Refungible::create_item(RuntimeOrigin::signed(owner), collection_id, balances, properties)?;
	Ok(get_token_id_from_last_event())
}

pub fn get_collection_id_from_last_event() -> CollectionId {
    match System::events().last().unwrap().event {
        RuntimeEvent::Common(CommonEvent::CollectionCreated(collection_id, _, _)) => {
            collection_id
        },
        _ => {
            panic!("Unexpected event");
        }
    }
}

pub fn get_token_id_from_last_event() -> TokenId {
	for event_record in System::events() {
		match event_record.event {
			RuntimeEvent::Common(CommonEvent::ItemCreated(_, token_id, _, _)) => {
				return token_id;
			},
			_ => {}
		}
	}
	
	panic!("ItemCreated event not found");
}

pub struct TokenInitializer {
	balances: Option<Vec<(AccountId, TokenBalance)>>,
	properties: Option<Vec<(Property, bool)>>
}

impl TokenInitializer {
	pub fn new() -> Self {
		Self {
			balances: None,
			properties: None
		}
	}

	pub fn balances(&mut self, balances: Vec<(AccountId, TokenBalance)>) -> &mut Self {
		self.balances = Some(balances);
		self
	}

	pub fn properties(&mut self, properties: Vec<(Property, bool)>) -> &mut Self {
		self.properties = Some(properties);
		self
	}

	pub fn init(&self, collection_id: CollectionId, owner: AccountId) -> Result<TokenId, DispatchError> {
		let mut user_balances: Vec<(AccountId, TokenBalance)> = vec![(owner, 10)];
		let mut properties: Vec<Property> = vec![];

		if let Some(properties_with_mut_flag) = &self.properties {			
			for (property, is_mutable) in properties_with_mut_flag {
				let property_permission = PropertyPermission {
					mutable: *is_mutable,
					collection_admin: false,
					token_owner: false
				};
				let property_key_permission = PropertyKeyPermission {
					key: property.key.clone(),
					permission: property_permission
				};

				let common_collection = <CollectionHandle<MockRuntime>>::try_get(collection_id)?;
				Common::set_property_permission(&common_collection, &owner, property_key_permission)?;
			}

			properties = properties_with_mut_flag.iter().map(|p| p.0.clone()).collect();
		}

		if let Some(balances) = &self.balances {
			user_balances = balances.to_vec();
		}

		// TODO: refactoring
		Refungible::create_item(RuntimeOrigin::signed(owner), collection_id, user_balances, properties)?;
		Ok(get_token_id_from_last_event())
	}
}