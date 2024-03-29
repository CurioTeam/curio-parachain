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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

use codec::{Codec, FullCodec, MaxEncodedLen};
use scale_info::TypeInfo;
use frame_support::{
	pallet_prelude::*,
	traits::{
		tokens::{fungible, fungibles, DepositConsequence, WithdrawConsequence, Precision, Fortitude, Preservation, Provenance, Restriction},
		Currency as PalletCurrency, ExistenceRequirement, Get, Imbalance,
		LockableCurrency as PalletLockableCurrency, ReservableCurrency as PalletReservableCurrency, WithdrawReasons,
	},
	transactional,
};
use frame_system::pallet_prelude::*;
use orml_traits::{
	arithmetic::{Signed, SimpleArithmetic},
	currency::{TransferAll, OnDust},
	BalanceStatus, BasicCurrency, BasicCurrencyExtended, BasicLockableCurrency, BasicReservableCurrency,
	LockIdentifier, MultiCurrency, MultiCurrencyExtended, MultiLockableCurrency, MultiReservableCurrency
};
use sp_runtime::{
	traits::{CheckedSub, MaybeSerializeDeserialize, StaticLookup, Zero},
	DispatchError, DispatchResult,
};
use sp_std::{fmt::Debug, marker, result, vec::Vec};

mod mock;
mod mock_currency;
mod tests;
pub mod weights;

pub use module::*;
pub use weights::WeightInfo;

type AmountOf<T> =
	<<T as Config>::MultiCurrency as MultiCurrencyExtended<<T as frame_system::Config>::AccountId>>::Amount;
type BalanceOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug + MaxEncodedLen + TypeInfo;
		type MultiCurrency: TransferAll<Self::AccountId>
			+ MultiCurrencyExtended<Self::AccountId, CurrencyId = Self::CurrencyId>
			+ MultiLockableCurrency<Self::AccountId, CurrencyId = Self::CurrencyId>
			+ MultiReservableCurrency<Self::AccountId, CurrencyId = Self::CurrencyId> 
			+ fungibles::Inspect<Self::AccountId, AssetId = Self::CurrencyId, Balance = BalanceOf<Self>>
			+ fungibles::Mutate<Self::AccountId, AssetId = Self::CurrencyId, Balance = BalanceOf<Self>>
			+ fungibles::Unbalanced<Self::AccountId, AssetId = Self::CurrencyId, Balance = BalanceOf<Self>>
			+ fungibles::InspectHold<Self::AccountId, AssetId = Self::CurrencyId, Balance = BalanceOf<Self>, Reason = ()>
			+ fungibles::MutateHold<Self::AccountId, AssetId = Self::CurrencyId, Balance = BalanceOf<Self>>
			+ fungibles::UnbalancedHold<Self::AccountId, AssetId = Self::CurrencyId, Balance = BalanceOf<Self>>;
		type NativeCurrency: BasicCurrencyExtended<Self::AccountId, Balance = BalanceOf<Self>, Amount = AmountOf<Self>>
			+ BasicLockableCurrency<Self::AccountId, Balance = BalanceOf<Self>>
			+ BasicReservableCurrency<Self::AccountId, Balance = BalanceOf<Self>>
			+ fungible::Inspect<Self::AccountId, Balance = BalanceOf<Self>>
			+ fungible::Mutate<Self::AccountId, Balance = BalanceOf<Self>>
			+ fungible::Unbalanced<Self::AccountId, Balance = BalanceOf<Self>>
			+ fungible::InspectHold<Self::AccountId, Balance = BalanceOf<Self>>
			+ fungible::MutateHold<Self::AccountId, Balance = BalanceOf<Self>>
			+ fungible::UnbalancedHold<Self::AccountId, Balance = BalanceOf<Self>>;

		/// The native currency id
		#[pallet::constant]
		type GetNativeCurrencyId: Get<Self::CurrencyId>;

		/// Weight information for extrinsics in this module.
		type WeightInfo: WeightInfo;

		/// The AccountId that can perform a sweep dust.
		type SweepOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Handler to burn or transfer account's dust
		type OnDust: OnDust<Self::AccountId, Self::CurrencyId, BalanceOf<Self>>;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Unable to convert the Amount type into Balance.
		AmountIntoBalanceFailed,
		/// Balance is too low.
		BalanceTooLow,
		/// Deposit result is not expected
		DepositFailed,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Currency transfer success.
		Transferred {
			currency_id: T::CurrencyId,
			from: T::AccountId,
			to: T::AccountId,
			amount: BalanceOf<T>,
		},
		/// Withdrawn some balances from an account
		Withdrawn {
			currency_id: T::CurrencyId,
			who: T::AccountId,
			amount: BalanceOf<T>,
		},
		/// Deposited some balance into an account
		Deposited {
			currency_id: T::CurrencyId,
			who: T::AccountId,
			amount: BalanceOf<T>,
		},
		/// Dust swept.
		DustSwept {
			currency_id: T::CurrencyId,
			who: T::AccountId,
			amount: BalanceOf<T>,
		},
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transfer some balance to another account under `currency_id`.
		///
		/// The dispatch origin for this call must be `Signed` by the
		/// transactor.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::transfer_non_native_currency())]
		pub fn transfer(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			currency_id: T::CurrencyId,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			let to = T::Lookup::lookup(dest)?;
			<Self as MultiCurrency<T::AccountId>>::transfer(currency_id, &from, &to, amount)
		}

		/// Transfer some native currency to another account.
		///
		/// The dispatch origin for this call must be `Signed` by the
		/// transactor.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::transfer_native_currency())]
		pub fn transfer_native_currency(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			let to = T::Lookup::lookup(dest)?;
			T::NativeCurrency::transfer(&from, &to, amount)
		}

		/// Update amount of account `who` under `currency_id`.
		///
		/// The dispatch origin of this call must be _Root_.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::update_balance_non_native_currency())]
		pub fn update_balance(
			origin: OriginFor<T>,
			who: <T::Lookup as StaticLookup>::Source,
			currency_id: T::CurrencyId,
			amount: AmountOf<T>,
		) -> DispatchResult {
			ensure_root(origin)?;
			let dest = T::Lookup::lookup(who)?;
			<Self as MultiCurrencyExtended<T::AccountId>>::update_balance(currency_id, &dest, amount)
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::sweep_dust(accounts.len() as u32))]
		pub fn sweep_dust(
			origin: OriginFor<T>,
			currency_id: T::CurrencyId,
			accounts: Vec<T::AccountId>,
		) -> DispatchResult {
			T::SweepOrigin::ensure_origin(origin)?;
			for account in accounts {
				let free_balance = Self::free_balance(currency_id, &account);
				if free_balance.is_zero() {
					continue;
				}
				let total_balance = Self::total_balance(currency_id, &account);
				if free_balance != total_balance {
					continue;
				}
				if free_balance < Self::minimum_balance(currency_id) {
					T::OnDust::on_dust(&account, currency_id, free_balance);
					Self::deposit_event(Event::<T>::DustSwept {
						currency_id,
						who: account,
						amount: free_balance,
					});
				}
			}
			Ok(())
		}
	}
}

