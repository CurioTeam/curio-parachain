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

// Source https://github.com/UniqueNetwork/unique-chain
// Subject to the GPL-3.0 license.

// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use core::ops::{Deref, DerefMut};

use frame_support::{
    ensure,
    dispatch::{
        DispatchResult, DispatchError
    }
};

use collection_primitives::{
    CollectionId, Collection, SponsorshipState
};

use crate::{
    Config, Error, CollectionById,
    IsAdmin
};

/// Collection handle contains information about collection data and id.
/// Also provides functionality to count consumed gas.
///
/// CollectionHandle is used as a generic wrapper for collections of all types.
/// It allows to perform common operations and queries on any collection type,
/// both completely general for all, as well as their respective implementations of [`CommonCollectionOperations`].
#[must_use = "Should call submit_logs or save, otherwise some data will be lost for evm side"]
pub struct CollectionHandle<T: Config> {
	/// Collection id
	pub id: CollectionId,
	collection: Collection<T::AccountId>,
}

impl<T: Config> CollectionHandle<T> {
	/// Retrives collection data from storage and creates collection handle with default parameters.
	/// If collection not found return `None`
	pub fn new(id: CollectionId) -> Option<Self> {
		<CollectionById<T>>::get(id).map(|collection| Self {
			id,
			collection,
		})
	}

	/// Same as [`CollectionHandle::new`] but if collection not found [CollectionNotFound](Error::CollectionNotFound) returned.
	pub fn try_get(id: CollectionId) -> Result<Self, DispatchError> {
		Ok(Self::new(id).ok_or(<Error<T>>::CollectionNotFound)?)
	}
	
	/// Save collection to storage.
	pub fn save(&self) -> DispatchResult {
		<CollectionById<T>>::insert(self.id, &self.collection);
		Ok(())
	}

	/// Set collection sponsor.
	///
	/// Unique collections allows sponsoring for certain actions.
	/// This method allows you to set the sponsor of the collection.
	/// In order for sponsorship to become active, it must be confirmed through [`Self::confirm_sponsorship`].
	pub fn set_sponsor(&mut self, sponsor: T::AccountId) -> DispatchResult {
		ensure!(
			self.collection.sponsorship.pending_sponsor() != Some(&sponsor),
			Error::<T>::AccountAlreadySponsor
		);
		self.collection.sponsorship = SponsorshipState::Unconfirmed(sponsor.clone());
		Ok(())
	}

	/// Confirm sponsorship
	///
	/// In order for the sponsorship to become active, the user set as the sponsor must confirm their participation.
	/// Before confirming sponsorship, the user must be specified as the sponsor of the collection via [`Self::set_sponsor`].
	pub fn confirm_sponsorship(&mut self, sender: &T::AccountId) -> DispatchResult {
		ensure!(
			self.collection.sponsorship == SponsorshipState::Unconfirmed(sender.clone()),
			Error::<T>::NotUnconfirmedSponsor
		);
		self.collection.sponsorship = SponsorshipState::Confirmed(sender.clone());
		Ok(())
	}

	/// Remove collection sponsor.
	pub fn remove_sponsor(&mut self) -> DispatchResult {
		ensure!(
			self.collection.sponsorship != SponsorshipState::Disabled,
			Error::<T>::SponsorshipAlreadyDisabled
		);
		self.collection.sponsorship = SponsorshipState::Disabled;
		Ok(())
	}
}

impl<T: Config> Deref for CollectionHandle<T> {
	type Target = Collection<T::AccountId>;

	fn deref(&self) -> &Self::Target {
		&self.collection
	}
}

impl<T: Config> DerefMut for CollectionHandle<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.collection
	}
}

impl<T: Config> CollectionHandle<T> {
	/// Checks if the `user` is the owner of the collection.
	// LOG: CrossAccountId -> AccountId
	pub fn check_is_owner(&self, user: &T::AccountId) -> DispatchResult {
		ensure!(*user == self.owner, <Error<T>>::NoPermission);
		Ok(())
	}

	/// Returns **true** if the `user` is the owner or administrator of the collection.
	// LOG: CrossAccountId -> AccountId
	pub fn is_owner_or_admin(&self, user: &T::AccountId) -> bool {
		*user == self.owner || <IsAdmin<T>>::get((self.id, user))
	}

	/// Checks if the `user` is the owner or administrator of the collection.
	// LOG: CrossAccountId -> AccountId
	pub fn check_is_owner_or_admin(&self, user: &T::AccountId) -> DispatchResult {
		ensure!(self.is_owner_or_admin(user), <Error<T>>::NoPermission);
		Ok(())
	}

	/// Return **true** if `user` was not allowed to have tokens, and he can ignore such restrictions.
	// LOG: CrossAccountId -> AccountId
	pub fn ignores_allowance(&self, user: &T::AccountId) -> bool {
		self.limits.owner_can_transfer() && self.is_owner_or_admin(user)
	}

	/// Return **true** if `user` does not have enough token parts, and he can ignore such restrictions.
	// LOG: CrossAccountId -> AccountId
	pub fn ignores_owned_amount(&self, user: &T::AccountId) -> bool {
		self.limits.owner_can_transfer() && self.is_owner_or_admin(user)
	}
}