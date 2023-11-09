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

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{
	AccountId, AdaptedBasicCurrency, CouncilAccount, Currencies, Alice, Bob, Eva,
	DustAccount, RuntimeEvent, ExtBuilder, NativeCurrency, RuntimeOrigin, PalletBalances, Runtime, System, Tokens,
	DOT, ID_1, NATIVE_CURRENCY_ID, X_TOKEN_ID,
};
use sp_runtime::{
	traits::{BadOrigin, Bounded},
	ModuleError, TokenError,
};
use orml_tokens as tokens;

#[test]
fn multi_lockable_currency_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(Currencies::set_lock(ID_1, X_TOKEN_ID, &Alice::get(), 50));
			assert_eq!(Tokens::locks(&Alice::get(), X_TOKEN_ID).len(), 1);
			assert_ok!(Currencies::set_lock(ID_1, NATIVE_CURRENCY_ID, &Alice::get(), 50));
			assert_eq!(PalletBalances::locks(&Alice::get()).len(), 1);
		});
}

#[test]
fn multi_reservable_currency_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_eq!(Currencies::total_issuance(NATIVE_CURRENCY_ID), 200);
			assert_eq!(Currencies::total_issuance(X_TOKEN_ID), 200);
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Alice::get()), 100);
			assert_eq!(NativeCurrency::free_balance(&Alice::get()), 100);

			assert_ok!(Currencies::reserve(X_TOKEN_ID, &Alice::get(), 30));
			assert_ok!(Currencies::reserve(NATIVE_CURRENCY_ID, &Alice::get(), 40));
			assert_eq!(Currencies::reserved_balance(X_TOKEN_ID, &Alice::get()), 30);
			assert_eq!(Currencies::reserved_balance(NATIVE_CURRENCY_ID, &Alice::get()), 40);
		});
}

#[test]
fn native_currency_lockable_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(NativeCurrency::set_lock(ID_1, &Alice::get(), 10));
			assert_eq!(PalletBalances::locks(&Alice::get()).len(), 1);
			assert_ok!(NativeCurrency::remove_lock(ID_1, &Alice::get()));
			assert_eq!(PalletBalances::locks(&Alice::get()).len(), 0);
		});
}

#[test]
fn native_currency_reservable_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(NativeCurrency::reserve(&Alice::get(), 50));
			assert_eq!(NativeCurrency::reserved_balance(&Alice::get()), 50);
		});
}

#[test]
fn basic_currency_adapting_pallet_balances_lockable() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(AdaptedBasicCurrency::set_lock(ID_1, &Alice::get(), 10));
			assert_eq!(PalletBalances::locks(&Alice::get()).len(), 1);
			assert_ok!(AdaptedBasicCurrency::remove_lock(ID_1, &Alice::get()));
			assert_eq!(PalletBalances::locks(&Alice::get()).len(), 0);
		});
}

#[test]
fn basic_currency_adapting_pallet_balances_reservable() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(AdaptedBasicCurrency::reserve(&Alice::get(), 50));
			assert_eq!(AdaptedBasicCurrency::reserved_balance(&Alice::get()), 50);
		});
}

#[test]
fn multi_currency_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(Currencies::transfer(Some(Alice::get()).into(), Bob::get(), X_TOKEN_ID, 50));
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Alice::get()), 50);
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Bob::get()), 150);
		});
}

#[test]
fn multi_currency_extended_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(<Currencies as MultiCurrencyExtended<AccountId>>::update_balance(
				X_TOKEN_ID,
				&Alice::get(),
				50
			));
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Alice::get()), 150);
		});
}

#[test]
fn native_currency_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(Currencies::transfer_native_currency(Some(Alice::get()).into(), Bob::get(), 50));
			assert_eq!(NativeCurrency::free_balance(&Alice::get()), 50);
			assert_eq!(NativeCurrency::free_balance(&Bob::get()), 150);

			assert_ok!(NativeCurrency::transfer(&Alice::get(), &Bob::get(), 10));
			assert_eq!(NativeCurrency::free_balance(&Alice::get()), 40);
			assert_eq!(NativeCurrency::free_balance(&Bob::get()), 160);

			assert_eq!(Currencies::slash(NATIVE_CURRENCY_ID, &Alice::get(), 10), 0);
			assert_eq!(NativeCurrency::free_balance(&Alice::get()), 30);
			assert_eq!(NativeCurrency::total_issuance(), 190);
		});
}