impl<T: Config> MultiCurrency<T::AccountId> for Pallet<T> {
	type CurrencyId = T::CurrencyId;
	type Balance = BalanceOf<T>;
	fn minimum_balance(currency_id: Self::CurrencyId) -> Self::Balance {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => <T::NativeCurrency as BasicCurrency<_>>::minimum_balance(),
			_ => <T::MultiCurrency as MultiCurrency<_>>::minimum_balance(currency_id),
		}
	}

	fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => <T::NativeCurrency as BasicCurrency<_>>::total_issuance(),
			_ => <T::MultiCurrency as MultiCurrency<_>>::total_issuance(currency_id),
		}
	}

	fn total_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => <T::NativeCurrency as BasicCurrency<_>>::total_balance(who),
			_ => <T::MultiCurrency as MultiCurrency<_>>::total_balance(currency_id, who),
		}
	}

	fn free_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => <T::NativeCurrency as BasicCurrency<_>>::free_balance(who),
			_ => <T::MultiCurrency as MultiCurrency<_>>::free_balance(currency_id, who),
		}
	}

	fn ensure_can_withdraw(currency_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicCurrency<_>>::ensure_can_withdraw(who, amount)
			}
			_ => <T::MultiCurrency as MultiCurrency<_>>::ensure_can_withdraw(currency_id, who, amount),
		}
	}

	fn transfer(
		currency_id: Self::CurrencyId,
		from: &T::AccountId,
		to: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if amount.is_zero() || from == to {
			return Ok(());
		}

		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicCurrency<_>>::transfer(from, to, amount)?
			}
			_ => <T::MultiCurrency as MultiCurrency<_>>::transfer(currency_id, from, to, amount)?,
		}

		Self::deposit_event(Event::Transferred {
			currency_id,
			from: from.clone(),
			to: to.clone(),
			amount,
		});
		Ok(())
	}

	fn deposit(currency_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		if amount.is_zero() {
			return Ok(());
		}

		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => <T::NativeCurrency as BasicCurrency<_>>::deposit(who, amount),
			_ => <T::MultiCurrency as MultiCurrency<_>>::deposit(currency_id, who, amount),
		}
	}

	fn withdraw(currency_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		if amount.is_zero() {
			return Ok(());
		}

		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => <T::NativeCurrency as BasicCurrency<_>>::withdraw(who, amount),
			_ => <T::MultiCurrency as MultiCurrency<_>>::withdraw(currency_id, who, amount),
		}
	}

	fn can_slash(currency_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> bool {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicCurrency<_>>::can_slash(who, amount)
			}
			_ => <T::MultiCurrency as MultiCurrency<_>>::can_slash(currency_id, who, amount),
		}
	}

	fn slash(currency_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> Self::Balance {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => <T::NativeCurrency as BasicCurrency<_>>::slash(who, amount),
			_ => <T::MultiCurrency as MultiCurrency<_>>::slash(currency_id, who, amount),
		}
	}
}

impl<T: Config> MultiCurrencyExtended<T::AccountId> for Pallet<T> {
	type Amount = AmountOf<T>;

	fn update_balance(currency_id: Self::CurrencyId, who: &T::AccountId, by_amount: Self::Amount) -> DispatchResult {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicCurrencyExtended<_>>::update_balance(who, by_amount)
			}
			_ => <T::MultiCurrency as MultiCurrencyExtended<_>>::update_balance(currency_id, who, by_amount),
		}
	}
}

impl<T: Config> MultiLockableCurrency<T::AccountId> for Pallet<T> {
	type Moment = BlockNumberFor<T>;

	fn set_lock(
		lock_id: LockIdentifier,
		currency_id: Self::CurrencyId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicLockableCurrency<_>>::set_lock(lock_id, who, amount)
			}
			_ => <T::MultiCurrency as MultiLockableCurrency<_>>::set_lock(lock_id, currency_id, who, amount),
		}
	}

	fn extend_lock(
		lock_id: LockIdentifier,
		currency_id: Self::CurrencyId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicLockableCurrency<_>>::extend_lock(lock_id, who, amount)
			}
			_ => <T::MultiCurrency as MultiLockableCurrency<_>>::extend_lock(lock_id, currency_id, who, amount),
		}
	}

	fn remove_lock(lock_id: LockIdentifier, currency_id: Self::CurrencyId, who: &T::AccountId) -> DispatchResult {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicLockableCurrency<_>>::remove_lock(lock_id, who)
			}
			_ => <T::MultiCurrency as MultiLockableCurrency<_>>::remove_lock(lock_id, currency_id, who),
		}
	}
}

