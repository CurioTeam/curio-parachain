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

//! # Common pallet
//!
//! The Common pallet provides functionality for handling collections.
//!
//! ## Overview
//!
//! The Common pallet provides an interface for common collection operations for different collection types
//! (see [CommonCollectionOperations]), as well as a generic dispatcher for these, see [dispatch] module.
//!
//! The Common pallet provides functions for:
//!
//! - Setting and approving collection sponsor.
//! - Get\set\delete allow list.
//! - Get\set\delete collection properties.
//! - Get\set\delete collection property permissions.
//! - Get\set\delete token property permissions.
//! - Get\set\delete collection administrators.
//! - Checking access permissions.
//!
//! ### Terminology
//! **Collection sponsor** - For the collection, you can set a sponsor, at whose expense it will
//! be possible to mint tokens.
//!
//! **Allow list** - List of users who have the right to minting tokens.
//!
//! **Collection properties** - Collection properties are simply key-value stores where various
//! metadata can be placed.
//!
//! **Permissions on token properties** - For each property in the token can be set permission
//! to change, see [`PropertyPermission`].
//!
//! **Collection administrator** - For a collection, you can set administrators who have the right
//! to most actions on the collection.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use sp_std::vec::Vec;
use frame_support::{
	dispatch::{DispatchResult},
	ensure,
	traits::{Imbalance, Get, Currency, WithdrawReasons, ExistenceRequirement},
	transactional,
};

use collection_primitives::{
	COLLECTION_NUMBER_LIMIT,
	Collection,
	RpcCollection,
	CollectionId,
	MAX_TOKEN_PREFIX_LENGTH,
	COLLECTION_ADMINS_LIMIT,
	TokenChild,
	CollectionStats,
	MAX_TOKEN_OWNERSHIP,
	CollectionMode,
	NFT_SPONSOR_TRANSFER_TIMEOUT,
	FUNGIBLE_SPONSOR_TRANSFER_TIMEOUT,
	REFUNGIBLE_SPONSOR_TRANSFER_TIMEOUT,
	MAX_SPONSOR_TIMEOUT,
	CUSTOM_DATA_LIMIT,
	CollectionLimits,
	CreateCollectionData,
	SponsorshipState,
	SponsoringRateLimit,
	PhantomType,
	Property,
	Properties,
	PropertiesPermissionMap,
	PropertyKey,
	PropertyValue,
	PropertyPermission,
	PropertiesError,
	PropertyKeyPermission,
	TokenData,
	TrySetProperty,
	PropertyScope,
};

use pallet_whitelist::traits::WhitelistInterface;

pub use handle::CollectionHandle;

pub use pallet::*;
use sp_runtime::{ArithmeticError, DispatchError};

pub mod mock;
pub mod tests;
pub mod handle;
pub mod collection_initializer;

