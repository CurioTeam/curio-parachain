use crate::{
    AccountId,
    Currencies,
    Runtime,
    RuntimeEvent,
    collections::GetNativeCurrencyId,
    weights,
};
use frame_system::EnsureRoot;

impl pallet_bridge::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type BridgeRoot = EnsureRoot<AccountId>;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type MultiCurrency = Currencies;
	type WeightInfo = weights::pallet_bridge::WeightInfo<Runtime>;
}