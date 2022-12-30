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

//! # Whitelist pallet
//! 
//! ## Overview
//! 
//! The goal of this module is to control Security Assets possesion
//! and transfering by listing priviliged accounts (so called Investors) able to manage
//! Security Assets.
//! 
//! Whitelisted investors and their statuses (active or not) are controlled by
//! special roles which are: `RolesRoot`, `Admin` and `Manager`
//! - `RolesRoot` is able to assign and resign `Admin`
//! - `Admin` is the most priveleged role with plenty of control possibilities
//! - `Manager` is able to control investors statuses

#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::ensure_signed;
pub use pallet::*;

use codec::MaxEncodedLen;
use scale_info::TypeInfo;

use sp_core::{Encode, Decode};
use sp_std::prelude::*;

use frame_support::{traits::EnsureOrigin};
use frame_support::{pallet_prelude::*, dispatch::DispatchError, fail};
use frame_system::pallet_prelude::*;

use module_support::is_vec_unique;

pub mod traits;
use traits::WhitelistInterface;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::WeightInfo;

/// Struct presenting investor with corresponding `AccountId` and status
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Investor<AccountId> {
	/// Investor AccountId
	pub account: AccountId,
	/// Investor status
	/// `false` - investor is not active and can't transfer and accept Security Assets
	/// `true` - investor is active
	pub is_active: bool,
}

/// Investor KYC hash
pub type InvestorKey = [u8; 32];

