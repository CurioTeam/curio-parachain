#![cfg(test)]

pub use frame_support::{
    construct_runtime,
    parameter_types,
    traits::{
        Everything, 
        Contains
    }
};
pub use frame_system::EnsureRoot;
pub use sp_runtime::{
    RuntimeDebug,
    traits::{
        BlakeTwo256, AccountIdLookup, ConstU32
    }
};
pub use codec::{
    Encode, 
    Decode, 
    MaxEncodedLen
};
pub use scale_info::TypeInfo;
pub use sp_std::{
    str::FromStr
};
pub use primitives::{
    Index, Header, Hash, BlockHashCount, BlockNumber, Amount
};
pub use pallet_currencies::BasicCurrencyAdapter;
pub use mock_support::{
    primitives::{
        AccountId, 
        Balance
    },
    accounts::*
};
use module_currency_id::create_currency_id;
pub use crate::mock_shared::{
    CGT_ERC20, 
    DAI_ERC20, 
    ETH_ADDRESS_1
};
mod pallet_bridge {
    pub use crate::*;
}
use self::pallet_bridge::{
    EvmAddress,
    NativeBridgedSupply,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};


impl frame_system::Config for MockRuntime {
    type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	
    type BaseCallFilter = Everything;

	type AccountId = AccountId;
	type AccountData = pallet_balances::AccountData<Balance>;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type MaxConsumers = ConstU32<16>;
	type SS58Prefix = ();

	type Index = Index;
	type Header = Header;
	type BlockNumber = BlockNumber;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockWeights = ();
	
    type Hash = Hash;
	type Hashing = BlakeTwo256;
	
	type OnSetCode = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
    
	type SystemWeightInfo = ();
	type DbWeight = ();
    
    type PalletInfo = PalletInfo;
	type Version = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1000;
}

#[derive(
	Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, MaxEncodedLen, Debug, TypeInfo,
)]
pub enum HoldReason {
	/// The NIS Pallet has reserved it for a non-fungible receipt.
	Nis,
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
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type HoldIdentifier = HoldReason;
	type ReserveIdentifier = [u8; 8];
}


create_currency_id! {
    #[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[repr(u8)]
	pub enum TokenSymbol {
		// Native token
		CGT("Curio Governance Token", 18) = 0,

		// Bridged
		DAI("Dai Stablecoin", 18) = 20,
	}
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum CurrencyId {
	Token(TokenSymbol),
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = CGT;
	pub const GetStakingCurrencyId: CurrencyId = CGT;
	pub TreasuryAccount: AccountId = ALICE;
}

impl pallet_currencies::Config for MockRuntime {
	type RuntimeEvent = RuntimeEvent;
	
    type CurrencyId = CurrencyId;
    type MultiCurrency = Tokens;
	type NativeCurrency = BasicCurrencyAdapter<MockRuntime, Balances, Amount, BlockNumber>;
	type GetNativeCurrencyId = GetNativeCurrencyId;

	type SweepOrigin = EnsureRoot<AccountId>;
	type OnDust = pallet_currencies::TransferDust<MockRuntime, TreasuryAccount>;

	type WeightInfo = ();
}

pub struct ExistentialDeposits;
impl orml_traits::GetByKey<CurrencyId, Balance> for ExistentialDeposits {
	fn get(_k: &CurrencyId) -> Balance {
		0 as Balance
	}
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(_a: &AccountId) -> bool {
		false
	}
}

pub struct MutationHooks;
impl orml_traits::currency::MutationHooks<AccountId, CurrencyId, Balance> for MutationHooks {
	type OnDust = orml_tokens::BurnDust<MockRuntime>;
	type OnSlash = ();
	type PreDeposit = ();
	type PostDeposit = ();
	type PreTransfer = ();
	type PostTransfer = ();
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

impl orml_tokens::Config for MockRuntime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type ExistentialDeposits = ExistentialDeposits;
	type CurrencyHooks = MutationHooks;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type WeightInfo = ();
}

impl pallet_bridge::Config for MockRuntime {
    type RuntimeEvent = RuntimeEvent;
    type BridgeRoot = EnsureRoot<Self::AccountId>;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type MultiCurrency = Currencies;
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
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
        Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
        Currencies: pallet_currencies::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Bridge: pallet_bridge::{Pallet, Call, Storage, Config<T>, Event<T>},
    }
}

