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

use sp_core::{U256, ConstU32, ConstU128};
use smallvec::smallvec;
use sp_runtime::{
	generic,
	traits::{
		AccountIdLookup, BlakeTwo256
	}
};

use sp_std::prelude::*;

use frame_support::{
	construct_runtime, parameter_types,
	dispatch::{DispatchError, DispatchClass},
	traits::Everything,
	weights::{
		ConstantMultiplier, WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial
	},
	ord_parameter_types
};
use frame_system::{
	limits::{BlockLength, BlockWeights},
	EnsureSignedBy
};
pub use sp_runtime::{Percent, Permill, Perbill};
use polkadot_runtime_common::SlowAdjustingFeeUpdate;
use pallet_whitelist::{Investor, InvestorKey};
pub use pallet_common::{
	Event as CommonEvent, CollectionHandle
};

use crate::sponsoring::CurioSponsorshipHandler;

pub use pallet_balances::{Event as BalancesEvent};

mod curio_devnet_runtime {
    pub use super::super::*;
}
use curio_devnet_runtime::weights::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight};
pub use pallet_refungible::{Balance as RefBalance, Call as RefCall, TokenBalance, CreateItemData};

// Curio
use primitives::{
	Address, Index, Block, BlockNumber, Hash, BlockHashCount,
	Balance, CENTS, MILLICENTS, Signature, time::devnet::DAYS, NORMAL_DISPATCH_RATIO,
	MAXIMUM_BLOCK_WEIGHT, AVERAGE_ON_INITIALIZE_RATIO
};

pub use collection_primitives::{
	Property, PropertyKeyPermission, CollectionId, TokenId, PropertyPermission,
};

pub mod opaque {
	use super::*;
	use sp_runtime::{generic, traits::BlakeTwo256};

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<MockRuntime>,
	frame_system::CheckSpecVersion<MockRuntime>,
	frame_system::CheckTxVersion<MockRuntime>,
	frame_system::CheckGenesis<MockRuntime>,
	frame_system::CheckEra<MockRuntime>,
	frame_system::CheckNonce<MockRuntime>,
	frame_system::CheckWeight<MockRuntime>,
	pallet_charge_transaction::ChargeTransactionPayment<MockRuntime>,
);

pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

// Mock accounts
pub const ROLES_ROOT: AccountId = 777;
pub const ADMIN_1: AccountId = 101;
pub const ADMIN_2: AccountId = 102;
pub const ADMIN_3: AccountId = 103;
pub const POOR_ADMIN: AccountId = 110;

pub use mock_support::primitives::*;
pub use mock_support::accounts::*;
pub use mock_support::consts::*;
pub use mock_support::collections::*;

pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		// in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 CENTS:
		// in our template, we map to 1/10 of that, or 1/10 CENTS
		let p = CENTS / 10;
		let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
		smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

parameter_types! {
	// This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
	//  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
	// `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
	// the lazy contract deletion.
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub const SS58Prefix: u16 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for MockRuntime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// Runtime version.
	type Version = ();
	/// Converts a module to an index of this module in the runtime.
	type PalletInfo = PalletInfo;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = Everything;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = curio_devnet_runtime::weights::frame_system::WeightInfo<MockRuntime>;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The action to take on a Runtime Upgrade
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 0;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for MockRuntime {
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxFreezes = ConstU32<50>;
	type MaxHolds = ConstU32<50>;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<2>;
	type AccountStore = System;
	type WeightInfo = ();//curio_devnet_runtime::weights::pallet_balances::WeightInfo<MockRuntime>;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type HoldIdentifier = ();
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	/// Relay Chain `TransactionByteFee` / 10
	pub const TransactionByteFee: Balance = 10 * MILLICENTS;
	pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for MockRuntime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
	type WeightToFee = WeightToFee;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
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

ord_parameter_types! {
	pub const RolesRoot: AccountId = ROLES_ROOT;
}

impl pallet_whitelist::Config for MockRuntime {
	type RuntimeEvent = RuntimeEvent;
	type RolesRoot = EnsureSignedBy<RolesRoot, AccountId>;
	type WeightInfo = curio_devnet_runtime::weights::pallet_whitelist::WeightInfo<MockRuntime>;
}

impl pallet_refungible::Config for MockRuntime {
	type WeightInfo = curio_devnet_runtime::weights::pallet_refungible::WeightInfo<MockRuntime>;
}

parameter_types! {
	pub const DefaultSponsoringRateLimit: BlockNumber = 1 * DAYS;
	pub const DefaultSponsoringFeeLimit: U256 = U256::MAX;
}

type SponsorshipHandler = CurioSponsorshipHandler<MockRuntime>;

impl pallet_charge_transaction::Config for MockRuntime {
	type SponsorshipHandler = SponsorshipHandler;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime! {
	pub enum MockRuntime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		// Basic stuff.
		System: frame_system,

		// Monetary stuff.
		Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,

		// Unique
		Charging: pallet_charge_transaction::{Pallet, Storage},

		// Curio
		Common: pallet_common,
		Refungible: pallet_refungible,
		Whitelist: pallet_whitelist,
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
	let data = CreateItemData::<AccountId> {
		balances: default_token_balances(),
		properties: default_token_properties()
	};

	Refungible::create_item(RuntimeOrigin::signed(owner), collection_id, data)?;
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
					mutable: *is_mutable
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
		let data = CreateItemData::<AccountId> {
			balances: user_balances,
			properties: properties
		};
		Refungible::create_item(RuntimeOrigin::signed(owner), collection_id, data)?;
		Ok(get_token_id_from_last_event())
	}
}