/// Maximum number of investors that can be added at a time
pub const MAX_NEW_INVESTORS: u8 = 100;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Superuser able to assign all other roles
		type RolesRoot: EnsureOrigin<Self::RuntimeOrigin>;

		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type WeightInfo: weights::WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// [InvestorKey] mappend to [Investor]
	#[pallet::storage]
	#[pallet::getter(fn investor)]
	pub type Investors<T: Config> = StorageMap<_, Blake2_128Concat, InvestorKey, Investor<T::AccountId>>;

	/// Investor's AccountId mapped to [InvestorKey]
	#[pallet::storage]
	#[pallet::getter(fn investor_key)]
	pub type KeysOfInvestors<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, InvestorKey>;

	/// Admins accounts
	#[pallet::storage]
	#[pallet::getter(fn is_admin)]
	pub type Admins<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

	/// Managers accounts
	#[pallet::storage]
	#[pallet::getter(fn is_manager)]
	pub type Managers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New `Admin` added
		AddAdmin {
			new_admin: T::AccountId,
		},
		/// `Admin` removed
		RemoveAdmin {
			admin: T::AccountId
		},
		/// New `Manager` added
		AddManager {
			who: T::AccountId, 
			new_manager: T::AccountId,
		},
		/// `Manager` removed
		RemoveManager {
			who: T::AccountId, 
			manager: T::AccountId,
		},
		/// New `Investor` added
		AddInvestor {
			who: T::AccountId, 
			investor_key: InvestorKey,
			investor: Investor<T::AccountId>,
		},
		/// 'Investor' active status changed
		InvestorStatusSet {
			who: T::AccountId, 
			investor: T::AccountId,
			is_active: bool
		},
		/// 'Investor' account changed
		InvestorAccountChanged {
			who: T::AccountId, 
			old_account: T::AccountId,
			new_account: T::AccountId
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Call argument is invalid (possibly empty)
		InvalidInput,
		/// RuntimeOrigin does not pass roles check
		PermissionDenied,
		/// `Admin` with this AccountId already exists
		WhitelistAdminExists,
		/// Account is not `Admin`
		NotWhitelistAdmin,
		/// `Manager` with this AccountId already exists
		WhitelistManagerExists,
		/// Account is not `Manager`
		NotWhitelistManager,
		/// Account is already investor
		AccountAlreadyInvestor,
		/// Two investors with same AccountId given
		AccountDuplicate,
		/// InvestorKey is already used
		InvestorKeyExists,
		/// Two investors with same InvestorKey given
		KeyDuplicate,
		/// Account is not investor
		NotInvestor,
		/// Investor status is already active
		AlreadyActive,
		/// Investor status is already not active
		AlreadyNotActive,
		/// Can't change investor's 
		SameAddress
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// Initially whitelisted investors
		pub investors: Vec<(InvestorKey, Investor<T::AccountId>)>,
		/// Initial list of admins
		pub admins: Vec<T::AccountId>,
		/// Initial list of managers
		pub managers: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { 
				investors: vec![],
				admins: vec![],
				managers: vec![]
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Ok(_) = Pallet::<T>::_verify_add_investors(&self.investors) {
				for (investor_key, investor) in &self.investors {
					KeysOfInvestors::<T>::insert(investor.account.clone(), investor_key.clone());
					Investors::<T>::insert(investor_key.clone(), investor.clone());
				}
			}

			for admin in &self.admins {
				Admins::<T>::insert(admin, true);
			}

			for manager in &self.managers {
				Managers::<T>::insert(manager, true);
			}
		}
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add admin
		/// 
		/// - `new_admin` - AccountId to be `Admin`
		/// 
		/// Fails:
		/// - with [PermissionDenied](crate::pallet::Error::PermissionDenied) when origin is not `RolesRoot`
		/// - with [AdminExists](crate::pallet::Error::AdminExists) when given AccountId is already admin
		#[pallet::weight(<T as Config>::WeightInfo::add_admin())]
		pub fn add_admin(origin: OriginFor<T>, new_admin: T::AccountId) -> DispatchResult {
			T::RolesRoot::ensure_origin(origin)?;

			Self::_add_admin(&new_admin)?;

			Ok(().into())
		}

		/// Remove admin
		/// 
		/// - 'admin' - admin's AccountId to be removed
		/// 
		/// Fails:
		/// - with [PermissionDenied](crate::pallet::Error::PermissionDenied) when origin is not `RolesRoot`
		/// - with [NotAdmin](crate::pallet::Error::NotAdmin) when given AccountId is not admin
		#[pallet::weight(<T as Config>::WeightInfo::remove_admin())]
		pub fn remove_admin(origin: OriginFor<T>, admin: T::AccountId) -> DispatchResult {
			T::RolesRoot::ensure_origin(origin)?;

			Self::_remove_admin(&admin)?;

			Ok(().into())
		}

		/// Add manager
		/// 
		/// - `new_manager` - AccountId to be `Manager`
		/// 
		/// Fails:
		/// - with [PermissionDenied](crate::pallet::Error::PermissionDenied) when origin is not `Admin`
		/// - with [ManagerExists](crate::pallet::Error::ManagerExists) when given AccountId is already manager
		#[pallet::weight(<T as Config>::WeightInfo::add_manager())]
		pub fn add_manager(origin: OriginFor<T>, new_manager: T::AccountId) -> DispatchResult {
			let who = Self::ensure_origin_is_admin(origin)?;

			Self::_add_manager(&who, &new_manager)?;

			Ok(().into())
		}

		/// Remove manager
		/// 
		/// - `manager` - AccountId to be removed
		/// 
		/// Fails:
		/// - with [PermissionDenied](crate::pallet::Error::PermissionDenied) when origin is not `Admin`
		/// - with [NotManager](crate::pallet::Error::NotManager) when given AccountId is not manager
		#[pallet::weight(<T as Config>::WeightInfo::remove_manager())]
		pub fn remove_manager(origin: OriginFor<T>, manager: T::AccountId) -> DispatchResult {
			let who = Self::ensure_origin_is_admin(origin)?;

			Self::_remove_manager(&who, &manager)?;

			Ok(().into())
		}

		/// Add investors
		/// 
		/// - `new_investors` - array of ([InvestorKey], [Investor]) to be added
		/// 
		/// Fails:
		/// - with [PermissionDenied](crate::pallet::Error::PermissionDenied) when origin is not `Admin` or `Manager`
		/// - with [NotInvestor](crate::pallet::Error::NotInvestor) when given AccountId is not investor
		/// - with [AlreadyActive](crate::pallet::Error::AlreadyActive) when given AccountId is not investor
		/// - with [AlreadyNotActive](crate::pallet::Error::AlreadyNotActive) when given AccountId is not investor
		/// - with [InvalidInput](crate::pallet::Error::InvalidInput) when `new_investors` is empty or break [MAX_NEW_INVESTORS]
		#[pallet::weight(<T as Config>::WeightInfo::add_investors(new_investors.len() as u32))]
		pub fn add_investors(origin: OriginFor<T>, new_investors: Vec<(InvestorKey, Investor<T::AccountId>)>) -> DispatchResult {
			let who = Self::ensure_origin_is_admin_or_manager(origin)?;

			Self::_add_investors(&who, new_investors)?;

			Ok(().into())
		}

		/// Set investor status
		/// 
		/// - `investor_account` - investor's AccountId
		/// 
		/// Fails:
		/// - with [PermissionDenied](crate::pallet::Error::PermissionDenied) when origin is not `Admin` or `Manager`
		/// - with [NotInvestor](crate::pallet::Error::NotInvestor) when given AccountId is not investor
		/// - with [AccountDuplicate](crate::pallet::Error::AccountDuplicate) when input contains equal AccountIds
		/// - with [AccountAlreadyInvestor](crate::pallet::Error::AccountAlreadyInvestor) when input contains AccountId which is already an investor
		/// - with [KeyDuplicate](crate::pallet::Error::KeyDuplicate) when input contains two equal [InvestorKey]
		/// - with [InvestorKeyExists](crate::pallet::Error::InvestorKeyExists) when given [InvestorKey] is already used
		#[pallet::weight(<T as Config>::WeightInfo::set_investor_status())]
		pub fn set_investor_status(origin: OriginFor<T>, investor_account: T::AccountId, is_active: bool) -> DispatchResult {
			let who = Self::ensure_origin_is_admin_or_manager(origin)?;

			Self::_set_investor_status(&who, &investor_account, is_active)?;

			Ok(().into())
		}

		/// Change investor address
		/// 
		/// - `investor_account` - current investor's AccountId
		/// - `new_account` - new investor's AccountId
		/// 
		/// Fails:
		/// - with [PermissionDenied](crate::pallet::Error::PermissionDenied) when origin is not `Admin`
		/// - with [NotInvestor](crate::pallet::Error::NotInvestor) when given AccountId is not investor
		/// - with [SameAddress](crate::pallet::Error::SameAddress) when input contains equal AccountIds
		#[pallet::weight(<T as Config>::WeightInfo::change_investor_address())]
		pub fn change_investor_address(origin: OriginFor<T>, investor_account: T::AccountId, new_account: T::AccountId) -> DispatchResult {
			let who = Self::ensure_origin_is_admin(origin)?;

			Self::_change_investor_address(&who, &investor_account, &new_account)?;

			Ok(().into())
		}

		/// Change my address
		/// 

		/// - `new_account` - new investor's AccountId
		/// 
		/// Fails:
		/// - with [NotInvestor](crate::pallet::Error::NotInvestor) when RuntimeOrigin is not investor
		/// - with [SameAddress](crate::pallet::Error::SameAddress) when input contains equal AccountIds
		#[pallet::weight(<T as Config>::WeightInfo::change_my_address())]
		pub fn change_my_address(origin: OriginFor<T>, new_account: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::_change_investor_address(&who, &who, &new_account)?;

			Ok(().into())
		}
	}
}