pub const DAI_DECIMALS: Balance = 1_000_000_000_000_000_000;
pub const CGT_DECIMALS: Balance = 1_000_000_000_000_000_000;

pub struct ExtBuilder {
    pub tokens_balances: Vec<(AccountId, CurrencyId, Balance)>,
    pub native_balances: Vec<(AccountId, Balance)>,
    pub blacklisted_eth: Vec<EvmAddress>,
    pub blacklisted_sub: Vec<AccountId>,
    pub managers: Vec<AccountId>,
    pub currencies_map: Vec<(CurrencyId, EvmAddress, bool)>,
    pub currencies_pause: Vec<CurrencyId>,
    pub full_pause: bool,
    pub native_bridged_supply: Balance
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			tokens_balances: vec![],
            native_balances: vec![],
            blacklisted_eth: vec![],
            blacklisted_sub: vec![],
            managers: vec![ALICE],
            currencies_map: vec![
                (CGT, CGT_ERC20(), false),
                (DAI, DAI_ERC20(), false)
            ],
            currencies_pause: vec![],
            full_pause: false,
            native_bridged_supply: 0
		}
	}
}

impl ExtBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn balances(mut self, balances: Vec<(AccountId, CurrencyId, Balance)>) -> Self {
        for (account, currency_id, balance) in balances.iter() {
            if *currency_id == GetNativeCurrencyId::get() {
                self.native_balances.push((*account, *balance));
            } else {
                self.tokens_balances.push((*account, *currency_id, *balance));
            }
        }

        self
    }

    pub fn full_pause(mut self) -> Self {
        self.full_pause = true;
        self
    }

    pub fn managers(mut self, managers: Vec<AccountId>) -> Self {
        self.managers = managers;
        self
    }

    pub fn blacklist_sub(mut self, blacklisted: Vec<AccountId>) -> Self {
        self.blacklisted_sub = blacklisted;
        self
    }
    pub fn blacklist_eth(mut self, blacklisted: Vec<EvmAddress>) -> Self {
        self.blacklisted_eth = blacklisted;
        self
    }

    pub fn currencies(mut self, currencies: Vec<(CurrencyId, EvmAddress, bool)>) -> Self {
        self.currencies_map = currencies;
        self
    }

    pub fn currencies_pause(mut self, currencies: Vec<CurrencyId>) -> Self {
        for (currency_id, _, paused) in self.currencies_map.iter_mut() {
            if currencies.contains(currency_id) {
                *paused = true;
            }
        }

        self
    }

    pub fn native_bridged_supply(mut self, supply: Balance) -> Self {
        self.native_bridged_supply = supply;
        self
    }
    
	pub fn build(self) -> sp_io::TestExternalities {
        use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild;
        
		let mut t = frame_system::GenesisConfig::default()
        .build_storage::<MockRuntime>()
        .unwrap();
        
        pallet_balances::GenesisConfig::<MockRuntime> {
            balances: self.native_balances
        }
        .assimilate_storage(&mut t)
        .unwrap();
        
        orml_tokens::GenesisConfig::<MockRuntime> {
            balances: self.tokens_balances
        }
        .assimilate_storage(&mut t)
        .unwrap();
        
        
        pallet_bridge::GenesisConfig::<MockRuntime> {
            supported_currencies: self.currencies_map,
            initial_managers: self.managers,
            blacklisted_sub: self.blacklisted_sub,
            blacklisted_eth: self.blacklisted_eth,
            full_pause: self.full_pause,
        }
        .assimilate_storage(&mut t)
        .unwrap();
        
        
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| {
            System::set_block_number(1);
            NativeBridgedSupply::<MockRuntime>::set(self.native_bridged_supply);
        });
        ext
	}
}