#[test]
fn native_currency_extended_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(NativeCurrency::update_balance(&Alice::get(), 10));
			assert_eq!(NativeCurrency::free_balance(&Alice::get()), 110);

			assert_ok!(<Currencies as MultiCurrencyExtended<AccountId>>::update_balance(
				NATIVE_CURRENCY_ID,
				&Alice::get(),
				10
			));
			assert_eq!(NativeCurrency::free_balance(&Alice::get()), 120);
		});
}

#[test]
fn basic_currency_adapting_pallet_balances_transfer() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(AdaptedBasicCurrency::transfer(&Alice::get(), &Bob::get(), 50));
			assert_eq!(PalletBalances::total_balance(&Alice::get()), 50);
			assert_eq!(PalletBalances::total_balance(&Bob::get()), 150);

			// creation fee
			assert_ok!(AdaptedBasicCurrency::transfer(&Alice::get(), &Eva::get(), 10));
			assert_eq!(PalletBalances::total_balance(&Alice::get()), 40);
			assert_eq!(PalletBalances::total_balance(&Eva::get()), 10);
		});
}

#[test]
fn basic_currency_adapting_pallet_balances_deposit() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(AdaptedBasicCurrency::deposit(&Eva::get(), 50));
			assert_eq!(PalletBalances::total_balance(&Eva::get()), 50);
			assert_eq!(PalletBalances::total_issuance(), 250);
		});
}

#[test]
fn basic_currency_adapting_pallet_balances_deposit_throw_error_when_actual_deposit_is_not_expected() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_eq!(PalletBalances::total_balance(&Eva::get()), 0);
			assert_eq!(PalletBalances::total_issuance(), 200);
			assert_noop!(
				AdaptedBasicCurrency::deposit(&Eva::get(), 1),
				Error::<Runtime>::DepositFailed
			);
			assert_eq!(PalletBalances::total_balance(&Eva::get()), 0);
			assert_eq!(PalletBalances::total_issuance(), 200);
			assert_ok!(AdaptedBasicCurrency::deposit(&Eva::get(), 2));
			assert_eq!(PalletBalances::total_balance(&Eva::get()), 2);
			assert_eq!(PalletBalances::total_issuance(), 202);
		});
}

#[test]
fn basic_currency_adapting_pallet_balances_withdraw() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(AdaptedBasicCurrency::withdraw(&Alice::get(), 100));
			assert_eq!(PalletBalances::total_balance(&Alice::get()), 0);
			assert_eq!(PalletBalances::total_issuance(), 100);
		});
}

#[test]
fn basic_currency_adapting_pallet_balances_slash() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_eq!(AdaptedBasicCurrency::slash(&Alice::get(), 101), 1);
			assert_eq!(PalletBalances::total_balance(&Alice::get()), 0);
			assert_eq!(PalletBalances::total_issuance(), 100);
		});
}

#[test]
fn basic_currency_adapting_pallet_balances_update_balance() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(AdaptedBasicCurrency::update_balance(&Alice::get(), -10));
			assert_eq!(PalletBalances::total_balance(&Alice::get()), 90);
			assert_eq!(PalletBalances::total_issuance(), 190);
		});
}

#[test]
fn update_balance_call_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(Currencies::update_balance(
				RuntimeOrigin::root(),
				Alice::get(),
				NATIVE_CURRENCY_ID,
				-10
			));
			assert_eq!(NativeCurrency::free_balance(&Alice::get()), 90);
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Alice::get()), 100);
			assert_ok!(Currencies::update_balance(RuntimeOrigin::root(), Alice::get(), X_TOKEN_ID, 10));
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Alice::get()), 110);
		});
}

#[test]
fn update_balance_call_fails_if_not_root_origin() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Currencies::update_balance(Some(Alice::get()).into(), Alice::get(), X_TOKEN_ID, 100),
			BadOrigin
		);
	});
}