impl<T: Config> MultiReservableCurrency<T::AccountId> for Pallet<T> {
	fn can_reserve(currency_id: Self::CurrencyId, who: &T::AccountId, value: Self::Balance) -> bool {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicReservableCurrency<_>>::can_reserve(who, value)
			}
			_ => <T::MultiCurrency as MultiReservableCurrency<_>>::can_reserve(currency_id, who, value),
		}
	}

	fn slash_reserved(currency_id: Self::CurrencyId, who: &T::AccountId, value: Self::Balance) -> Self::Balance {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicReservableCurrency<_>>::slash_reserved(who, value)
			}
			_ => <T::MultiCurrency as MultiReservableCurrency<_>>::slash_reserved(currency_id, who, value),
		}
	}

	fn reserved_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicReservableCurrency<_>>::reserved_balance(who)
			}
			_ => <T::MultiCurrency as MultiReservableCurrency<_>>::reserved_balance(currency_id, who),
		}
	}

	fn reserve(currency_id: Self::CurrencyId, who: &T::AccountId, value: Self::Balance) -> DispatchResult {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicReservableCurrency<_>>::reserve(who, value)
			}
			_ => <T::MultiCurrency as MultiReservableCurrency<_>>::reserve(currency_id, who, value),
		}
	}

	fn unreserve(currency_id: Self::CurrencyId, who: &T::AccountId, value: Self::Balance) -> Self::Balance {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicReservableCurrency<_>>::unreserve(who, value)
			}
			_ => <T::MultiCurrency as MultiReservableCurrency<_>>::unreserve(currency_id, who, value),
		}
	}

	fn repatriate_reserved(
		currency_id: Self::CurrencyId,
		slashed: &T::AccountId,
		beneficiary: &T::AccountId,
		value: Self::Balance,
		status: BalanceStatus,
	) -> result::Result<Self::Balance, DispatchError> {
		match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicReservableCurrency<_>>::repatriate_reserved(
					slashed,
					beneficiary,
					value,
					status,
				)
			}
			_ => <T::MultiCurrency as MultiReservableCurrency<_>>::repatriate_reserved(
				currency_id,
				slashed,
				beneficiary,
				value,
				status,
			),
		}
	}
}

/// impl fungiles for Pallet<T>
impl<T: Config> fungibles::Inspect<T::AccountId> for Pallet<T> {
	type AssetId = T::CurrencyId;
	type Balance = BalanceOf<T>;

	fn total_issuance(asset_id: Self::AssetId) -> Self::Balance {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => <T::NativeCurrency as fungible::Inspect<_>>::total_issuance(),
			_ => <T::MultiCurrency as fungibles::Inspect<_>>::total_issuance(asset_id),
		}
	}

	fn minimum_balance(asset_id: Self::AssetId) -> Self::Balance {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => <T::NativeCurrency as fungible::Inspect<_>>::minimum_balance(),
			_ => <T::MultiCurrency as fungibles::Inspect<_>>::minimum_balance(asset_id),
		}
	}

	fn balance(asset_id: Self::AssetId, who: &T::AccountId) -> Self::Balance {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => <T::NativeCurrency as fungible::Inspect<_>>::balance(who),
			_ => <T::MultiCurrency as fungibles::Inspect<_>>::balance(asset_id, who),
		}
	}

	fn total_balance(asset_id: Self::AssetId, who: &T::AccountId) -> Self::Balance {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::Inspect<_>>::total_balance(who)
			}
			_ => <T::MultiCurrency as fungibles::Inspect<_>>::total_balance(asset_id, who),
		}
	}

	fn reducible_balance(
		asset_id: Self::AssetId,
		who: &T::AccountId,
		preservation: Preservation,
		force: Fortitude,
	) -> Self::Balance {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::Inspect<_>>::reducible_balance(who, preservation, force)
			}
			_ => <T::MultiCurrency as fungibles::Inspect<_>>::reducible_balance(asset_id, who, preservation, force),
		}
	}

	fn can_deposit(
		asset_id: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
		provenance: Provenance,
	) -> DepositConsequence {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::Inspect<_>>::can_deposit(who, amount, provenance)
			}
			_ => <T::MultiCurrency as fungibles::Inspect<_>>::can_deposit(asset_id, who, amount, provenance),
		}
	}

	fn can_withdraw(
		asset_id: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> WithdrawConsequence<Self::Balance> {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::Inspect<_>>::can_withdraw(who, amount)
			}
			_ => <T::MultiCurrency as fungibles::Inspect<_>>::can_withdraw(asset_id, who, amount),
		}
	}

	fn asset_exists(asset_id: Self::AssetId) -> bool {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => true,
			_ => <T::MultiCurrency as fungibles::Inspect<_>>::asset_exists(asset_id),
		}
	}
}

impl<T: Config> fungibles::Mutate<T::AccountId> for Pallet<T> {
	fn mint_into(
		asset_id: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::Mutate<_>>::mint_into(who, amount)
			}
			_ => <T::MultiCurrency as fungibles::Mutate<_>>::mint_into(asset_id, who, amount),
		}
	}

	fn burn_from(
		asset_id: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
		fortitude: Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::Mutate<_>>::burn_from(who, amount, precision, fortitude)
			}
			_ => <T::MultiCurrency as fungibles::Mutate<_>>::burn_from(asset_id, who, amount, precision, fortitude),
		}
	}

	fn transfer(
		asset_id: Self::AssetId,
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
		preservation: Preservation,
	) -> Result<Self::Balance, DispatchError> {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::Mutate<_>>::transfer(source, dest, amount, preservation).map(|actual| {
					Self::deposit_event(Event::Transferred {
						currency_id: asset_id,
						from: source.clone(),
						to: dest.clone(),
						amount: actual,
					});
					actual
				})
			}
			_ => <T::MultiCurrency as fungibles::Mutate<_>>::transfer(asset_id, source, dest, amount, preservation)
				.map(|actual| {
					Self::deposit_event(Event::Transferred {
						currency_id: asset_id,
						from: source.clone(),
						to: dest.clone(),
						amount: actual,
					});
					actual
				}),
		}
	}
}

impl<T: Config> fungibles::Unbalanced<T::AccountId> for Pallet<T> {
	fn handle_dust(_dust: fungibles::Dust<T::AccountId, Self>) {
		// https://github.com/paritytech/substrate/blob/569aae5341ea0c1d10426fa1ec13a36c0b64393b/frame/support/src/traits/tokens/fungibles/regular.rs#L124
		// Note: currently the field of Dust type is private and there is no constructor for it, so
		// we can't construct a Dust value and pass it. Do nothing here.
		// `Pallet<T>` overwrites these functions which can be called as user-level operation of
		// fungibles traits when calling these functions, it will not actually reach
		// `Unbalanced::handle_dust`.
	}