macro_rules! limit_default {
	($old:ident, $new:ident, $($field:ident $(($arg:expr))? => $check:expr),* $(,)?) => {{
		$(
			if let Some($new) = $new.$field {
				let $old = $old.$field($($arg)?);
				let _ = $new;
				let _ = $old;
				$check
			} else {
				$new.$field = $old.$field
			}
		)*
	}};
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{Blake2_128Concat, pallet_prelude::*, storage::Key, traits::StorageVersion};
	use frame_system::pallet_prelude::*;
	use frame_support::traits::Currency;
	use collection_primitives::{TokenId};
	use scale_info::TypeInfo;

	// LOG: CrossAccountId -> AccountId
	// LOG: EvmTokenAddressMapping deleted
	// LOG: CrossTokenAddressMapping deleted
	// LOG: Whitelist interface added
	#[pallet::config]
	pub trait Config:
		frame_system::Config +
		pallet_whitelist::Config +
		TypeInfo
	{
		/// Events compatible with [`frame_system::Config::RuntimeEvent`].
		type RuntimeEvent: IsType<<Self as frame_system::Config>::RuntimeEvent> + From<Event<Self>>;

		/// Handler of accounts and payment.
		type Currency: Currency<Self::AccountId>;

		/// Set price to create a collection.
		#[pallet::constant]
		type CollectionCreationPrice: Get<
			<<Self as Config>::Currency as Currency<Self::AccountId>>::Balance,
		>;

		/// Account which holds the chain's treasury.
		type TreasuryAccountId: Get<Self::AccountId>;

		/// Interface for whitelist pallet
		type Whitelist: WhitelistInterface<Self>;
	}

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::extra_constants]
	impl<T: Config> Pallet<T> {
		/// Maximum admins per collection.
		pub fn collection_admins_limit() -> u32 {
			COLLECTION_ADMINS_LIMIT
		}
	}

	
	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]
	pub enum Event<T: Config> {
		/// New collection was created
		CollectionCreated(
			/// Globally unique identifier of newly created collection.
			CollectionId,
			/// [`CollectionMode`] converted into _u8_.
			u8,
			/// Collection owner.
			T::AccountId,
		),

		/// New collection was destroyed
		CollectionDestroyed(
			/// Globally unique identifier of collection.
			CollectionId,
		),

		// LOG: CrossAccountId -> AccountId
		/// New item was created.
		ItemCreated(
			/// Id of the collection where item was created.
			CollectionId,
			/// Id of an item. Unique within the collection.
			TokenId,
			/// Owner of newly created item
			T::AccountId,
			/// Always 1 for NFT
			u128,
		),

		// LOG: CrossAccountId -> AccountId
		/// Collection item was burned.
		ItemDestroyed(
			/// Id of the collection where item was destroyed.
			CollectionId,
			/// Identifier of burned NFT.
			TokenId,
			/// Which user has destroyed its tokens.
			T::AccountId,
			/// Amount of token pieces destroed. Always 1 for NFT.
			u128,
		),

		// LOG: CrossAccountId -> AccountId
		/// Item was transferred
		Transfer(
			/// Id of collection to which item is belong.
			CollectionId,
			/// Id of an item.
			TokenId,
			/// Original owner of item.
			T::AccountId,
			/// New owner of item.
			T::AccountId,
			/// Amount of token pieces transfered. Always 1 for NFT.
			u128,
		),

		// LOG: CrossAccountId -> AccountId
		/// Amount pieces of token owned by `sender` was approved for `spender`.
		Approved(
			/// Id of collection to which item is belong.
			CollectionId,
			/// Id of an item.
			TokenId,
			/// Original owner of item.
			T::AccountId,
			/// Id for which the approval was granted.
			T::AccountId,
			/// Amount of token pieces transfered. Always 1 for NFT.
			u128,
		),

		/// The colletion property has been added or edited.
		CollectionPropertySet(
			/// Id of collection to which property has been set.
			CollectionId,
			/// The property that was set.
			PropertyKey,
		),

		/// The property has been deleted.
		CollectionPropertyDeleted(
			/// Id of collection to which property has been deleted.
			CollectionId,
			/// The property that was deleted.
			PropertyKey,
		),

		/// The token property has been added or edited.
		TokenPropertySet(
			/// Identifier of the collection whose token has the property set.
			CollectionId,
			/// The token for which the property was set.
			TokenId,
			/// The property that was set.
			PropertyKey,
		),

		/// The token property has been deleted.
		TokenPropertyDeleted(
			/// Identifier of the collection whose token has the property deleted.
			CollectionId,
			/// The token for which the property was deleted.
			TokenId,
			/// The property that was deleted.
			PropertyKey,
		),

		/// The token property permission of a collection has been set.
		PropertyPermissionSet(
			/// ID of collection to which property permission has been set.
			CollectionId,
			/// The property permission that was set.
			PropertyKey,
		),

		/// User's admin status has been changed
		AdminToggled(
			/// User which status has been changed
			T::AccountId,
			/// Current status (true = admin, false = not admin)
			bool,
		),

		/// The sponsor of a collection has been set.
		SponsorSet(
			/// ID of collection to which sponsor has been set.
			CollectionId,
			/// Sponsor of a collection.
			T::AccountId
		),

		/// The sponsor of a collection has been confirmed.
		SponsorhipConfirmed(
			/// ID of collection to which sponsor has been confirmed.
			CollectionId,
			/// Who confirmed sponsorship.
			T::AccountId
		),

		/// The sponsor of a collection has been removed.
		SponsorshipRemoved(
			/// ID of collection from which sponsor has been removed.
			CollectionId
		)
	}

	#[pallet::error]
	pub enum Error<T> {
		/// This collection does not exist.
		CollectionNotFound,
		/// Sender parameter and item owner must be equal.
		MustBeTokenOwner,
		/// No permission to perform action
		NoPermission,
		/// Destroying only empty collections is allowed
		CantDestroyNotEmptyCollection,
		/// Collection is not in mint mode.
		PublicMintingNotAllowed,
		/// Address is not in allow list.
		AddressNotInAllowlist,

		/// Collection name can not be longer than 63 char.
		CollectionNameLimitExceeded,
		/// Collection description can not be longer than 255 char.
		CollectionDescriptionLimitExceeded,
		/// Token prefix can not be longer than 15 char.
		CollectionTokenPrefixLimitExceeded,
		/// Total collections bound exceeded.
		TotalCollectionsLimitExceeded,
		/// Exceeded max admin count
		CollectionAdminCountExceeded,
		/// Collection limit bounds per collection exceeded
		CollectionLimitBoundsExceeded,
		/// Tried to enable permissions which are only permitted to be disabled
		OwnerPermissionsCantBeReverted,
		/// Collection settings not allowing items transferring
		TransfersDisabled,
		/// Account token limit exceeded per collection
		AccountTokenLimitExceeded,
		/// Collection token limit exceeded
		CollectionTokenLimitExceeded,
		/// Metadata flag frozen
		MetadataFlagFrozen,

		/// Item does not exist
		TokenNotFound,
		/// Item is balance not enough
		TokenValueTooLow,
		/// Requested value is more than the approved
		ApprovedValueTooLow,
		/// Tried to approve more than owned
		CantApproveMoreThanOwned,

		/// Can't transfer tokens to ethereum zero address
		AddressIsZero,

		/// The operation is not supported
		UnsupportedOperation,

		/// Insufficient funds to perform an action
		NotSufficientFounds,

		/// User does not satisfy the nesting rule
		UserIsNotAllowedToNest,
		/// Only tokens from specific collections may nest tokens under this one
		SourceCollectionIsNotAllowedToNest,

		/// Tried to store more data than allowed in collection field
		CollectionFieldSizeExceeded,

		/// Property permission set that property is unmutable 
		UnmutableProperty,

		/// Tried to store more property data than allowed
		NoSpaceForProperty,

		/// Tried to store more property keys than allowed
		PropertyLimitReached,

		/// Property key is too long
		PropertyKeyIsTooLong,

		/// Only ASCII letters, digits, and symbols `_`, `-`, and `.` are allowed
		InvalidCharacterInPropertyKey,

		/// Empty property keys are forbidden
		EmptyPropertyKey,

		/// Tried to access an external collection with an internal API
		CollectionIsExternal,

		/// Tried to access an internal collection with an external API
		CollectionIsInternal,

		/// The same sponsor is already set
		AccountAlreadySponsor,

		/// Given sponsor account (call sender) is not unconfirmed one
		NotUnconfirmedSponsor,

		/// Sponsorship already disabled
		SponsorshipAlreadyDisabled,
	}

	/// Storage of the count of created collections. Essentially contains the last collection ID.
	#[pallet::storage]
	pub type CreatedCollectionCount<T> = StorageValue<Value = CollectionId, QueryKind = ValueQuery>;

	/// Storage of the count of deleted collections.
	#[pallet::storage]
	pub type DestroyedCollectionCount<T> =
		StorageValue<Value = CollectionId, QueryKind = ValueQuery>;

	/// Storage of collection info.
	#[pallet::storage]
	pub type CollectionById<T> = StorageMap<
		Hasher = Blake2_128Concat,
		Key = CollectionId,
		Value = Collection<<T as frame_system::Config>::AccountId>,
		QueryKind = OptionQuery,
	>;

	/// Storage of collection properties.
	#[pallet::storage]
	#[pallet::getter(fn collection_properties)]
	pub type CollectionProperties<T> = StorageMap<
		Hasher = Blake2_128Concat,
		Key = CollectionId,
		Value = Properties,
		QueryKind = ValueQuery,
		OnEmpty = collection_primitives::CollectionProperties,
	>;

	/// Storage of property permissions of a collection.
	#[pallet::storage]
	#[pallet::getter(fn property_permissions)]
	pub type PropertyPermissions<T> = StorageMap<
		Hasher = Blake2_128Concat,
		Key = CollectionId,
		Value = PropertiesPermissionMap,
		QueryKind = ValueQuery,
	>;

	/// Storage of the amount of collection admins.
	#[pallet::storage]
	pub type AdminAmount<T> = StorageMap<
		Hasher = Blake2_128Concat,
		Key = CollectionId,
		Value = u32,
		QueryKind = ValueQuery,
	>;

	// LOG: CrossAccountId -> AccountId
	/// List of collection admins.
	#[pallet::storage]
	pub type IsAdmin<T: Config> = StorageNMap<
		Key = (
			Key<Blake2_128Concat, CollectionId>,
			Key<Blake2_128Concat, T::AccountId>,
		),
		Value = bool,
		QueryKind = ValueQuery,
	>;

	// LOG: CrossAccountId -> AccountId
	/// Not used by code, exists only to provide some types to metadata.
	#[pallet::storage]
	pub type DummyStorageValue<T: Config> = StorageValue<
		Value = (
			CollectionStats,
			CollectionId,
			TokenId,
			TokenChild,
			PhantomType<(
				TokenData<T::AccountId>,
				RpcCollection<T::AccountId>
			)>,
		),
		QueryKind = OptionQuery,
	>;

	/// (Collection id (controlled?2), who created (real))
	/// TODO: Off chain worker should remove from this map when collection gets removed
	#[pallet::storage]
	#[pallet::getter(fn create_item_busket)]
	pub type CreateItemBasket<T: Config> = StorageMap<
		Hasher = Blake2_128Concat,
		Key = (CollectionId, T::AccountId),
		Value = T::BlockNumber,
		QueryKind = OptionQuery,
	>;

	/// Last sponsoring of token property setting // todo:doc rephrase this and the following
	#[pallet::storage]
	#[pallet::getter(fn token_property_basket)]
	pub type TokenPropertyBasket<T: Config> = StorageDoubleMap<
		Hasher1 = Blake2_128Concat,
		Key1 = CollectionId,
		Hasher2 = Blake2_128Concat,
		Key2 = TokenId,
		Value = T::BlockNumber,
		QueryKind = OptionQuery,
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
}