#[test]
fn call_event_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob()
		.build()
		.execute_with(|| {
			assert_ok!(Currencies::transfer(Some(Alice::get()).into(), Bob::get(), X_TOKEN_ID, 50));
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Alice::get()), 50);
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Bob::get()), 150);
			System::assert_has_event(RuntimeEvent::Tokens(tokens::Event::Transfer {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 50,
			}));
			System::assert_has_event(RuntimeEvent::Currencies(crate::Event::Transferred {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 50,
			}));

			System::reset_events();
			assert_ok!(<Currencies as MultiCurrency<AccountId>>::transfer(
				X_TOKEN_ID,
				&Alice::get(),
				&Bob::get(),
				10
			));
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Alice::get()), 40);
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Bob::get()), 160);
			System::assert_has_event(RuntimeEvent::Tokens(tokens::Event::Transfer {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 10,
			}));
			System::assert_has_event(RuntimeEvent::Currencies(crate::Event::Transferred {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 10,
			}));

			assert_ok!(<Currencies as MultiCurrency<AccountId>>::deposit(
				X_TOKEN_ID,
				&Alice::get(),
				100
			));
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Alice::get()), 140);
			System::assert_last_event(RuntimeEvent::Tokens(tokens::Event::Deposited {
				currency_id: X_TOKEN_ID,
				who: Alice::get(),
				amount: 100,
			}));

			assert_ok!(<Currencies as MultiCurrency<AccountId>>::withdraw(
				X_TOKEN_ID,
				&Alice::get(),
				20
			));
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Alice::get()), 120);
			System::assert_last_event(RuntimeEvent::Tokens(tokens::Event::Withdrawn {
				currency_id: X_TOKEN_ID,
				who: Alice::get(),
				amount: 20,
			}));
		});
}

