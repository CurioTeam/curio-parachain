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
	DustAccount, Event, ExtBuilder, NativeCurrency, Origin, PalletBalances, Runtime, System, Tokens,
	DOT, ID_1, NATIVE_CURRENCY_ID, X_TOKEN_ID,
};
use sp_runtime::{
	traits::{BadOrigin, Bounded},
	ModuleError,
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
				Origin::root(),
				Alice::get(),
				NATIVE_CURRENCY_ID,
				-10
			));
			assert_eq!(NativeCurrency::free_balance(&Alice::get()), 90);
			assert_eq!(Currencies::free_balance(X_TOKEN_ID, &Alice::get()), 100);
			assert_ok!(Currencies::update_balance(Origin::root(), Alice::get(), X_TOKEN_ID, 10));
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
			System::assert_has_event(Event::Tokens(tokens::Event::Transfer {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 50,
			}));
			System::assert_has_event(Event::Currencies(crate::Event::Transferred {
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
			System::assert_has_event(Event::Tokens(tokens::Event::Transfer {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 10,
			}));
			System::assert_has_event(Event::Currencies(crate::Event::Transferred {
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
			System::assert_last_event(Event::Tokens(tokens::Event::Deposited {
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
			System::assert_last_event(Event::Tokens(tokens::Event::Withdrawn {
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
			(Alice::get(), NATIVE_CURRENCY_ID, 100000),
			(Alice::get(), X_TOKEN_ID, 200000),
		])
		.build()
		.execute_with(|| {
			// Test for Inspect::total_issuance
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(NATIVE_CURRENCY_ID),
				100000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::total_issuance(X_TOKEN_ID),
				200000
			);
			assert_eq!(<NativeCurrency as fungible::Inspect<_>>::total_issuance(), 100000);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 100000);

			// Test for Inspect::minimum_balance
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::minimum_balance(NATIVE_CURRENCY_ID),
				2
			);
			assert_eq!(<Currencies as fungibles::Inspect<_>>::minimum_balance(X_TOKEN_ID), 0);
			assert_eq!(<NativeCurrency as fungible::Inspect<_>>::minimum_balance(), 2);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::minimum_balance(), 2);

			// Test for Inspect::balance
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				100000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(<NativeCurrency as fungible::Inspect<_>>::balance(&Alice::get()), 100000);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()), 100000);

			// Test for Inspect::reducible_balance. No locks or reserves
			// With Keep alive
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(NATIVE_CURRENCY_ID, &Alice::get(), true),
				99998
			);
			assert_eq!(
				<NativeCurrency as fungible::Inspect<_>>::reducible_balance(&Alice::get(), true),
				99998
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::reducible_balance(&Alice::get(), true),
				99998
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(X_TOKEN_ID, &Alice::get(), true),
				200000
			);

			// Test for Inspect::reducible_balance. No locks or reserves
			// without Keep alive.
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(NATIVE_CURRENCY_ID, &Alice::get(), false),
				100000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(X_TOKEN_ID, &Alice::get(), false),
				200000
			);
			assert_eq!(
				<NativeCurrency as fungible::Inspect<_>>::reducible_balance(&Alice::get(), false),
				100000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::reducible_balance(&Alice::get(), false),
				100000
			);

			// Set some locks
			assert_ok!(Currencies::set_lock(ID_1, NATIVE_CURRENCY_ID, &Alice::get(), 1000));
			assert_ok!(Currencies::set_lock(ID_1, X_TOKEN_ID, &Alice::get(), 1000));

			// Test Inspect::reducible_balance with locks
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(NATIVE_CURRENCY_ID, &Alice::get(), true),
				99000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(X_TOKEN_ID, &Alice::get(), true),
				199000
			);
			assert_eq!(
				<NativeCurrency as fungible::Inspect<_>>::reducible_balance(&Alice::get(), true),
				99000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::reducible_balance(&Alice::get(), true),
				99000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(NATIVE_CURRENCY_ID, &Alice::get(), false),
				99000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::reducible_balance(X_TOKEN_ID, &Alice::get(), false),
				199000
			);
			assert_eq!(
				<NativeCurrency as fungible::Inspect<_>>::reducible_balance(&Alice::get(), false),
				99000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::reducible_balance(&Alice::get(), false),
				99000
			);

			// Test for Inspect::can_deposit
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					Bounded::max_value(),
					false
				),
				DepositConsequence::Overflow
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_deposit(&Alice::get(), Bounded::max_value(), false),
				DepositConsequence::Overflow
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(NATIVE_CURRENCY_ID, &Bob::get(), 1, false),
				DepositConsequence::BelowMinimum
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_deposit(&Bob::get(), 1, false),
				DepositConsequence::BelowMinimum
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(NATIVE_CURRENCY_ID, &Alice::get(), 100, false),
				DepositConsequence::Success
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_deposit(&Alice::get(), 100, false),
				DepositConsequence::Success
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(NATIVE_CURRENCY_ID, &Alice::get(), 0, false),
				DepositConsequence::Success
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_deposit(&Alice::get(), 0, false),
				DepositConsequence::Success
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(X_TOKEN_ID, &Alice::get(), Bounded::max_value(), false),
				DepositConsequence::Overflow
			);
			assert_eq!(
				<Tokens as fungibles::Inspect<_>>::can_deposit(X_TOKEN_ID, &Alice::get(), Bounded::max_value(), false),
				DepositConsequence::Overflow
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(X_TOKEN_ID, &Alice::get(), 100, false),
				DepositConsequence::Success
			);
			assert_eq!(
				<Tokens as fungibles::Inspect<_>>::can_deposit(X_TOKEN_ID, &Alice::get(), 100, false),
				DepositConsequence::Success
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::can_deposit(X_TOKEN_ID, &Alice::get(), 0, false),
				DepositConsequence::Success
			);
			assert_eq!(
				<Tokens as fungibles::Inspect<_>>::can_deposit(X_TOKEN_ID, &Alice::get(), 0, false),
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
				<Currencies as fungibles::Inspect<_>>::can_withdraw(NATIVE_CURRENCY_ID, &Alice::get(), 99001),
				WithdrawConsequence::Frozen
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::can_withdraw(&Alice::get(), 99001),
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
			System::assert_last_event(Event::Balances(pallet_balances::Event::Deposit {
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
			System::assert_last_event(Event::Tokens(tokens::Event::Deposited {
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

			// mint_into will deposit erc20 holding account to recipient.
			// but here erc20 holding account don't have enough balance.

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
				1000
			));
			System::assert_last_event(Event::Balances(pallet_balances::Event::Withdraw {
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
				1000
			));
			System::assert_last_event(Event::Tokens(tokens::Event::Withdrawn {
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
			assert_ok!(<AdaptedBasicCurrency as fungible::Mutate<_>>::burn_from(&Alice::get(), 1000));
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 100000);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				100000
			);

			// Burn dust if remaining is less than ED.
			assert_eq!(
				<Currencies as fungibles::Mutate<_>>::burn_from(NATIVE_CURRENCY_ID, &Alice::get(), 99_999),
				Ok(100_000)
			);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::total_issuance(), 0);
		});
}

#[test]
fn fungible_transfer_trait_should_work() {
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
			assert_ok!(<Currencies as fungibles::Transfer<_>>::transfer(
				NATIVE_CURRENCY_ID,
				&Alice::get(),
				&Bob::get(),
				10000,
				true
			));
			System::assert_has_event(Event::Balances(pallet_balances::Event::Transfer {
				from: Alice::get(),
				to: Bob::get(),
				amount: 10000,
			}));
			System::assert_has_event(Event::Currencies(crate::Event::Transferred {
				currency_id: NATIVE_CURRENCY_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 10000,
			}));

			assert_noop!(
				<Currencies as fungibles::Transfer<_>>::transfer(NATIVE_CURRENCY_ID, &Alice::get(), &Bob::get(), 489_999, true),
				DispatchError::Module(ModuleError {
					index: 1,
					error: [4, 0, 0, 0],
					message: Some("KeepAlive")
				})
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
			assert_ok!(<Currencies as fungibles::Transfer<_>>::transfer(
				X_TOKEN_ID,
				&Alice::get(),
				&Bob::get(),
				10000,
				true
			));
			System::assert_has_event(Event::Tokens(tokens::Event::Transfer {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 10000,
			}));
			System::assert_has_event(Event::Currencies(crate::Event::Transferred {
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
			assert_ok!(<AdaptedBasicCurrency as fungible::Transfer<_>>::transfer(
				&Alice::get(),
				&Bob::get(),
				10000,
				true
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
			assert_ok!(<Currencies as fungibles::Unbalanced<_>>::set_balance(
				NATIVE_CURRENCY_ID,
				&Alice::get(),
				80000
			));
			System::assert_last_event(Event::Balances(pallet_balances::Event::BalanceSet {
				who: Alice::get(),
				free: 80000,
				reserved: 0,
			}));
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
			assert_ok!(<Currencies as fungibles::Unbalanced<_>>::set_balance(
				X_TOKEN_ID,
				&Alice::get(),
				80000
			));
			System::assert_last_event(Event::Tokens(tokens::Event::BalanceSet {
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
			assert_ok!(<AdaptedBasicCurrency as fungible::Unbalanced<_>>::set_balance(
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
			System::assert_last_event(Event::Tokens(tokens::Event::TotalIssuanceSet {
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
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &Alice::get()),
				0
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::can_hold(NATIVE_CURRENCY_ID, &Alice::get(), 499998),
				true
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::can_hold(NATIVE_CURRENCY_ID, &Alice::get(), 500001),
				false
			);
			assert_ok!(<Currencies as fungibles::MutateHold<_>>::hold(
				NATIVE_CURRENCY_ID,
				&Alice::get(),
				20000
			));
			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::hold(NATIVE_CURRENCY_ID, &Alice::get(), 500000),
				DispatchError::Module(ModuleError {
					index: 1,
					error: [2, 0, 0, 0],
					message: Some("InsufficientBalance",),
				})
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				500000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &Alice::get()),
				20000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &Alice::get()),
				0
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::can_hold(X_TOKEN_ID, &Alice::get(), 200000),
				true
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::can_hold(X_TOKEN_ID, &Alice::get(), 200001),
				false
			);
			assert_ok!(<Currencies as fungibles::MutateHold<_>>::hold(
				X_TOKEN_ID,
				&Alice::get(),
				20000
			));
			System::assert_last_event(Event::Tokens(tokens::Event::Reserved {
				currency_id: X_TOKEN_ID,
				who: Alice::get(),
				amount: 20000,
			}));

			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::hold(X_TOKEN_ID, &Alice::get(), 200000),
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
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &Alice::get()),
				20000
			);

			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				500000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&Alice::get()),
				20000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::can_hold(&Alice::get(), 20000),
				true
			);
			assert_ok!(<AdaptedBasicCurrency as fungible::MutateHold<_>>::hold(&Alice::get(), 20000));
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				500000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&Alice::get()),
				40000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				500000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &Alice::get()),
				40000
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::release(NATIVE_CURRENCY_ID, &Alice::get(), 10000, true),
				Ok(10000)
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				500000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &Alice::get()),
				30000
			);
			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::release(NATIVE_CURRENCY_ID, &Alice::get(), 50000, false),
				DispatchError::Module(ModuleError {
					index: 1,
					error: [2, 0, 0, 0],
					message: Some("InsufficientBalance")
				})
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::release(NATIVE_CURRENCY_ID, &Alice::get(), 50000, true),
				Ok(30000)
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &Alice::get()),
				0
			);
			assert_ok!(<Currencies as fungibles::MutateHold<_>>::hold(
				NATIVE_CURRENCY_ID,
				&Alice::get(),
				30000
			));

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &Alice::get()),
				20000
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::release(X_TOKEN_ID, &Alice::get(), 10000, true),
				Ok(10000)
			);
			System::assert_last_event(Event::Tokens(tokens::Event::Unreserved {
				currency_id: X_TOKEN_ID,
				who: Alice::get(),
				amount: 10000,
			}));

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &Alice::get()),
				10000
			);
			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::release(X_TOKEN_ID, &Alice::get(), 100000, false),
				DispatchError::Module(ModuleError {
					index: 2,
					error: [0, 0, 0, 0],
					message: Some("BalanceTooLow")
				})
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::release(X_TOKEN_ID, &Alice::get(), 100000, true),
				Ok(10000)
			);
			assert_ok!(<Currencies as fungibles::MutateHold<_>>::hold(
				X_TOKEN_ID,
				&Alice::get(),
				10000
			));

			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				500000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&Alice::get()),
				30000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::MutateHold<_>>::release(&Alice::get(), 10000, true),
				Ok(10000)
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				500000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&Alice::get()),
				20000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				500000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &Alice::get()),
				20000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Bob::get()),
				10000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &Bob::get()),
				0
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::transfer_held(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					&Bob::get(),
					2000,
					false,
					true
				),
				Ok(2000)
			);
			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::transfer_held(
					NATIVE_CURRENCY_ID,
					&Alice::get(),
					&Bob::get(),
					200000,
					false,
					true
				),
				DispatchError::Module(ModuleError {
					index: 1,
					error: [2, 0, 0, 0],
					message: Some("InsufficientBalance")
				})
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Alice::get()),
				498000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &Alice::get()),
				18000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(NATIVE_CURRENCY_ID, &Bob::get()),
				12000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(NATIVE_CURRENCY_ID, &Bob::get()),
				2000
			);

			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Alice::get()),
				200000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &Alice::get()),
				10000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Bob::get()),
				10000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &Bob::get()),
				0
			);
			assert_eq!(
				<Currencies as fungibles::MutateHold<_>>::transfer_held(
					X_TOKEN_ID,
					&Alice::get(),
					&Bob::get(),
					2000,
					false,
					true
				),
				Ok(2000)
			);
			System::assert_last_event(Event::Tokens(tokens::Event::ReserveRepatriated {
				currency_id: X_TOKEN_ID,
				from: Alice::get(),
				to: Bob::get(),
				amount: 2000,
				status: BalanceStatus::Reserved,
			}));

			assert_noop!(
				<Currencies as fungibles::MutateHold<_>>::transfer_held(
					X_TOKEN_ID,
					&Alice::get(),
					&Bob::get(),
					200000,
					false,
					true
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
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &Alice::get()),
				8000
			);
			assert_eq!(
				<Currencies as fungibles::Inspect<_>>::balance(X_TOKEN_ID, &Bob::get()),
				12000
			);
			assert_eq!(
				<Currencies as fungibles::InspectHold<_>>::balance_on_hold(X_TOKEN_ID, &Bob::get()),
				2000
			);

			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				498000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&Alice::get()),
				18000
			);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Bob::get()), 12000);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&Bob::get()),
				2000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::MutateHold<_>>::transfer_held(&Alice::get(), &Bob::get(), 2000, false, true),
				Ok(2000)
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Alice::get()),
				496000
			);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&Alice::get()),
				16000
			);
			assert_eq!(<AdaptedBasicCurrency as fungible::Inspect<_>>::balance(&Bob::get()), 14000);
			assert_eq!(
				<AdaptedBasicCurrency as fungible::InspectHold<_>>::balance_on_hold(&Bob::get()),
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
			Currencies::sweep_dust(Origin::signed(Bob::get()), DOT, accounts.clone()),
			DispatchError::BadOrigin
		);

		assert_ok!(Currencies::sweep_dust(
			Origin::signed(CouncilAccount::get()),
			DOT,
			accounts
		));
		System::assert_last_event(Event::Currencies(crate::Event::DustSwept {
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
				misc_frozen: 0,
				fee_frozen: 0,
			},
		));
		assert_ok!(<Runtime as pallet_balances::Config>::AccountStore::insert(
			&Eva::get(),
			pallet_balances::AccountData {
				free: 2,
				reserved: 0,
				misc_frozen: 0,
				fee_frozen: 0,
			},
		));
		assert_ok!(<Runtime as pallet_balances::Config>::AccountStore::insert(
			&Alice::get(),
			pallet_balances::AccountData {
				free: 0,
				reserved: 0,
				misc_frozen: 2,
				fee_frozen: 2,
			},
		));
		assert_ok!(<Runtime as pallet_balances::Config>::AccountStore::insert(
			&DustAccount::get(),
			pallet_balances::AccountData {
				free: 100,
				reserved: 0,
				misc_frozen: 0,
				fee_frozen: 0,
			},
		));
		pallet_balances::TotalIssuance::<Runtime>::put(104);

		assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &Bob::get()), 1);
		assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &Eva::get()), 2);
		assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &Alice::get()), 0);
		assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &DustAccount::get()), 100);

		let accounts = vec![Bob::get(), Eva::get(), Alice::get()];

		assert_noop!(
			Currencies::sweep_dust(Origin::signed(Bob::get()), NATIVE_CURRENCY_ID, accounts.clone()),
			DispatchError::BadOrigin
		);

		assert_ok!(Currencies::sweep_dust(
			Origin::signed(CouncilAccount::get()),
			NATIVE_CURRENCY_ID,
			accounts
		));
		System::assert_last_event(Event::Currencies(crate::Event::DustSwept {
			currency_id: NATIVE_CURRENCY_ID,
			who: Bob::get(),
			amount: 1,
		}));

		// Bob::get()'s account is gone
		assert_eq!(System::account_exists(&Bob::get()), false);
		assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &Bob::get()), 0);

		// Eva::get()'s account remains, not below ED
		assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &Eva::get()), 2);

		// Dust transferred to dust receiver
		assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &DustAccount::get()), 101);
		// Total issuance remains the same
		assert_eq!(Currencies::total_issuance(NATIVE_CURRENCY_ID), 104);
	});
}