impl<T: Config> Pallet<T> {
	// LOG: CrossAccountId -> AccountId
	// LOG: EVM event deleted
	// LOG: WL admin check added
	/// Create new collection.
	///
	/// * `owner` - The owner of the collection.
	/// * `data` - Description of the created collection.
	/// * `flags` - Extra flags to store.
	#[transactional]
	pub fn init_collection(
		owner: T::AccountId,
		payer: T::AccountId,
		data: CreateCollectionData<T::AccountId>,
	) -> Result<CollectionId, DispatchError> {
		
		ensure!(
			T::Whitelist::is_admin(&owner),
			Error::<T>::NoPermission
		);

		ensure!(
			data.token_prefix.len() <= MAX_TOKEN_PREFIX_LENGTH as usize,
			Error::<T>::CollectionTokenPrefixLimitExceeded
		);

		let created_count = <CreatedCollectionCount<T>>::get()
			.0
			.checked_add(1)
			.ok_or(ArithmeticError::Overflow)?;
		let destroyed_count = <DestroyedCollectionCount<T>>::get().0;
		let id = CollectionId(created_count);

		// bound Total number of collections
		ensure!(
			created_count - destroyed_count <= COLLECTION_NUMBER_LIMIT,
			<Error<T>>::TotalCollectionsLimitExceeded
		);

		// =========

		// Take a (non-refundable) deposit of collection creation
		{
			let mut imbalance =
				<<<T as Config>::Currency as Currency<T::AccountId>>::PositiveImbalance>::zero();
			imbalance.subsume(
				<<T as Config>::Currency as Currency<T::AccountId>>::deposit_creating(
					&T::TreasuryAccountId::get(),
					T::CollectionCreationPrice::get(),
				),
			);
			<T as Config>::Currency::settle(
				&payer,
				imbalance,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::KeepAlive,
			)
			.map_err(|_| Error::<T>::NotSufficientFounds)?;
		}

		let collection = Collection {
			owner: owner.clone(),
			name: data.name,
			mode: data.mode.clone(),
			description: data.description,
			token_prefix: data.token_prefix,
			sponsorship: data
				.pending_sponsor
				.map(SponsorshipState::Unconfirmed)
				.unwrap_or_default(),
			limits: data
				.limits
				.map(|limits| Self::clamp_limits(data.mode.clone(), &Default::default(), limits))
				.unwrap_or_else(|| Ok(CollectionLimits::default()))?,
		};

		let mut collection_properties = collection_primitives::CollectionProperties::get();
		collection_properties
			.try_set_from_iter(data.properties.into_iter())
			.map_err(<Error<T>>::from)?;

		CollectionProperties::<T>::insert(id, collection_properties);

		let mut props_permissions = PropertiesPermissionMap::new();
		props_permissions
			.try_set_from_iter(data.property_permissions.into_iter())
			.map_err(<Error<T>>::from)?;

		PropertyPermissions::<T>::insert(id, props_permissions);

		<CreatedCollectionCount<T>>::put(created_count);
		<Pallet<T>>::deposit_event(Event::CollectionCreated(
			id,
			data.mode.id(),
			owner.clone(),
		));

		<CollectionById<T>>::insert(id, collection);
		Ok(id)
	}