#[test]
fn fungible_inspect_trait_should_work() {
	ExtBuilder::default()
		.balances(vec![
			(Alice::get(), NATIVE_CURRENCY_ID, 200000),
			(Alice::get(), X_TOKEN_ID, 200000),
		])
		.build()
		.execute_with(|| {
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(NATIVE_CURRENCY_ID),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(X_TOKEN_ID),
				200000
			);
			assert_eq!(<NativeCurrency as fungible::Inspect<_>>::total_issuance(), 200000);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 200000);

			// Test for Inspect::minimum_balance
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::minimum_balance(NATIVE_CURRENCY_ID),
				2
			);
			assert_eq!(<Currencies as fungibles::Inspect<_>>::minimum_balance(X_TOKEN_ID), 0);
			assert_eq!(<NativeCurrency as fungible::Inspect<_>>::minimum_balance(), 2);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::minimum_balance(), 2);

			// Test for Inspect::balance and Inspect::total_balance
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_balance(NATIVE_CURRENCY_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(<NativeCurrency as fungible::Inspect<_>>::balance(&Alice::get()), 200000);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				200000
			);

			// Test for Inspect::reducible_balance. No locks or reserves
			// With Keep alive
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					Preservation::Preserve,
					Fortitude::Polite
				),
				199998
			);
			assert_eq!(
				<NativeCurrency as fungible::Inspect<_>>::reducible_balance(
					&Alice::get(),
					Preservation::Preserve,
					Fortitude::Polite
				),
				199998
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::reducible_balance(
					&Alice::get(),
					Preservation::Preserve,
					Fortitude::Polite
				),
				199998
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(
					X_TOKEN_ID,
					&Alice::get(),
					Preservation::Preserve,
					Fortitude::Polite
				),
				200000
			);

			// Test for Inspect::reducible_balance. No locks or reserves
			// without Keep alive.
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					Preservation::Expendable,
					Fortitude::Polite
				),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(
					X_TOKEN_ID,
					&Alice::get(),
					Preservation::Expendable,
					Fortitude::Polite
				),
				200000
			);
			assert_eq!(
				<NativeCurrency as fungible::Inspect<_>>::reducible_balance(
					&Alice::get(),
					Preservation::Expendable,
					Fortitude::Polite
				),
				200000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::reducible_balance(
					&Alice::get(),
					Preservation::Expendable,
					Fortitude::Polite
				),
				200000
			);

			// Set some locks
			assert_ok!(Currencies::set_lock(ID_1, NATIVE_CURRENCY_ID, &Alice::get(), 1000));
			assert_ok!(Currencies::set_lock(ID_1, X_TOKEN_ID, &Alice::get(), 1000));

			// Test Inspect::reducible_balance with locks
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					Preservation::Preserve,
					Fortitude::Polite
				),
				199000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(
					X_TOKEN_ID,
					&Alice::get(),
					Preservation::Preserve,
					Fortitude::Polite
				),
				199000
			);
			assert_eq!(
				<NativeCurrency as fungible::Inspect<_>>::reducible_balance(
					&Alice::get(),
					Preservation::Preserve,
					Fortitude::Polite
				),
				199000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::reducible_balance(
					&Alice::get(),
					Preservation::Preserve,
					Fortitude::Polite
				),
				199000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					Preservation::Expendable,
					Fortitude::Polite
				),
				199000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(
					X_TOKEN_ID,
					&Alice::get(),
					Preservation::Expendable,
					Fortitude::Polite
				),
				199000
			);
			assert_eq!(
				<NativeCurrency as fungible::Inspect<_>>::reducible_balance(
					&Alice::get(),
					Preservation::Expendable,
					Fortitude::Polite
				),
				199000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::reducible_balance(
					&Alice::get(),
					Preservation::Expendable,
					Fortitude::Polite
				),
				199000
			);

			// Test for Inspect::can_deposit
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					Bounded::max_value(),
					Provenance::Minted
				),
				DepositConsequence::Overflow
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_deposit(
					&Alice::get(),
					Bounded::max_value(),
					Provenance::Minted
				),
				DepositConsequence::Overflow
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(NATIVE_CURRENCY_ID, &Bob::get(), 1, Provenance::Minted),
				DepositConsequence::BelowMinimum
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_deposit(&Bob::get(), 1, Provenance::Minted),
				DepositConsequence::BelowMinimum
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					100,
					Provenance::Minted
				),
				DepositConsequence::Success
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_deposit(&Alice::get(), 100, Provenance::Minted),
				DepositConsequence::Success
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(NATIVE_CURRENCY_ID, &Alice::get(), 0, Provenance::Minted),
				DepositConsequence::Success
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_deposit(&Alice::get(), 0, Provenance::Minted),
				DepositConsequence::Success
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(
					X_TOKEN_ID,
					&Alice::get(),
					Bounded::max_value(),
					Provenance::Minted
				),
				DepositConsequence::Overflow
			);
			assert_eq!(
				<Tokens as fungibles::Inspect<_>>::can_deposit(
					X_TOKEN_ID,
					&Alice::get(),
					Bounded::max_value(),
					Provenance::Minted
				),
				DepositConsequence::Overflow
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(X_TOKEN_ID, &Alice::get(), 100, Provenance::Minted),
				DepositConsequence::Success
			);
			assert_eq!(
				<Tokens as fungibles::Inspect<_>>::can_deposit(X_TOKEN_ID, &Alice::get(), 100, Provenance::Minted),
				DepositConsequence::Success
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(X_TOKEN_ID, &Alice::get(), 0, Provenance::Minted),
				DepositConsequence::Success
			);
			assert_eq!(
				<Tokens as fungibles::Inspect<_>>::can_deposit(X_TOKEN_ID, &Alice::get(), 0, Provenance::Minted),
				DepositConsequence::Success
			);

			// Test Inspect::can_withdraw
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_withdraw(NATIVE_CURRENCY_ID, &Alice::get(), Bounded::max_value()),
				WithdrawConsequence::Underflow
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_withdraw(&Alice::get(), Bounded::max_value()),
				WithdrawConsequence::Underflow
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_withdraw(NATIVE_CURRENCY_ID, &Alice::get(), 199000 + 1),
				WithdrawConsequence::Frozen
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_withdraw(&Alice::get(), 199000 + 1),
				WithdrawConsequence::Frozen
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_withdraw(NATIVE_CURRENCY_ID, &Alice::get(), 100),
				WithdrawConsequence::Success
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_withdraw(&Alice::get(), 100),
				WithdrawConsequence::Success
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_withdraw(NATIVE_CURRENCY_ID, &Alice::get(), 0),
				WithdrawConsequence::Success
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_withdraw(&Alice::get(), 0),
				WithdrawConsequence::Success
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_withdraw(X_TOKEN_ID, &Alice::get(), Bounded::max_value()),
				WithdrawConsequence::Underflow
			);
			assert_eq!(
				<Tokens as fungibles::Inspect<_>>::can_withdraw(X_TOKEN_ID, &Alice::get(), Bounded::max_value()),
				WithdrawConsequence::Underflow
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_withdraw(X_TOKEN_ID, &Alice::get(), 200001),
				WithdrawConsequence::Underflow
			);
			assert_eq!(
				<Tokens as fungibles::Inspect<_>>::can_withdraw(X_TOKEN_ID, &Alice::get(), 200001),
				WithdrawConsequence::Underflow
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_withdraw(X_TOKEN_ID, &Alice::get(), 100),
				WithdrawConsequence::Success
			);
			assert_eq!(
				<Tokens as fungibles::Inspect<_>>::can_withdraw(X_TOKEN_ID, &Alice::get(), 100),
				WithdrawConsequence::Success
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_withdraw(X_TOKEN_ID, &Alice::get(), 0),
				WithdrawConsequence::Success
			);
			assert_eq!(
				<Tokens as fungibles::Inspect<_>>::can_withdraw(X_TOKEN_ID, &Alice::get(), 0),
				WithdrawConsequence::Success
			);

			// Test Inspect::asset_exists
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::asset_exists(NATIVE_CURRENCY_ID),
				true
			);
			assert_eq!(<Currencies as fungibles::Inspect<_>>::asset_exists(X_TOKEN_ID), true);
		});
}