	fn write_balance(
		asset_id: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> Result<Option<Self::Balance>, DispatchError> {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::Unbalanced<_>>::write_balance(who, amount)
			}
			_ => <T::MultiCurrency as fungibles::Unbalanced<_>>::write_balance(asset_id, who, amount),
		}
	}

	fn set_total_issuance(asset_id: Self::AssetId, amount: Self::Balance) {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::Unbalanced<_>>::set_total_issuance(amount)
			}
			_ => <T::MultiCurrency as fungibles::Unbalanced<_>>::set_total_issuance(asset_id, amount),
		}
	}
}

type ReasonOf<P, T> = <P as fungibles::InspectHold<<T as frame_system::Config>::AccountId>>::Reason;
impl<T: Config> fungibles::InspectHold<T::AccountId> for Pallet<T> {
	type Reason = <T::NativeCurrency as fungible::InspectHold<T::AccountId>>::Reason;

	fn balance_on_hold(asset_id: Self::AssetId, reason: &Self::Reason, who: &T::AccountId) -> Self::Balance {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::InspectHold<_>>::balance_on_hold(reason, who)
			}
			_ => <T::MultiCurrency as fungibles::InspectHold<_>>::balance_on_hold(asset_id, &(), who),
		}
	}

	fn total_balance_on_hold(asset_id: Self::AssetId, who: &T::AccountId) -> Self::Balance {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::InspectHold<_>>::total_balance_on_hold(who)
			}
			_ => <T::MultiCurrency as fungibles::InspectHold<_>>::total_balance_on_hold(asset_id, who),
		}
	}

	fn reducible_total_balance_on_hold(asset_id: Self::AssetId, who: &T::AccountId, force: Fortitude) -> Self::Balance {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::InspectHold<_>>::reducible_total_balance_on_hold(who, force)
			}
			_ => <T::MultiCurrency as fungibles::InspectHold<_>>::reducible_total_balance_on_hold(asset_id, who, force),
		}
	}

	fn hold_available(asset_id: Self::AssetId, reason: &Self::Reason, who: &T::AccountId) -> bool {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::InspectHold<_>>::hold_available(reason, who)
			}
			_ => <T::MultiCurrency as fungibles::InspectHold<_>>::hold_available(asset_id, &(), who),
		}
	}

	fn can_hold(asset_id: Self::AssetId, reason: &Self::Reason, who: &T::AccountId, amount: Self::Balance) -> bool {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::InspectHold<_>>::can_hold(reason, who, amount)
			}
			_ => <T::MultiCurrency as fungibles::InspectHold<_>>::can_hold(asset_id, &(), who, amount),
		}
	}
}

impl<T: Config> fungibles::UnbalancedHold<T::AccountId> for Pallet<T> {
	fn set_balance_on_hold(
		asset_id: Self::AssetId,
		reason: &Self::Reason,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::UnbalancedHold<_>>::set_balance_on_hold(reason, who, amount)
			}
			_ => <T::MultiCurrency as fungibles::UnbalancedHold<_>>::set_balance_on_hold(asset_id, &(), who, amount),
		}
	}
}

impl<T: Config> fungibles::MutateHold<T::AccountId> for Pallet<T> {
	fn hold(
		asset_id: Self::AssetId,
		reason: &ReasonOf<Self, T>,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::MutateHold<_>>::hold(reason, who, amount)
			}
			_ => <T::MultiCurrency as fungibles::MutateHold<_>>::hold(asset_id, &(), who, amount),
		}
	}

	fn release(
		asset_id: Self::AssetId,
		reason: &ReasonOf<Self, T>,
		who: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
	) -> Result<Self::Balance, DispatchError> {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::MutateHold<_>>::release(reason, who, amount, precision)
			}
			_ => <T::MultiCurrency as fungibles::MutateHold<_>>::release(asset_id, &(), who, amount, precision),
		}
	}

	fn transfer_on_hold(
		asset_id: Self::AssetId,
		reason: &ReasonOf<Self, T>,
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
		restriction: Restriction,
		fortitude: Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		match asset_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as fungible::MutateHold<_>>::transfer_on_hold(
					reason,
					source,
					dest,
					amount,
					precision,
					restriction,
					fortitude,
				)
			}
			_ => <T::MultiCurrency as fungibles::MutateHold<_>>::transfer_on_hold(
				asset_id,
				&(),
				source,
				dest,
				amount,
				precision,
				restriction,
				fortitude,
			),
		}
	}
}

pub struct Currency<T, GetCurrencyId>(marker::PhantomData<T>, marker::PhantomData<GetCurrencyId>);

impl<T, GetCurrencyId> BasicCurrency<T::AccountId> for Currency<T, GetCurrencyId>
where
	T: Config,
	GetCurrencyId: Get<T::CurrencyId>,
{
	type Balance = BalanceOf<T>;

	fn minimum_balance() -> Self::Balance {
		<Pallet<T> as MultiCurrency<T::AccountId>>::minimum_balance(GetCurrencyId::get())
	}

	fn total_issuance() -> Self::Balance {
		<Pallet<T> as MultiCurrency<T::AccountId>>::total_issuance(GetCurrencyId::get())
	}

	fn total_balance(who: &T::AccountId) -> Self::Balance {
		<Pallet<T> as MultiCurrency<T::AccountId>>::total_balance(GetCurrencyId::get(), who)
	}

	fn free_balance(who: &T::AccountId) -> Self::Balance {
		<Pallet<T> as MultiCurrency<T::AccountId>>::free_balance(GetCurrencyId::get(), who)
	}

	fn ensure_can_withdraw(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<Pallet<T> as MultiCurrency<T::AccountId>>::ensure_can_withdraw(GetCurrencyId::get(), who, amount)
	}

	fn transfer(from: &T::AccountId, to: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<Pallet<T> as MultiCurrency<T::AccountId>>::transfer(GetCurrencyId::get(), from, to, amount)
	}

	fn deposit(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<Pallet<T> as MultiCurrency<T::AccountId>>::deposit(GetCurrencyId::get(), who, amount)
	}

	fn withdraw(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<Pallet<T> as MultiCurrency<T::AccountId>>::withdraw(GetCurrencyId::get(), who, amount)
	}

	fn can_slash(who: &T::AccountId, amount: Self::Balance) -> bool {
		<Pallet<T> as MultiCurrency<T::AccountId>>::can_slash(GetCurrencyId::get(), who, amount)
	}

	fn slash(who: &T::AccountId, amount: Self::Balance) -> Self::Balance {
		<Pallet<T> as MultiCurrency<T::AccountId>>::slash(GetCurrencyId::get(), who, amount)
	}
}

