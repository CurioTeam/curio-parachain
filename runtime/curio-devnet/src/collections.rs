
use frame_support::{
    parameter_types,
    traits::{
        Contains,
    }
};
use frame_system::{
    EnsureRoot,
};
use sp_core::U256;
use sp_runtime::{
    traits::{
        AccountIdConversion,
        ConstU32,
    }
};

use pallet_currencies::BasicCurrencyAdapter;

use crate::{
    AccountId,
    Amount,
    Balance,
    Balances,
    BlockNumber,
    DAYS,
    Runtime,
    RuntimeEvent,
    Tokens,
    Whitelist,
    monetary::{
        TreasuryPalletId,
    },
    currency_id::{
        CGT,
        CurrencyId,
    },
    sponsoring::{
        CurioSponsorshipHandler
    },
    weights,
};


parameter_types! {
    pub const GetNativeCurrencyId: CurrencyId = CGT;
	pub const GetStakingCurrencyId: CurrencyId = CGT;
	pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

impl pallet_currencies::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
	type CurrencyId = CurrencyId;
	type MultiCurrency = Tokens;
	type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = weights::pallet_currencies::WeightInfo<Runtime>;
	type SweepOrigin = EnsureRoot<AccountId>;
    // Send dust to treasury
    // TODO: Do we need pallet_currencies::OnDust if we already have this hook in orml_tokens?
	type OnDust = pallet_currencies::TransferDust<Runtime, TreasuryAccount>;
}

pub struct MutationHooks;
impl orml_traits::currency::MutationHooks<AccountId, CurrencyId, Balance> for MutationHooks {
    // Send dust to treasury
    type OnDust = pallet_currencies::TransferDust<Runtime, TreasuryAccount>;
	type OnSlash = ();
	type PreDeposit = ();
	type PostDeposit = ();
	type PreTransfer = ();
	type PostTransfer = ();
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

// TODO: Set ExistentialDeposits for orml_tokens
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

impl orml_tokens::Config for Runtime {
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
	type WeightInfo = weights::orml_tokens::WeightInfo<Runtime>;
}

impl pallet_refungible::Config for Runtime {
	type WeightInfo = weights::pallet_refungible::WeightInfo<Runtime>;
}

parameter_types! {
	pub const CollectionCreationPrice: Balance = 0;
}

impl pallet_common::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CollectionCreationPrice = CollectionCreationPrice;
	type TreasuryAccountId = TreasuryAccount;
	type Whitelist = Whitelist;
}

// TODO: Overview sponsoring config
parameter_types! {
	pub const DefaultSponsoringRateLimit: BlockNumber = 1 * DAYS;
	pub const DefaultSponsoringFeeLimit: U256 = U256::MAX;
}

type SponsorshipHandler = CurioSponsorshipHandler<Runtime>;

impl pallet_charge_transaction::Config for Runtime {
	type SponsorshipHandler = SponsorshipHandler;
}