// Interface for using the pallet in security assets pallets
impl<T: Config> WhitelistInterface<T> for Pallet<T> {
	fn ensure_origin_is_admin(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError> {
		if let Ok(account_id) = ensure_signed(origin) {
			if Self::is_admin(account_id.clone()) {
				return Ok(account_id);
			} else {
				fail!(Error::<T>::PermissionDenied)
			}
		}

		Err(DispatchError::BadOrigin)
	}

	fn ensure_origin_is_manager(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError> {
		if let Ok(account_id) = ensure_signed(origin) {
			if Self::is_manager(account_id.clone()) {
				return Ok(account_id);
			} else {
				fail!(Error::<T>::PermissionDenied)
			}
		}

		Err(DispatchError::BadOrigin)
	}

	fn is_admin(who: &T::AccountId) -> bool {
		Self::is_admin(who)
	}

    fn is_manager(who: &T::AccountId) -> bool {
		Self::is_manager(who)
	}

	fn is_active_investor(who: &T::AccountId) -> bool {
		if let Ok((investor, _)) = Self::_try_get_investor_and_key(who) {
			investor.is_active
		} else {
			false
		}
	}
}

// Internal ensure-like methods for checking origin privilege 
impl<T: Config> Pallet<T> {
	#[allow(dead_code)]
	fn ensure_origin_is_roles_root(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError> {
		if let Ok(_) = T::RolesRoot::ensure_origin(origin.clone()) {
			if let Ok(account_id) = ensure_signed(origin) {
				return Ok(account_id);
			}
		}

		Err(DispatchError::BadOrigin)
	}
}

// Roles control methods
impl<T: Config> Pallet<T> {
	fn _add_admin(new_admin: &T::AccountId) -> DispatchResult {
		ensure!(
			!Self::is_admin(new_admin),
			Error::<T>::WhitelistAdminExists
		);

		Admins::<T>::insert(new_admin, true);

		Self::deposit_event(Event::AddAdmin{new_admin: new_admin.clone()});

		Ok(())
	}

	fn _remove_admin(admin: &T::AccountId) -> DispatchResult {
		ensure!(
			Self::is_admin(admin),
			Error::<T>::NotWhitelistAdmin
		);

		Admins::<T>::remove(admin);

		Self::deposit_event(Event::RemoveAdmin{ admin: admin.clone()});

		Ok(())
	}

	fn _add_manager(who: &T::AccountId, new_manager: &T::AccountId) -> DispatchResult {
		ensure!(
			!Self::is_manager(new_manager),
			Error::<T>::WhitelistManagerExists
		);

		Managers::<T>::insert(new_manager, true);

		Self::deposit_event(Event::AddManager{ who: who.clone(), new_manager: new_manager.clone()});

		Ok(())
	}

	fn _remove_manager(who: &T::AccountId, manager: &T::AccountId) -> DispatchResult {
		ensure!(
			Self::is_manager(manager),
			Error::<T>::NotWhitelistManager
		);

		Managers::<T>::remove(manager);

		Self::deposit_event(Event::RemoveManager{ who: who.clone(), manager: manager.clone()});

		Ok(())
	}
}

// Investor related mathods
impl<T: Config> Pallet<T> {
	fn _verify_add_investors(investors: &Vec<(InvestorKey, Investor<T::AccountId>)>) -> DispatchResult {
		ensure!(
			!investors.is_empty() && investors.len() <= MAX_NEW_INVESTORS.into(),
			Error::<T>::InvalidInput
		);

		let (investors_keys, investors): (Vec<_>, Vec<_>) = investors.iter().cloned().unzip();
		let investors_accounts: Vec<_> = investors
			.iter()
			.map(|investor| investor.account.clone())
			.collect();

		ensure!(
			is_vec_unique(&investors_accounts),
			Error::<T>::AccountDuplicate
		);

		ensure!(
			investors_accounts.iter().all(|account| !KeysOfInvestors::<T>::contains_key(&account)),
			Error::<T>::AccountAlreadyInvestor
		);

		ensure!(
			is_vec_unique(&investors_keys),
			Error::<T>::KeyDuplicate
		);

		ensure!(
			investors_keys.iter().all(|key| !Investors::<T>::contains_key(&key)),
			Error::<T>::InvestorKeyExists
		);

		Ok(())
	}

	fn _add_investors(who: &T::AccountId, investors: Vec<(InvestorKey, Investor<T::AccountId>)>) -> DispatchResult {
		Self::_verify_add_investors(&investors)?;

		for (investor_key, investor) in investors {

			KeysOfInvestors::<T>::insert(investor.account.clone(), investor_key.clone());

			Investors::<T>::insert(investor_key.clone(), investor.clone());
			
			Self::deposit_event(Event::AddInvestor {
				who: who.clone(), 
				investor_key: investor_key,
				investor: investor
			});
		}

		Ok(())
	}

	fn _try_get_investor_and_key(investor_account: &T::AccountId) -> Result<(Investor<T::AccountId>, InvestorKey), DispatchError> {
		if let Some(key) = Self::investor_key(investor_account) {
			Ok((Self::investor(key).unwrap(), key))
		} else {
			fail!(Error::<T>::NotInvestor);
		}
	}

	fn _set_investor_status(who: &T::AccountId, investor_account: &T::AccountId, is_active: bool) -> DispatchResult {
		let (mut investor, key) = Self::_try_get_investor_and_key(investor_account)?;
		
		if is_active {
			ensure!(
				!investor.is_active,
				Error::<T>::AlreadyActive
			);
		} else {
			ensure!(
				investor.is_active,
				Error::<T>::AlreadyNotActive
			);
		}

		investor.is_active = is_active;

		Investors::<T>::insert(key, investor);

		Self::deposit_event(Event::InvestorStatusSet {
			who: who.clone(), 
			investor: investor_account.clone(), 
			is_active: is_active 
		});

		Ok(())
	}

	fn _change_investor_address(who: &T::AccountId, investor_account: &T::AccountId, new_account: &T::AccountId) -> DispatchResult {
		if investor_account == new_account {
			fail!(Error::<T>::SameAddress);
		}

		let (mut investor, key) = Self::_try_get_investor_and_key(investor_account)?;

		investor.account = new_account.clone();

		Investors::<T>::insert(key, investor);

		KeysOfInvestors::<T>::remove(investor_account);
		KeysOfInvestors::<T>::insert(new_account.clone(), key);

		Self::deposit_event(Event::InvestorAccountChanged {
			who: who.clone(), 
			old_account: investor_account.clone(), 
			new_account: new_account.clone() 
		});

		Ok(())
	}
}