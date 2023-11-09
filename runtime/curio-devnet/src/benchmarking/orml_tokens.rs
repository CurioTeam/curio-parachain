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

// Source https://github.com/AcalaNetwork/Acala
// Subject to the GPL-3.0 license.

// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use super::utils::{lookup_of_account, set_balance as update_balance, token_unit};
use crate::{
	Runtime, 
	Tokens,
	currency_id::{
		CurrencyId,
		TokenSymbol
	}
};
use primitives::{AccountId, Balance};
use sp_std::prelude::*;

use frame_benchmarking::{account, whitelisted_caller};
use frame_system::RawOrigin;

use orml_benchmarking::runtime_benchmarks;
use orml_traits::MultiCurrency;

const SEED: u32 = 0;

runtime_benchmarks! {
	{ Runtime, orml_tokens }

	transfer {
		let amount: Balance = token_unit(CurrencyId::Token(TokenSymbol::DOT));

		let from: AccountId = whitelisted_caller();
		update_balance(CurrencyId::Token(TokenSymbol::DOT), &from, amount);

		let to: AccountId = account("to", 0, SEED);
		let to_lookup = lookup_of_account(to.clone());
	}: _(RawOrigin::Signed(from), to_lookup, CurrencyId::Token(TokenSymbol::DOT), amount)
	verify {
		assert_eq!(<Tokens as MultiCurrency<_>>::total_balance(CurrencyId::Token(TokenSymbol::DOT), &to), amount);
	}

	transfer_all {
		let amount: Balance = token_unit(CurrencyId::Token(TokenSymbol::DOT));

		let from: AccountId = whitelisted_caller();
		update_balance(CurrencyId::Token(TokenSymbol::DOT), &from, amount);

		let to: AccountId = account("to", 0, SEED);
		let to_lookup = lookup_of_account(to);
	}: _(RawOrigin::Signed(from.clone()), to_lookup, CurrencyId::Token(TokenSymbol::DOT), false)
	verify {
		assert_eq!(<Tokens as MultiCurrency<_>>::total_balance(CurrencyId::Token(TokenSymbol::DOT), &from), 0);
	}

	transfer_keep_alive {
		let from: AccountId = whitelisted_caller();
		update_balance(CurrencyId::Token(TokenSymbol::DOT), &from, 2 * token_unit(CurrencyId::Token(TokenSymbol::DOT)));

		let to: AccountId = account("to", 0, SEED);
		let to_lookup = lookup_of_account(to.clone());
	}: _(RawOrigin::Signed(from), to_lookup, CurrencyId::Token(TokenSymbol::DOT), token_unit(CurrencyId::Token(TokenSymbol::DOT)))
	verify {
		assert_eq!(<Tokens as MultiCurrency<_>>::total_balance(CurrencyId::Token(TokenSymbol::DOT), &to), token_unit(CurrencyId::Token(TokenSymbol::DOT)));
	}

	force_transfer {
		let from: AccountId = account("from", 0, SEED);
		let from_lookup = lookup_of_account(from.clone());
		update_balance(CurrencyId::Token(TokenSymbol::DOT), &from, 2 * token_unit(CurrencyId::Token(TokenSymbol::DOT)));

		let to: AccountId = account("to", 0, SEED);
		let to_lookup = lookup_of_account(to.clone());
	}: _(RawOrigin::Root, from_lookup, to_lookup, CurrencyId::Token(TokenSymbol::DOT), token_unit(CurrencyId::Token(TokenSymbol::DOT)))
	verify {
		assert_eq!(<Tokens as MultiCurrency<_>>::total_balance(CurrencyId::Token(TokenSymbol::DOT), &to), token_unit(CurrencyId::Token(TokenSymbol::DOT)));
	}

	set_balance {
		let who: AccountId = account("who", 0, SEED);
		let who_lookup = lookup_of_account(who.clone());

	}: _(RawOrigin::Root, who_lookup, CurrencyId::Token(TokenSymbol::DOT), token_unit(CurrencyId::Token(TokenSymbol::DOT)), token_unit(CurrencyId::Token(TokenSymbol::DOT)))
	verify {
		assert_eq!(<Tokens as MultiCurrency<_>>::total_balance(CurrencyId::Token(TokenSymbol::DOT), &who), 2 * token_unit(CurrencyId::Token(TokenSymbol::DOT)));
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::benchmarking::utils::tests::new_test_ext;
	use orml_benchmarking::impl_benchmark_test_suite;

	impl_benchmark_test_suite!(new_test_ext(),);
}