#[test]
fn fungible_mutate_trait_should_work() {
	ExtBuilder::default()
		.balances(vec![
			(Alice::get(), NATIVE_CURRENCY_ID, 100000),
			(Alice::get(), X_TOKEN_ID, 200000),
		])
		.build()
		.execute_with(|| {
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(NATIVE_CURRENCY_ID),
				100000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				100000
			);
			assert_ok!(<Currencies as fungibles::Mutate<_>>::mint_into(
				NATIVE_CURRENCY_ID,
				&Alice::get(),
				1000
			));
			System::assert_last_event(RuntimeEvent::Balances(pallet_balances::Event::Minted {
				who: Alice::get(),
				amount: 1000,
			}));
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(NATIVE_CURRENCY_ID),
				101000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				101000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(X_TOKEN_ID),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_ok!(<Currencies as fungibles::Mutate<_>>::mint_into(
				X_TOKEN_ID,
				&Alice::get(),
				1000
			));
			System::assert_last_event(RuntimeEvent::Tokens(tokens::Event::Deposited {
				currency_id: X_TOKEN_ID,
				who: Alice::get(),
				amount: 1000,
			}));
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(X_TOKEN_ID),
				201000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				201000
			);

			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 101000);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				101000
			);
			assert_ok!(<AdaptedBasicCurrency as fungible::Mutate<_>>::mint_into(&Alice::get(), 1000));
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 102000);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				102000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(NATIVE_CURRENCY_ID),
				102000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				102000
			);
			assert_ok!(<Currencies as fungibles::Mutate<_>>::burn_from(
				NATIVE_CURRENCY_ID,
				&Alice::get(),
				1000,
				Precision::Exact,
				Fortitude::Force,
			));
			System::assert_last_event(RuntimeEvent::Balances(pallet_balances::Event::Burned {
				who: Alice::get(),
				amount: 1000,
			}));
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(NATIVE_CURRENCY_ID),
				101000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				101000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(X_TOKEN_ID),
				201000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				201000
			);
			assert_ok!(<Currencies as fungibles::Mutate<_>>::burn_from(
				X_TOKEN_ID,
				&Alice::get(),
				1000,
				Precision::Exact,
				Fortitude::Force,
			));
			System::assert_last_event(RuntimeEvent::Tokens(tokens::Event::Withdrawn {
				currency_id: X_TOKEN_ID,
				who: Alice::get(),
				amount: 1000,
			}));
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(X_TOKEN_ID),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);

			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 101000);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				101000
			);
			assert_ok!(<AdaptedBasicCurrency as fungible::Mutate<_>>::burn_from(
				&Alice::get(),
				1000,
				Precision::Exact,
				Fortitude::Force,
			));
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 100000);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				100000
			);

			// Burn dust if remaining is less than ED.
			assert_eq!(
				<Currencies as fungibles::Mutate<_>>::burn_from(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					99_999,
					Precision::Exact,
					Fortitude::Force,
				),
				Ok(99_999)
			);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 0);
		});
}