impl<T, GetCurrencyId> BasicCurrencyExtended<T::AccountId> for Currency<T, GetCurrencyId>
where
	T: Config,
	GetCurrencyId: Get<T::CurrencyId>,
{
	type Amount = AmountOf<T>;

	fn update_balance(who: &T::AccountId, by_amount: Self::Amount) -> DispatchResult {
		<Pallet<T> as MultiCurrencyExtended<T::AccountId>>::update_balance(GetCurrencyId::get(), who, by_amount)
	}
}

impl<T, GetCurrencyId> BasicLockableCurrency<T::AccountId> for Currency<T, GetCurrencyId>
where
	T: Config,
	GetCurrencyId: Get<T::CurrencyId>,
{
	type Moment = BlockNumberFor<T>;

	fn set_lock(lock_id: LockIdentifier, who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<Pallet<T> as MultiLockableCurrency<T::AccountId>>::set_lock(lock_id, GetCurrencyId::get(), who, amount)
	}

	fn extend_lock(lock_id: LockIdentifier, who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<Pallet<T> as MultiLockableCurrency<T::AccountId>>::extend_lock(lock_id, GetCurrencyId::get(), who, amount)
	}

	fn remove_lock(lock_id: LockIdentifier, who: &T::AccountId) -> DispatchResult {
		<Pallet<T> as MultiLockableCurrency<T::AccountId>>::remove_lock(lock_id, GetCurrencyId::get(), who)
	}
}

impl<T, GetCurrencyId> BasicReservableCurrency<T::AccountId> for Currency<T, GetCurrencyId>
where
	T: Config,
	GetCurrencyId: Get<T::CurrencyId>,
{
	fn can_reserve(who: &T::AccountId, value: Self::Balance) -> bool {
		<Pallet<T> as MultiReservableCurrency<T::AccountId>>::can_reserve(GetCurrencyId::get(), who, value)
	}

	fn slash_reserved(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
		<Pallet<T> as MultiReservableCurrency<T::AccountId>>::slash_reserved(GetCurrencyId::get(), who, value)
	}

	fn reserved_balance(who: &T::AccountId) -> Self::Balance {
		<Pallet<T> as MultiReservableCurrency<T::AccountId>>::reserved_balance(GetCurrencyId::get(), who)
	}

	fn reserve(who: &T::AccountId, value: Self::Balance) -> DispatchResult {
		<Pallet<T> as MultiReservableCurrency<T::AccountId>>::reserve(GetCurrencyId::get(), who, value)
	}

	fn unreserve(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
		<Pallet<T> as MultiReservableCurrency<T::AccountId>>::unreserve(GetCurrencyId::get(), who, value)
	}

	fn repatriate_reserved(
		slashed: &T::AccountId,
		beneficiary: &T::AccountId,
		value: Self::Balance,
		status: BalanceStatus,
	) -> result::Result<Self::Balance, DispatchError> {
		<Pallet<T> as MultiReservableCurrency<T::AccountId>>::repatriate_reserved(
			GetCurrencyId::get(),
			slashed,
			beneficiary,
			value,
			status,
		)
	}
}

impl<T, GetCurrencyId> fungible::Inspect<T::AccountId> for Currency<T, GetCurrencyId>
where
	T: Config,
	GetCurrencyId: Get<T::CurrencyId>,
{
	type Balance = BalanceOf<T>;

	fn total_issuance() -> Self::Balance {
		<Pallet<T> as fungibles::Inspect<_>>::total_issuance(GetCurrencyId::get())
	}

	fn minimum_balance() -> Self::Balance {
		<Pallet<T> as fungibles::Inspect<_>>::minimum_balance(GetCurrencyId::get())
	}

	fn balance(who: &T::AccountId) -> Self::Balance {
		<Pallet<T> as fungibles::Inspect<_>>::balance(GetCurrencyId::get(), who)
	}

	fn total_balance(who: &T::AccountId) -> Self::Balance {
		<Pallet<T> as fungibles::Inspect<_>>::total_balance(GetCurrencyId::get(), who)
	}

	fn reducible_balance(who: &T::AccountId, preservation: Preservation, force: Fortitude) -> Self::Balance {
		<Pallet<T> as fungibles::Inspect<_>>::reducible_balance(GetCurrencyId::get(), who, preservation, force)
	}

	fn can_deposit(who: &T::AccountId, amount: Self::Balance, provenance: Provenance) -> DepositConsequence {
		<Pallet<T> as fungibles::Inspect<_>>::can_deposit(GetCurrencyId::get(), who, amount, provenance)
	}

	fn can_withdraw(who: &T::AccountId, amount: Self::Balance) -> WithdrawConsequence<Self::Balance> {
		<Pallet<T> as fungibles::Inspect<_>>::can_withdraw(GetCurrencyId::get(), who, amount)
	}
}

impl<T, GetCurrencyId> fungible::Mutate<T::AccountId> for Currency<T, GetCurrencyId>
where
	T: Config,
	GetCurrencyId: Get<T::CurrencyId>,
{
	fn mint_into(who: &T::AccountId, amount: Self::Balance) -> Result<Self::Balance, DispatchError> {
		<Pallet<T> as fungibles::Mutate<_>>::mint_into(GetCurrencyId::get(), who, amount)
	}

	fn burn_from(
		who: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
		fortitude: Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		<Pallet<T> as fungibles::Mutate<_>>::burn_from(GetCurrencyId::get(), who, amount, precision, fortitude)
	}

	fn transfer(
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
		preservation: Preservation,
	) -> Result<Self::Balance, DispatchError> {
		<Pallet<T> as fungibles::Mutate<_>>::transfer(GetCurrencyId::get(), source, dest, amount, preservation)
	}
}



