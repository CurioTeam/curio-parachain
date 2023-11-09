use crate::{monetary::GetNativeCurrencyId, weights, AccountId, Currencies, Runtime, RuntimeEvent};
use frame_system::EnsureRoot;

impl pallet_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type BridgeRoot = EnsureRoot<AccountId>;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type MultiCurrency = Currencies;
    type WeightInfo = weights::pallet_bridge::WeightInfo<Runtime>;
}