#[test]
fn fungible_mutate_trait_transfer_should_work() {
	ExtBuilder::default()
		.balances(vec![
			(Alice::get(), NATIVE_CURRENCY_ID, 500000),
			(Alice::get(), X_TOKEN_ID, 200000),
		])
		.build()
		.execute_with(|| {
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				500000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Bob::get()),
				0
			);

			System::reset_events();
			assert_ok!(<Currencies as fungibles::Mutate<_>>::transfer(
				NATIVE_CURRENCY_ID,
				&Alice::get(),
				&Bob::get(),
				10000,
				Preservation::Preserve,
			));
			System::assert_has_event(RuntimeEvent::Balances(pallet_balances::Event::Transfer {
				from: Alice::get(),
				to: Bob::get(),
				amount: 10000,
			}));
			System::assert_has_event(RuntimeEvent::Currencies(crate::Event::Transferred {
				currency_id: NATIVE_CURRENCY_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 10000,
			}));

			assert_noop!(
				<Currencies as fungibles::Mutate<_>>::transfer(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					&Bob::get(),
					489_999,
					Preservation::Preserve,
				),
				TokenError::NotExpendable,
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				490000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Bob::get()),
				10000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Bob::get()), 0);
			System::reset_events();
			assert_ok!(<Currencies as fungibles::Mutate<_>>::transfer(
				X_TOKEN_ID,
				&Alice::get(),
				&Bob::get(),
				10000,
				Preservation::Preserve,
			));
			System::assert_has_event(RuntimeEvent::Tokens(tokens::Event::Transfer {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 10000,
			}));
			System::assert_has_event(RuntimeEvent::Currencies(crate::Event::Transferred {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 10000,
			}));
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				190000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Bob::get()),
				10000
			);

			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				490000
			);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Bob::get()), 10000);
			assert_ok!(<AdaptedBasicCurrency as fungible::Mutate<_>>::transfer(
				&Alice::get(),
				&Bob::get(),
				10000,
				Preservation::Preserve,
			));
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				480000
			);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Bob::get()), 20000);
		});
}

#[test]
fn fungible_unbalanced_trait_should_work() {
	ExtBuilder::default()
		.balances(vec![
			(Alice::get(), NATIVE_CURRENCY_ID, 100000),
			(Alice::get(), X_TOKEN_ID, 200000),
		])
		.build()
		.execute_with(|| {
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(NATIVE_CURRENCY_ID),
				100000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				100000
			);
			assert_ok!(<Currencies as fungibles::Unbalanced<_>>::write_balance(
				NATIVE_CURRENCY_ID,
				&Alice::get(),
				80000
			));

			// now, fungible::Unbalanced::write_balance as low-level function, does not use BalanceSet event

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(NATIVE_CURRENCY_ID),
				100000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				80000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(X_TOKEN_ID),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_ok!(<Currencies as fungibles::Unbalanced<_>>::write_balance(
				X_TOKEN_ID,
				&Alice::get(),
				80000
			));
			System::assert_last_event(RuntimeEvent::Tokens(tokens::Event::BalanceSet {
				currency_id: X_TOKEN_ID,
				who: Alice::get(),
				free: 80000,
				reserved: 0,
			}));

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(X_TOKEN_ID),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				80000
			);

			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 100000);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()), 80000);
			assert_ok!(<AdaptedBasicCurrency as fungible::Unbalanced<_>>::write_balance(
				&Alice::get(),
				60000
			));
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 100000);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()), 60000);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(NATIVE_CURRENCY_ID),
				100000
			);
			<Currencies as fungibles::Unbalanced<_>>::set_total_issuance(NATIVE_CURRENCY_ID, 60000);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(NATIVE_CURRENCY_ID),
				60000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(X_TOKEN_ID),
				200000
			);
			<Currencies as fungibles::Unbalanced<_>>::set_total_issuance(X_TOKEN_ID, 80000);
			assert_eq!(<Currencies as fungibles::Inspect<_>>::total_issuance(X_TOKEN_ID), 80000);
			System::assert_last_event(RuntimeEvent::Tokens(tokens::Event::TotalIssuanceSet {
				currency_id: X_TOKEN_ID,
				amount: 80000,
			}));

			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 60000);
			<AdaptedBasicCurrency as fungible::Unbalanced<_>>::set_total_issuance(0);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 0);

		});
}