impl<T, GetCurrencyId> fungible::Unbalanced<T::AccountId> for Currency<T, GetCurrencyId>
where
	T: Config,
	GetCurrencyId: Get<T::CurrencyId>,
{
	fn handle_dust(_dust: fungible::Dust<T::AccountId, Self>) {
		// https://github.com/paritytech/substrate/blob/569aae5341ea0c1d10426fa1ec13a36c0b64393b/frame/support/src/traits/tokens/fungibles/regular.rs#L124
		// Note: currently the field of Dust type is private and there is no constructor for it, so
		// we can't construct a Dust value and pass it. Do nothing here.
		// `Pallet<T>` overwrites these functions which can be called as user-level operation of
		// fungibles traits when calling these functions, it will not actually reach
		// `Unbalanced::handle_dust`.
	}

	fn write_balance(who: &T::AccountId, amount: Self::Balance) -> Result<Option<Self::Balance>, DispatchError> {
		<Pallet<T> as fungibles::Unbalanced<_>>::write_balance(GetCurrencyId::get(), who, amount)
	}

	fn set_total_issuance(amount: Self::Balance) {
		<Pallet<T> as fungibles::Unbalanced<_>>::set_total_issuance(GetCurrencyId::get(), amount)
	}
}

impl<T, GetCurrencyId> fungible::InspectHold<T::AccountId> for Currency<T, GetCurrencyId>
where
	T: Config,
	GetCurrencyId: Get<T::CurrencyId>,
{
	type Reason = ReasonOf<Pallet<T>, T>;

	fn balance_on_hold(reason: &Self::Reason, who: &T::AccountId) -> Self::Balance {
		<Pallet<T> as fungibles::InspectHold<_>>::balance_on_hold(GetCurrencyId::get(), reason, who)
	}
	fn total_balance_on_hold(who: &T::AccountId) -> Self::Balance {
		<Pallet<T> as fungibles::InspectHold<_>>::total_balance_on_hold(GetCurrencyId::get(), who)
	}
	fn reducible_total_balance_on_hold(who: &T::AccountId, force: Fortitude) -> Self::Balance {
		<Pallet<T> as fungibles::InspectHold<_>>::reducible_total_balance_on_hold(GetCurrencyId::get(), who, force)
	}
	fn hold_available(reason: &Self::Reason, who: &T::AccountId) -> bool {
		<Pallet<T> as fungibles::InspectHold<_>>::hold_available(GetCurrencyId::get(), reason, who)
	}
	fn can_hold(reason: &Self::Reason, who: &T::AccountId, amount: Self::Balance) -> bool {
		<Pallet<T> as fungibles::InspectHold<_>>::can_hold(GetCurrencyId::get(), reason, who, amount)
	}
}

type ReasonOfFungible<P, T> = <P as fungible::InspectHold<<T as frame_system::Config>::AccountId>>::Reason;
impl<T, GetCurrencyId> fungible::UnbalancedHold<T::AccountId> for Currency<T, GetCurrencyId>
where
	T: Config,
	GetCurrencyId: Get<T::CurrencyId>,
{
	fn set_balance_on_hold(
		reason: &ReasonOfFungible<Self, T>,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		<Pallet<T> as fungibles::UnbalancedHold<_>>::set_balance_on_hold(GetCurrencyId::get(), reason, who, amount)
	}
}

impl<T, GetCurrencyId> fungible::MutateHold<T::AccountId> for Currency<T, GetCurrencyId>
where
	T: Config,
	GetCurrencyId: Get<T::CurrencyId>,
{
	fn hold(reason: &ReasonOfFungible<Self, T>, who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<Pallet<T> as fungibles::MutateHold<_>>::hold(GetCurrencyId::get(), reason, who, amount)
	}
	fn release(
		reason: &ReasonOfFungible<Self, T>,
		who: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
	) -> Result<Self::Balance, DispatchError> {
		<Pallet<T> as fungibles::MutateHold<_>>::release(GetCurrencyId::get(), reason, who, amount, precision)
	}
	fn transfer_on_hold(
		reason: &ReasonOfFungible<Self, T>,
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
		restriction: Restriction,
		fortitude: Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		<Pallet<T> as fungibles::MutateHold<_>>::transfer_on_hold(
			GetCurrencyId::get(),
			reason,
			source,
			dest,
			amount,
			precision,
			restriction,
			fortitude,
		)
	}
}

/// Adapt other currency traits implementation to `BasicCurrency`.
pub struct BasicCurrencyAdapter<T, Currency, Amount, Moment>(marker::PhantomData<(T, Currency, Amount, Moment)>);

type PalletBalanceOf<A, Currency> = <Currency as PalletCurrency<A>>::Balance;

