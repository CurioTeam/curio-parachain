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

use primitives::{CurrencyId, TokenInfo, Balance, AccountId};
use frame_support::assert_ok;
use orml_traits::MultiCurrencyExtended;
use sp_runtime::traits::StaticLookup;
use sp_runtime::SaturatedConversion;
use crate::{Runtime, Currencies, GetNativeCurrencyId, GetStakingCurrencyId, System, RuntimeEvent};

pub const NATIVE: CurrencyId = GetNativeCurrencyId::get();
pub const STAKING: CurrencyId = GetStakingCurrencyId::get();

pub fn assert_last_event(generic_event: RuntimeEvent) {
	System::assert_last_event(generic_event.into());
}

// TODO: completely move to support module
pub fn token_unit(currency_id: CurrencyId) -> Balance {
    if let Some(decimals) = currency_id.decimals() {
        (10 as Balance).saturating_pow(decimals.into())
    } else {
        panic!("{:?} not a token", currency_id);
    }
}

pub fn lookup_of_account(who: AccountId) -> <<Runtime as frame_system::Config>::Lookup as StaticLookup>::Source {
	<Runtime as frame_system::Config>::Lookup::unlookup(who)
}

pub fn set_balance(currency_id: CurrencyId, who: &AccountId, balance: Balance) {
	assert_ok!(<Currencies as MultiCurrencyExtended<_>>::update_balance(
		currency_id,
		who,
		balance.saturated_into()
	));
}

#[cfg(test)]
pub mod tests {
	pub fn new_test_ext() -> sp_io::TestExternalities {
		frame_system::GenesisConfig::default()
			.build_storage::<crate::Runtime>()
			.unwrap()
			.into()
	}
}