#[test]
fn fungible_inspect_hold_and_hold_trait_should_work() {
	ExtBuilder::default()
		.balances(vec![
			(Alice::get(), NATIVE_CURRENCY_ID, 500000),
			(Alice::get(), X_TOKEN_ID, 200000),
			(Bob::get(), NATIVE_CURRENCY_ID, 10000),
			(Bob::get(), X_TOKEN_ID, 10000),
		])
		.build()
		.execute_with(|| {
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				500000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &(), &Alice::get()),
				0
			);

			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::can_hold(NATIVE_CURRENCY_ID, &(), &Alice::get(), 499998),
				true,
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::can_hold(NATIVE_CURRENCY_ID, &(), &Alice::get(), 500001),
				false
			);

			assert_ok!(<Currencies as fungibles::MutateHold<_>>::hold(
				NATIVE_CURRENCY_ID,
				&(),
				&Alice::get(),
				20000
			));
			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::hold(NATIVE_CURRENCY_ID, &(), &Alice::get(), 500000),
				TokenError::FundsUnavailable,
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				480000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &(), &Alice::get()),
				20000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &(), &Alice::get()),
				0
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::can_hold(X_TOKEN_ID, &(), &Alice::get(), 200000),
				true
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::can_hold(X_TOKEN_ID, &(), &Alice::get(), 200001),
				false
			);
			assert_ok!(<Currencies as fungibles::MutateHold<_>>::hold(
				X_TOKEN_ID,
				&(),
				&Alice::get(),
				20000
			));
			System::assert_last_event(RuntimeEvent::Tokens(tokens::Event::Reserved {
				currency_id: X_TOKEN_ID,
				who: Alice::get(),
				amount: 20000,
			}));

			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::hold(X_TOKEN_ID, &(), &Alice::get(), 200000),
				DispatchError::Module(ModuleError {
					index: 2,
					error: [0, 0, 0, 0],
					message: Some("BalanceTooLow",),
				},)
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &(), &Alice::get()),
				20000
			);

			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				480000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&(), &Alice::get()),
				20000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::can_hold(&(), &Alice::get(), 20000),
				true
			);
			assert_ok!(<AdaptedBasicCurrency as fungible::MutateHold<_>>::hold(
				&(),
				&Alice::get(),
				20000
			));
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				460000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&(), &Alice::get()),
				40000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				460000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &(), &Alice::get()),
				40000
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::release(
					NATIVE_CURRENCY_ID,
					&(),
					&Alice::get(),
					10000,
					Precision::BestEffort,
				),
				Ok(10000)
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				470000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &(), &Alice::get()),
				30000
			);
			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::release(
					NATIVE_CURRENCY_ID,
					&(),
					&Alice::get(),
					50000,
					Precision::Exact,
				),
				TokenError::FundsUnavailable,
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::release(
					NATIVE_CURRENCY_ID,
					&(),
					&Alice::get(),
					50000,
					Precision::BestEffort,
				),
				Ok(30000)
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &(), &Alice::get()),
				0
			);
			assert_ok!(<Currencies as fungibles::MutateHold<_>>::hold(
				NATIVE_CURRENCY_ID,
				&(),
				&Alice::get(),
				30000
			));

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &(), &Alice::get()),
				20000
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::release(
					X_TOKEN_ID,
					&(),
					&Alice::get(),
					10000,
					Precision::BestEffort,
				),
				Ok(10000)
			);
			System::assert_last_event(RuntimeEvent::Tokens(tokens::Event::Unreserved {
				currency_id: X_TOKEN_ID,
				who: Alice::get(),
				amount: 10000,
			}));

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &(), &Alice::get()),
				10000
			);
			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::release(X_TOKEN_ID, &(), &Alice::get(), 100000, Precision::Exact,),
				DispatchError::Module(ModuleError {
					index: 2,
					error: [0, 0, 0, 0],
					message: Some("BalanceTooLow")
				})
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::release(
					X_TOKEN_ID,
					&(),
					&Alice::get(),
					100000,
					Precision::BestEffort,
				),
				Ok(10000)
			);
			assert_ok!(<Currencies as fungibles::MutateHold<_>>::hold(
				X_TOKEN_ID,
				&(),
				&Alice::get(),
				10000
			));

			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				470000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&(), &Alice::get()),
				30000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::MutateHold<_>>::release(&(), &Alice::get(), 10000, Precision::BestEffort,),
				Ok(10000)
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				480000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&(), &Alice::get()),
				20000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				480000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &(), &Alice::get()),
				20000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Bob::get()),
				10000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &(), &Bob::get()),
				0
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::transfer_on_hold(
					NATIVE_CURRENCY_ID,
					&(),
					&Alice::get(),
					&Bob::get(),
					2000,
					Precision::Exact,
					Restriction::OnHold,
					Fortitude::Polite,
				),
				Ok(2000)
			);
			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::transfer_on_hold(
					NATIVE_CURRENCY_ID,
					&(),
					&Alice::get(),
					&Bob::get(),
					200000,
					Precision::Exact,
					Restriction::OnHold,
					Fortitude::Polite,
				),
				TokenError::Frozen,
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				480000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &(), &Alice::get()),
				18000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Bob::get()),
				10000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &(), &Bob::get()),
				2000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &(), &Alice::get()),
				10000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Bob::get()),
				10000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &(), &Bob::get()),
				0
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::transfer_on_hold(
					X_TOKEN_ID,
					&(),
					&Alice::get(),
					&Bob::get(),
					2000,
					Precision::Exact,
					Restriction::OnHold,
					Fortitude::Polite,
				),
				Ok(2000)
			);
			System::assert_last_event(RuntimeEvent::Tokens(tokens::Event::ReserveRepatriated {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 2000,
				status: BalanceStatus::Reserved,
			}));

			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::transfer_on_hold(
					X_TOKEN_ID,
					&(),
					&Alice::get(),
					&Bob::get(),
					200000,
					Precision::Exact,
					Restriction::OnHold,
					Fortitude::Polite,
				),
				DispatchError::Module(ModuleError {
					index: 2,
					error: [0, 0, 0, 0],
					message: Some("BalanceTooLow")
				})
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				198000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &(), &Alice::get()),
				8000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Bob::get()),
				12000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &(), &Bob::get()),
				2000
			);

			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				480000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&(), &Alice::get()),
				18000
			);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Bob::get()), 10000);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&(), &Bob::get()),
				2000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::MutateHold<_>>::transfer_on_hold(
					&(),
					&Alice::get(),
					&Bob::get(),
					2000,
					Precision::Exact,
					Restriction::OnHold,
					Fortitude::Polite,
				),
				Ok(2000)
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				480000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&(), &Alice::get()),
				16000
			);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Bob::get()), 10000);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&(), &Bob::get()),
				4000
			);
		});
}