// Adapt `frame_support::traits::Currency`
impl<T, AccountId, Currency, Amount, Moment> BasicCurrency<AccountId>
	for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
	Currency: PalletCurrency<AccountId>,
	T: Config,
{
	type Balance = PalletBalanceOf<AccountId, Currency>;

	fn minimum_balance() -> Self::Balance {
		<Currency as PalletCurrency<_>>::minimum_balance()
	}

	fn total_issuance() -> Self::Balance {
		<Currency as PalletCurrency<_>>::total_issuance()
	}

	fn total_balance(who: &AccountId) -> Self::Balance {
		<Currency as PalletCurrency<_>>::total_balance(who)
	}

	fn free_balance(who: &AccountId) -> Self::Balance {
		<Currency as PalletCurrency<_>>::free_balance(who)
	}

	fn ensure_can_withdraw(who: &AccountId, amount: Self::Balance) -> DispatchResult {
		let new_balance = Self::free_balance(who)
			.checked_sub(&amount)
			.ok_or(Error::<T>::BalanceTooLow)?;

		<Currency as PalletCurrency<_>>::ensure_can_withdraw(who, amount, WithdrawReasons::all(), new_balance)
	}

	fn transfer(from: &AccountId, to: &AccountId, amount: Self::Balance) -> DispatchResult {
		<Currency as PalletCurrency<_>>::transfer(from, to, amount, ExistenceRequirement::AllowDeath)
	}

	fn deposit(who: &AccountId, amount: Self::Balance) -> DispatchResult {
		if !amount.is_zero() {
			let deposit_result = <Currency as PalletCurrency<_>>::deposit_creating(who, amount);
			let actual_deposit = deposit_result.peek();
			ensure!(actual_deposit == amount, Error::<T>::DepositFailed);
		}

		Ok(())
	}

	fn withdraw(who: &AccountId, amount: Self::Balance) -> DispatchResult {
		<Currency as PalletCurrency<_>>::withdraw(who, amount, WithdrawReasons::all(), ExistenceRequirement::AllowDeath)
			.map(|_| ())
	}

	fn can_slash(who: &AccountId, amount: Self::Balance) -> bool {
		<Currency as PalletCurrency<_>>::can_slash(who, amount)
	}

	fn slash(who: &AccountId, amount: Self::Balance) -> Self::Balance {
		let (_, gap) = <Currency as PalletCurrency<_>>::slash(who, amount);
		gap
	}
}

// Adapt `frame_support::traits::Currency`
impl<T, AccountId, Currency, Amount, Moment> BasicCurrencyExtended<AccountId>
	for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
	Amount: Signed
		+ TryInto<PalletBalanceOf<AccountId, Currency>>
		+ TryFrom<PalletBalanceOf<AccountId, Currency>>
		+ SimpleArithmetic
		+ Codec
		+ Copy
		+ MaybeSerializeDeserialize
		+ Debug
		+ Default
		+ MaxEncodedLen,
	Currency: PalletCurrency<AccountId>,
	T: Config,
{
	type Amount = Amount;

	fn update_balance(who: &AccountId, by_amount: Self::Amount) -> DispatchResult {
		let by_balance = by_amount
			.abs()
			.try_into()
			.map_err(|_| Error::<T>::AmountIntoBalanceFailed)?;
		if by_amount.is_positive() {
			Self::deposit(who, by_balance)
		} else {
			Self::withdraw(who, by_balance)
		}
	}
}

// Adapt `frame_support::traits::LockableCurrency`
impl<T, AccountId, Currency, Amount, Moment> BasicLockableCurrency<AccountId>
	for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
	Currency: PalletLockableCurrency<AccountId>,
	T: Config,
{	
	type Moment = Moment;

	fn set_lock(lock_id: LockIdentifier, who: &AccountId, amount: Self::Balance) -> DispatchResult {
		<Currency as PalletLockableCurrency<_>>::set_lock(lock_id, who, amount, WithdrawReasons::all());
		Ok(())
	}

	fn extend_lock(lock_id: LockIdentifier, who: &AccountId, amount: Self::Balance) -> DispatchResult {
		<Currency as PalletLockableCurrency<_>>::extend_lock(lock_id, who, amount, WithdrawReasons::all());
		Ok(())
	}

	fn remove_lock(lock_id: LockIdentifier, who: &AccountId) -> DispatchResult {
		<Currency as PalletLockableCurrency<_>>::remove_lock(lock_id, who);
		Ok(())
	}
}

// Adapt `frame_support::traits::ReservableCurrency`
impl<T, AccountId, Currency, Amount, Moment> BasicReservableCurrency<AccountId>
	for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
	Currency: PalletReservableCurrency<AccountId>,
	T: Config,
{
	fn can_reserve(who: &AccountId, value: Self::Balance) -> bool {
		<Currency as PalletReservableCurrency<_>>::can_reserve(who, value)
	}

	fn slash_reserved(who: &AccountId, value: Self::Balance) -> Self::Balance {
		let (_, gap) = <Currency as PalletReservableCurrency<_>>::slash_reserved(who, value);
		gap
	}

	fn reserved_balance(who: &AccountId) -> Self::Balance {
		<Currency as PalletReservableCurrency<_>>::reserved_balance(who)
	}

	fn reserve(who: &AccountId, value: Self::Balance) -> DispatchResult {
		<Currency as PalletReservableCurrency<_>>::reserve(who, value)
	}

	fn unreserve(who: &AccountId, value: Self::Balance) -> Self::Balance {
		<Currency as PalletReservableCurrency<_>>::unreserve(who, value)
	}

	fn repatriate_reserved(
		slashed: &AccountId,
		beneficiary: &AccountId,
		value: Self::Balance,
		status: BalanceStatus,
	) -> result::Result<Self::Balance, DispatchError> {
		<Currency as PalletReservableCurrency<_>>::repatriate_reserved(slashed, beneficiary, value, status)
	}
}

type FungibleBalanceOf<A, Currency> = <Currency as fungible::Inspect<A>>::Balance;

impl<T, Currency, Amount, Moment> fungible::Inspect<T::AccountId>
	for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
	Currency: fungible::Inspect<T::AccountId>,
	T: Config,
{
	type Balance = FungibleBalanceOf<T::AccountId, Currency>;

	fn total_issuance() -> Self::Balance {
		<Currency as fungible::Inspect<_>>::total_issuance()
	}
	fn minimum_balance() -> Self::Balance {
		<Currency as fungible::Inspect<_>>::minimum_balance()
	}
	fn balance(who: &T::AccountId) -> Self::Balance {
		<Currency as fungible::Inspect<_>>::balance(who)
	}
	fn total_balance(who: &T::AccountId) -> Self::Balance {
		<Currency as fungible::Inspect<_>>::total_balance(who)
	}
	fn reducible_balance(who: &T::AccountId, preservation: Preservation, force: Fortitude) -> Self::Balance {
		<Currency as fungible::Inspect<_>>::reducible_balance(who, preservation, force)
	}
	fn can_deposit(who: &T::AccountId, amount: Self::Balance, provenance: Provenance) -> DepositConsequence {
		<Currency as fungible::Inspect<_>>::can_deposit(who, amount, provenance)
	}
	fn can_withdraw(who: &T::AccountId, amount: Self::Balance) -> WithdrawConsequence<Self::Balance> {
		<Currency as fungible::Inspect<_>>::can_withdraw(who, amount)
	}
}