	// LOG: CrossAccountId -> AccountId
	// LOG: EVM event deleted
	/// Destroy collection.
	///
	/// * `collection` - Collection handler.
	/// * `sender` - The owner or administrator of the collection.
	pub fn destroy_collection(
		collection: CollectionHandle<T>,
		sender: &T::AccountId,
	) -> DispatchResult {
		ensure!(
			collection.limits.owner_can_destroy(),
			<Error<T>>::NoPermission,
		);
		collection.check_is_owner(sender)?;

		let destroyed_collections = <DestroyedCollectionCount<T>>::get()
			.0
			.checked_add(1)
			.ok_or(ArithmeticError::Overflow)?;

		// =========

		<DestroyedCollectionCount<T>>::put(destroyed_collections);
		<CollectionById<T>>::remove(collection.id);
		<AdminAmount<T>>::remove(collection.id);
		let _ = <IsAdmin<T>>::clear_prefix((collection.id,), u32::MAX, None);
		<CollectionProperties<T>>::remove(collection.id);

		<Pallet<T>>::deposit_event(Event::CollectionDestroyed(collection.id));

		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	/// Set collection property.
	///
	/// * `collection` - Collection handler.
	/// * `sender` - The owner or administrator of the collection.
	/// * `property` - The property to set.
	pub fn set_collection_property(
		collection: &CollectionHandle<T>,
		sender: &T::AccountId,
		property: Property,
	) -> DispatchResult {
		collection.check_is_owner_or_admin(sender)?;

		Self::_check_property_is_mutable(collection, &property.key)?;

		CollectionProperties::<T>::try_mutate(collection.id, |properties| {
			let property = property.clone();
			properties.try_set(property.key, property.value)
		})
		.map_err(<Error<T>>::from)?;

		Self::deposit_event(Event::CollectionPropertySet(collection.id, property.key));

		Ok(())
	}

	/// Set scouped collection property.
	///
	/// * `collection_id` - ID of the collection for which the property is being set.
	/// * `scope` - Property scope.
	/// * `property` - The property to set.
	pub fn set_scoped_collection_property(
		collection_id: CollectionId,
		scope: PropertyScope,
		property: Property,
	) -> DispatchResult {
		CollectionProperties::<T>::try_mutate(collection_id, |properties| {
			properties.try_scoped_set(scope, property.key, property.value)
		})
		.map_err(<Error<T>>::from)?;

		Ok(())
	}

	/// Set scouped collection properties.
	///
	/// * `collection_id` - ID of the collection for which the properties is being set.
	/// * `scope` - Property scope.
	/// * `properties` - The properties to set.
	#[transactional]
	pub fn set_scoped_collection_properties(
		collection_id: CollectionId,
		scope: PropertyScope,
		properties: impl Iterator<Item = Property>,
	) -> DispatchResult {
		CollectionProperties::<T>::try_mutate(collection_id, |stored_properties| {
			stored_properties.try_scoped_set_from_iter(scope, properties)
		})
		.map_err(<Error<T>>::from)?;

		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	/// Set collection properties.
	///
	/// * `collection` - Collection handler.
	/// * `sender` - The owner or administrator of the collection.
	/// * `properties` - The properties to set.
	#[transactional]
	pub fn set_collection_properties(
		collection: &CollectionHandle<T>,
		sender: &T::AccountId,
		properties: Vec<Property>,
	) -> DispatchResult {
		for property in properties {
			Self::set_collection_property(collection, sender, property)?;
		}

		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	/// Delete collection property.
	///
	/// * `collection` - Collection handler.
	/// * `sender` - The owner or administrator of the collection.
	/// * `property` - The property to delete.
	pub fn delete_collection_property(
		collection: &CollectionHandle<T>,
		sender: &T::AccountId,
		property_key: PropertyKey,
	) -> DispatchResult {
		collection.check_is_owner_or_admin(sender)?;

		CollectionProperties::<T>::try_mutate(collection.id, |properties| {
			properties.remove(&property_key)
		})
		.map_err(<Error<T>>::from)?;

		Self::deposit_event(Event::CollectionPropertyDeleted(
			collection.id,
			property_key,
		));

		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	/// Delete collection properties.
	///
	/// * `collection` - Collection handler.
	/// * `sender` - The owner or administrator of the collection.
	/// * `properties` - The properties to delete.
	#[transactional]
	pub fn delete_collection_properties(
		collection: &CollectionHandle<T>,
		sender: &T::AccountId,
		property_keys: Vec<PropertyKey>,
	) -> DispatchResult {
		for key in property_keys {
			Self::delete_collection_property(collection, sender, key)?;
		}

		Ok(())
	}

	/// Set collection propetry permission without any checks.
	///
	/// Used for migrations.
	///
	/// * `collection` - Collection handler.
	/// * `property_permissions` - Property permissions.
	pub fn set_property_permission_unchecked(
		collection: CollectionId,
		property_permission: PropertyKeyPermission,
	) -> DispatchResult {
		<PropertyPermissions<T>>::try_mutate(collection, |permissions| {
			permissions.try_set(property_permission.key, property_permission.permission)
		})
		.map_err(<Error<T>>::from)?;
		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	/// Set collection property permission.
	///
	/// * `collection` - Collection handler.
	/// * `sender` - The owner or administrator of the collection.
	/// * `property_permission` - Property permission.
	pub fn set_property_permission(
		collection: &CollectionHandle<T>,
		sender: &T::AccountId,
		property_permission: PropertyKeyPermission,
	) -> DispatchResult {
		Self::set_scoped_property_permission(
			collection,
			sender,
			PropertyScope::None,
			property_permission,
		)
	}

	// LOG: CrossAccountId -> AccountId
	/// Set collection property permission with scope.
	///
	/// * `collection` - Collection handler.
	/// * `sender` - The owner or administrator of the collection.
	/// * `scope` - Property scope.
	/// * `property_permission` - Property permission.
	pub fn set_scoped_property_permission(
		collection: &CollectionHandle<T>,
		sender: &T::AccountId,
		scope: PropertyScope,
		property_permission: PropertyKeyPermission,
	) -> DispatchResult {
		collection.check_is_owner(sender)?;

		let all_permissions = PropertyPermissions::<T>::get(collection.id);
		let current_permission = all_permissions.get(&property_permission.key);
		if matches![
			current_permission,
			Some(PropertyPermission { mutable: false, .. })
		] {
			return Err(<Error<T>>::NoPermission.into());
		}

		PropertyPermissions::<T>::try_mutate(collection.id, |permissions| {
			let property_permission = property_permission.clone();
			permissions.try_scoped_set(
				scope,
				property_permission.key,
				property_permission.permission,
			)
		})
		.map_err(<Error<T>>::from)?;

		Self::deposit_event(Event::PropertyPermissionSet(
			collection.id,
			property_permission.key,
		));

		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	// LOG: Only WL admins could be collection admins
	/// Toggle `user` participation in the `collection`'s admin list.
	pub fn toggle_admin(
		collection: &CollectionHandle<T>,
		sender: &T::AccountId,
		user: &T::AccountId,
		admin: bool,
	) -> DispatchResult {
		collection.check_is_owner(sender)?;

		T::Whitelist::ensure_admin(user)?;

		let was_admin = <IsAdmin<T>>::get((collection.id, user));
		if was_admin == admin {
			return Ok(());
		}
		let amount = <AdminAmount<T>>::get(collection.id);

		if admin {
			let amount = amount
				.checked_add(1)
				.ok_or(<Error<T>>::CollectionAdminCountExceeded)?;
			ensure!(
				amount <= Self::collection_admins_limit(),
				<Error<T>>::CollectionAdminCountExceeded,
			);

			// =========

			<AdminAmount<T>>::insert(collection.id, amount);
			<IsAdmin<T>>::insert((collection.id, user), true);
		} else {
			<AdminAmount<T>>::insert(collection.id, amount.saturating_sub(1));
			<IsAdmin<T>>::remove((collection.id, user));
		}

		Self::deposit_event(Event::AdminToggled(
			user.clone(),
			admin,
		));

		Ok(())
	}

	fn _check_property_is_mutable(collection: &CollectionHandle<T>, key: &PropertyKey) -> DispatchResult {
		let permission = Self::property_permissions(collection.id)
			.get(&key)
			.cloned()
			.unwrap_or_else(PropertyPermission::unmutable);

		if let PropertyPermission { mutable: false, .. } = permission {
			Err(<Error<T>>::UnmutableProperty.into())
		} else {
			Ok(())
		}
	}

	/// Get collection property.
	pub fn get_collection_property(
		collection_id: CollectionId,
		key: &PropertyKey,
	) -> Option<PropertyValue> {
		Self::collection_properties(collection_id).get(key).cloned()
	}

	/// Get collection property permission.
	pub fn get_collection_property_permission(
		collection_id: CollectionId,
		key: &PropertyKey,
	) -> Option<PropertyPermission> {
		Self::property_permissions(collection_id).get(key).cloned()
	}

	/// Convert byte vector to property key vector.
	pub fn bytes_keys_to_property_keys(
		keys: Vec<Vec<u8>>,
	) -> Result<Vec<PropertyKey>, DispatchError> {
		keys.into_iter()
			.map(|key| -> Result<PropertyKey, DispatchError> {
				key.try_into()
					.map_err(|_| <Error<T>>::PropertyKeyIsTooLong.into())
			})
			.collect::<Result<Vec<PropertyKey>, DispatchError>>()
	}

	/// Get properties according to given keys.
	pub fn filter_collection_properties(
		collection_id: CollectionId,
		keys: Option<Vec<PropertyKey>>,
	) -> Result<Vec<Property>, DispatchError> {
		let properties = Self::collection_properties(collection_id);

		let properties = keys
			.map(|keys| {
				keys.into_iter()
					.filter_map(|key| {
						properties.get(&key).map(|value| Property {
							key,
							value: value.clone(),
						})
					})
					.collect()
			})
			.unwrap_or_else(|| {
				properties
					.into_iter()
					.map(|(key, value)| Property { key, value })
					.collect()
			});

		Ok(properties)
	}

	/// Get property permissions according to given keys.
	pub fn filter_property_permissions(
		collection_id: CollectionId,
		keys: Option<Vec<PropertyKey>>,
	) -> Result<Vec<PropertyKeyPermission>, DispatchError> {
		let permissions = Self::property_permissions(collection_id);

		let key_permissions = keys
			.map(|keys| {
				keys.into_iter()
					.filter_map(|key| {
						permissions
							.get(&key)
							.map(|permission| PropertyKeyPermission {
								key,
								permission: permission.clone(),
							})
					})
					.collect()
			})
			.unwrap_or_else(|| {
				permissions
					.into_iter()
					.map(|(key, permission)| PropertyKeyPermission { key, permission })
					.collect()
			});

		Ok(key_permissions)
	}

	/// Merge set fields from `new_limit` to `old_limit`.
	pub fn clamp_limits(
		mode: CollectionMode,
		old_limit: &CollectionLimits,
		mut new_limit: CollectionLimits,
	) -> Result<CollectionLimits, DispatchError> {
		let limits = old_limit;
		limit_default!(old_limit, new_limit,
			account_token_ownership_limit => ensure!(
				new_limit <= MAX_TOKEN_OWNERSHIP,
				<Error<T>>::CollectionLimitBoundsExceeded,
			),
			sponsored_data_size => ensure!(
				new_limit <= CUSTOM_DATA_LIMIT,
				<Error<T>>::CollectionLimitBoundsExceeded,
			),

			sponsored_data_rate_limit => {},
			token_limit => ensure!(
				old_limit >= new_limit && new_limit > 0,
				<Error<T>>::CollectionTokenLimitExceeded
			),

			sponsor_transfer_timeout(match mode {
				CollectionMode::NFT => NFT_SPONSOR_TRANSFER_TIMEOUT,
				CollectionMode::Fungible(_) => FUNGIBLE_SPONSOR_TRANSFER_TIMEOUT,
				CollectionMode::ReFungible => REFUNGIBLE_SPONSOR_TRANSFER_TIMEOUT,
			}) => ensure!(
				new_limit <= MAX_SPONSOR_TIMEOUT,
				<Error<T>>::CollectionLimitBoundsExceeded,
			),
			sponsor_approve_timeout => {},
			owner_can_transfer => ensure!(
				!limits.owner_can_transfer_instaled() ||
				old_limit || !new_limit,
				<Error<T>>::OwnerPermissionsCantBeReverted,
			),
			owner_can_destroy => ensure!(
				old_limit || !new_limit,
				<Error<T>>::OwnerPermissionsCantBeReverted,
			),
			transfers_enabled => {},
		);
		Ok(new_limit)
	}

	// LOG: CrossAccountId -> AccountId
	pub fn set_sponsor(
		collection_id: CollectionId,
		sender: &T::AccountId,
		sponsor: T::AccountId
	) -> DispatchResult {
		let mut collection = CollectionHandle::<T>::try_get(collection_id)?;

		collection.check_is_owner(sender)?;

		collection.set_sponsor(sponsor.clone())?;

		Self::deposit_event(Event::SponsorSet(collection.id, sponsor));

		collection.save()?;
		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	pub fn confirm_sponsorship(
		collection_id: CollectionId,
		sender: &T::AccountId
	) -> DispatchResult {
		let mut collection = CollectionHandle::<T>::try_get(collection_id)?;

		collection.confirm_sponsorship(sender)?;

		Self::deposit_event(Event::SponsorhipConfirmed(collection.id, sender.clone()));

		collection.save()?;
		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	pub fn remove_sponsor(
		collection_id: CollectionId,
		sender: &T::AccountId
	) -> DispatchResult {
		let mut collection = CollectionHandle::<T>::try_get(collection_id)?;

		collection.check_is_owner(sender)?;

		collection.remove_sponsor()?;

		Self::deposit_event(Event::SponsorshipRemoved(collection.id));

		collection.save()?;
		Ok(())
	}
}

/// Indicates unsupported methods by returning [Error::UnsupportedOperation].
#[macro_export]
macro_rules! unsupported {
	($runtime:path) => {
		Err($crate::Error::<$runtime>::UnsupportedOperation.into())
	};
}

impl<T: Config> From<PropertiesError> for Error<T> {
	fn from(error: PropertiesError) -> Self {
		match error {
			PropertiesError::NoSpaceForProperty => Self::NoSpaceForProperty,
			PropertiesError::PropertyLimitReached => Self::PropertyLimitReached,
			PropertiesError::InvalidCharacterInPropertyKey => Self::InvalidCharacterInPropertyKey,
			PropertiesError::PropertyKeyIsTooLong => Self::PropertyKeyIsTooLong,
			PropertiesError::EmptyPropertyKey => Self::EmptyPropertyKey,
		}
	}
}

// LOG: ensure_correct_receiver method deleted
impl<T: Config> Pallet<T> {
	// LOG: CrossAccountId -> AccountId
	/// Get a vector of collection admins.
	pub fn adminlist(collection: CollectionId) -> Vec<T::AccountId> {
		<IsAdmin<T>>::iter_prefix((collection,))
			.map(|(a, _)| a)
			.collect()
	}

	/// Get statistics of collections.
	pub fn collection_stats() -> CollectionStats {
		let created = <CreatedCollectionCount<T>>::get();
		let destroyed = <DestroyedCollectionCount<T>>::get();
		CollectionStats {
			created: created.0,
			destroyed: destroyed.0,
			alive: created.0 - destroyed.0,
		}
	}

	/// Get the effective limits for the collection.
	pub fn effective_collection_limits(collection: CollectionId) -> Option<CollectionLimits> {
		let collection = <CollectionById<T>>::get(collection)?;
		let limits = collection.limits;
		let effective_limits = CollectionLimits {
			account_token_ownership_limit: Some(limits.account_token_ownership_limit()),
			sponsored_data_size: Some(limits.sponsored_data_size()),
			sponsored_data_rate_limit: Some(
				limits
					.sponsored_data_rate_limit
					.unwrap_or(SponsoringRateLimit::SponsoringDisabled),
			),
			token_limit: Some(limits.token_limit()),
			sponsor_transfer_timeout: Some(limits.sponsor_transfer_timeout(
				match collection.mode {
					CollectionMode::NFT => NFT_SPONSOR_TRANSFER_TIMEOUT,
					CollectionMode::Fungible(_) => FUNGIBLE_SPONSOR_TRANSFER_TIMEOUT,
					CollectionMode::ReFungible => REFUNGIBLE_SPONSOR_TRANSFER_TIMEOUT,
				},
			)),
			sponsor_approve_timeout: Some(limits.sponsor_approve_timeout()),
			owner_can_transfer: Some(limits.owner_can_transfer()),
			owner_can_destroy: Some(limits.owner_can_destroy()),
			transfers_enabled: Some(limits.transfers_enabled()),
		};

		Some(effective_limits)
	}

	/// Returns information about the `collection` adapted for rpc.
	pub fn rpc_collection(collection: CollectionId) -> Option<RpcCollection<T::AccountId>> {
		let Collection {
			name,
			description,
			owner,
			mode,
			token_prefix,
			sponsorship,
			limits,
		} = <CollectionById<T>>::get(collection)?;

		let property_permissions = <PropertyPermissions<T>>::get(collection)
			.into_iter()
			.map(|(key, permission)| PropertyKeyPermission { key, permission })
			.collect();

		let properties = <CollectionProperties<T>>::get(collection)
			.into_iter()
			.map(|(key, value)| Property { key, value })
			.collect();

		Some(RpcCollection {
			name: name.into_inner(),
			description: description.into_inner(),
			owner,
			mode,
			token_prefix: token_prefix.into_inner(),
			sponsorship,
			limits,
			property_permissions,
			properties,
		})
	}
}
