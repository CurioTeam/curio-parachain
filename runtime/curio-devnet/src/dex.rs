use frame_support::{
    parameter_types, PalletId,
};
use frame_system::{
    EnsureRoot,
};

use crate::{
    AccountId,
    BlockNumber,
    Currencies,
    DAYS,
    Runtime,
    RuntimeEvent,
    currency_id::{
        CurrencyId,
    },
    weights,
};

parameter_types! {
	pub const DEXPalletId: PalletId = PalletId(*b"cur/dexm");
	pub const GetExchangeFee: (u32, u32) = (3, 1000);	// 0.3%
	pub const ExtendedProvisioningBlocks: BlockNumber = 2 * DAYS;
	pub const TradingPathLimit: u32 = 3;
}

impl pallet_dex::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CurrencyId = CurrencyId;
	type Currency = Currencies;
	type GetExchangeFee = GetExchangeFee;
	type TradingPathLimit = TradingPathLimit;
	type PalletId = DEXPalletId;
	type DEXIncentives = ();
	type WeightInfo = weights::pallet_dex::WeightInfo<Runtime>;
	type ListingOrigin = EnsureRoot<AccountId>;
	type ExtendedProvisioningBlocks = ExtendedProvisioningBlocks;
	type OnLiquidityPoolUpdated = ();
}