impl<T, Currency, Amount, Moment> fungible::Mutate<T::AccountId>
	for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
	Currency: fungible::Mutate<T::AccountId>,
	T: Config,
{
	fn mint_into(who: &T::AccountId, amount: Self::Balance) -> Result<Self::Balance, DispatchError> {
		<Currency as fungible::Mutate<_>>::mint_into(who, amount)
	}

	fn burn_from(
		who: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
		fortitude: Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		<Currency as fungible::Mutate<_>>::burn_from(who, amount, precision, fortitude)
	}

	fn transfer(
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
		preservation: Preservation,
	) -> Result<Self::Balance, DispatchError> {
		<Currency as fungible::Mutate<_>>::transfer(source, dest, amount, preservation)
	}
}

impl<T, Currency, Amount, Moment> fungible::Unbalanced<T::AccountId>
	for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
	Currency: fungible::Unbalanced<T::AccountId>,
	T: Config,
{
	fn handle_dust(_dust: fungible::Dust<T::AccountId, Self>) {
		// https://github.com/paritytech/substrate/blob/569aae5341ea0c1d10426fa1ec13a36c0b64393b/frame/support/src/traits/tokens/fungibles/regular.rs#L124
		// Note: currently the field of Dust type is private and there is no constructor for it, so
		// we can't construct a Dust value and pass it.
		// `BasicCurrencyAdapter` overwrites these functions which can be called as user-level
		// operation of fungible traits when calling these functions, it will not actually reach
		// `Unbalanced::handle_dust`.
	}

	fn write_balance(who: &T::AccountId, amount: Self::Balance) -> Result<Option<Self::Balance>, DispatchError> {
		<Currency as fungible::Unbalanced<_>>::write_balance(who, amount)
	}

	fn set_total_issuance(amount: Self::Balance) {
		<Currency as fungible::Unbalanced<_>>::set_total_issuance(amount)
	}
}

impl<T, Currency, Amount, Moment> fungible::InspectHold<T::AccountId>
	for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
	Currency: fungible::InspectHold<T::AccountId>,
	T: Config,
{
	type Reason = <Currency as fungible::InspectHold<T::AccountId>>::Reason;

	fn balance_on_hold(reason: &Self::Reason, who: &T::AccountId) -> Self::Balance {
		<Currency as fungible::InspectHold<_>>::balance_on_hold(reason, who)
	}
	fn total_balance_on_hold(who: &T::AccountId) -> Self::Balance {
		<Currency as fungible::InspectHold<_>>::total_balance_on_hold(who)
	}
	fn reducible_total_balance_on_hold(who: &T::AccountId, force: Fortitude) -> Self::Balance {
		<Currency as fungible::InspectHold<_>>::reducible_total_balance_on_hold(who, force)
	}
	fn hold_available(reason: &Self::Reason, who: &T::AccountId) -> bool {
		<Currency as fungible::InspectHold<_>>::hold_available(reason, who)
	}
	fn can_hold(reason: &Self::Reason, who: &T::AccountId, amount: Self::Balance) -> bool {
		<Currency as fungible::InspectHold<_>>::can_hold(reason, who, amount)
	}
}

impl<T, Currency, Amount, Moment> fungible::UnbalancedHold<T::AccountId>
	for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
	Currency: fungible::UnbalancedHold<T::AccountId>,
	T: Config,
{
	fn set_balance_on_hold(
		reason: &ReasonOfFungible<Self, T>,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		<Currency as fungible::UnbalancedHold<_>>::set_balance_on_hold(reason, who, amount)
	}
}

impl<T, Currency, Amount, Moment> fungible::MutateHold<T::AccountId>
	for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
	Currency: fungible::MutateHold<T::AccountId>,
	T: Config,
{
	fn hold(reason: &ReasonOfFungible<Self, T>, who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<Currency as fungible::MutateHold<_>>::hold(reason, who, amount)
	}

	fn release(
		reason: &ReasonOfFungible<Self, T>,
		who: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
	) -> Result<Self::Balance, DispatchError> {
		<Currency as fungible::MutateHold<_>>::release(reason, who, amount, precision)
	}

	fn transfer_on_hold(
		reason: &ReasonOfFungible<Self, T>,
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
		restriction: Restriction,
		fortitude: Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		<Currency as fungible::MutateHold<_>>::transfer_on_hold(
			reason,
			source,
			dest,
			amount,
			precision,
			restriction,
			fortitude,
		)
	}
}

impl<T: Config> TransferAll<T::AccountId> for Pallet<T> {
	#[transactional]
	fn transfer_all(source: &T::AccountId, dest: &T::AccountId) -> DispatchResult {
		// transfer non-native free to dest
		<T::MultiCurrency as TransferAll<_>>::transfer_all(source, dest)?;

		// transfer all free to dest
		<T::NativeCurrency as BasicCurrency<_>>::transfer(
			source,
			dest,
			<T::NativeCurrency as BasicCurrency<_>>::free_balance(source),
		)
	}
}

pub struct TransferDust<T, GetAccountId>(marker::PhantomData<(T, GetAccountId)>);
impl<T: Config, GetAccountId> OnDust<T::AccountId, T::CurrencyId, BalanceOf<T>> for TransferDust<T, GetAccountId>
where
	T: Config,
	GetAccountId: Get<T::AccountId>,
{
	fn on_dust(who: &T::AccountId, currency_id: T::CurrencyId, amount: BalanceOf<T>) {
		// transfer the dust to treasury account, ignore the result,
		// if failed will leave some dust which still could be recycled.
		let _ = match currency_id {
			id if id == T::GetNativeCurrencyId::get() => {
				<T::NativeCurrency as BasicCurrency<_>>::transfer(who, &GetAccountId::get(), amount)
			}
			_ => <T::MultiCurrency as MultiCurrency<_>>::transfer(currency_id, who, &GetAccountId::get(), amount),
		};
	}
}