#[test]
fn sweep_dust_tokens_works() {
	ExtBuilder::default().build().execute_with(|| {
		tokens::Accounts::<Runtime>::insert(
			Bob::get(),
			DOT,
			tokens::AccountData {
				free: 1,
				frozen: 0,
				reserved: 0,
			},
		);
		tokens::Accounts::<Runtime>::insert(
			Eva::get(),
			DOT,
			tokens::AccountData {
				free: 2,
				frozen: 0,
				reserved: 0,
			},
		);
		tokens::Accounts::<Runtime>::insert(
			Alice::get(),
			DOT,
			tokens::AccountData {
				free: 0,
				frozen: 1,
				reserved: 0,
			},
		);
		tokens::Accounts::<Runtime>::insert(
			DustAccount::get(),
			DOT,
			tokens::AccountData {
				free: 100,
				frozen: 0,
				reserved: 0,
			},
		);
		tokens::TotalIssuance::<Runtime>::insert(DOT, 104);

		let accounts = vec![Bob::get(), Eva::get(), Alice::get()];

		assert_noop!(
			Currencies::sweep_dust(RuntimeOrigin::signed(Bob::get()), DOT, accounts.clone()),
			DispatchError::BadOrigin
		);

		assert_ok!(Currencies::sweep_dust(
			RuntimeOrigin::signed(CouncilAccount::get()),
			DOT,
			accounts
		));
		System::assert_last_event(RuntimeEvent::Currencies(crate::Event::DustSwept {
			currency_id: DOT,
			who: Bob::get(),
			amount: 1,
		}));

		// Bob::get()'s account is gone
		assert_eq!(tokens::Accounts::<Runtime>::contains_key(Bob::get(), DOT), false);
		assert_eq!(Currencies::free_balance(DOT, &Bob::get()), 0);

		// Eva::get()'s account remains, not below ED
		assert_eq!(Currencies::free_balance(DOT, &Eva::get()), 2);

		// Dust transferred to dust receiver
		assert_eq!(Currencies::free_balance(DOT, &DustAccount::get()), 101);
		// Total issuance remains the same
		assert_eq!(Currencies::total_issuance(DOT), 104);
	});
}

#[test]
fn sweep_dust_native_currency_works() {
	use frame_support::traits::StoredMap;
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(<Runtime as pallet_balances::Config>::AccountStore::insert(
			&Bob::get(),
			pallet_balances::AccountData {
				free: 1,
				reserved: 0,
				frozen: 0,
				flags: Default::default(),
			},
		));

		// TODO: seems the insert directly does not work now, it's probably because of the new machanism of
		// provider and consumer: https://github.com/paritytech/substrate/blob/569aae5341ea0c1d10426fa1ec13a36c0b64393b/frame/system/src/lib.rs#L1692
		// consider deposit_creating alive account, then decrease the ED to fix this test!
		assert_eq!(
			<Runtime as pallet_balances::Config>::AccountStore::get(&Bob::get()),
			Default::default()
		);
	});
}
