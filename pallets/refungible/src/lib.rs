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

// ! # Refungible Pallet
// !
// ! The Refungible pallet provides functionality for handling refungible collections and tokens.
// !
// ! ## Overview
// !
// ! The Refungible pallet provides functions for:
// !
// ! - RFT collection creation and removal
// ! - Minting and burning of RFT tokens
// ! - Partition and repartition of RFT tokens
// ! - Retrieving number of pieces of RFT token
// ! - Retrieving account balances
// ! - Transfering RFT token pieces
// ! - Burning RFT token pieces
// ! - Setting and checking allowance for RFT tokens
// !
// ! ### Terminology
// !
// ! - **RFT token:** Non fungible token that was partitioned to pieces. If an account owns all
// !   of the RFT token pieces than it owns the RFT token and can repartition it.
// !
// ! - **RFT Collection:** A collection of RFT tokens. All RFT tokens are part of a collection.
// !   Each collection has its own settings and set of permissions.
// !
// ! - **RFT token piece:** A fungible part of an RFT token.
// !
// ! - **Balance:** RFT token pieces owned by an account
// !
// ! - **Allowance:** Maximum number of RFT token pieces that one account is allowed to
// !   transfer from the balance of another account
// !
// ! - **Burning:** The process of “deleting” a token from a collection or removing token pieces from
// !   an account balance.
// !
// ! ### Implementations
// !
// ! The Refungible pallet provides implementations for the following traits. If these traits provide
// ! the functionality that you need, then you can avoid coupling with the Refungible pallet.
// !
// ! - [`CommonWeightInfo`](pallet_common::CommonWeightInfo): Functions for retrieval of transaction weight
// ! - [`CommonCollectionOperations`](pallet_common::CommonCollectionOperations): Functions for dealing
// !   with collections
// ! - [`RefungibleExtensions`](pallet_common::RefungibleExtensions): Functions specific for refungible
// !   collection
// !
// ! ## Interface
// !
// ! ### Dispatchable Functions
// !
// ! - `init_collection` - Create RFT collection. RFT collection can be configured to allow or deny access for
// !   some accounts.
// ! - `destroy_collection` - Destroy exising RFT collection. There should be no tokens in the collection.
// ! - `burn` - Burn some amount of RFT token pieces owned by account. Burns the RFT token if no pieces left.
// ! - `transfer` - Transfer some amount of RFT token pieces. Transfers should be enabled for RFT collection.
// !   Nests the RFT token if RFT token pieces are sent to another token.
// ! - `create_item` - Mint RFT token in collection. Sender should have permission to mint tokens.
// ! - `set_allowance` - Set allowance for another account to transfer balance from sender's account.
// ! - `repartition` - Repartition token to selected number of pieces. Sender should own all existing pieces.
// !
// ! ## Assumptions
// !
// ! * Total number of pieces for one token shouldn't exceed `collection_primitives::MAX_REFUNGIBLE_PIECES`.
// ! * Total number of tokens of all types shouldn't be greater than `collection_primitives::MAX_TOKEN_PREFIX_LENGTH`.
// ! * Sender should be in collection's allow list to perform operations on tokens.

#![cfg_attr(not(feature = "std"), no_std)]

use core::ops::Deref;
use derivative::Derivative;
use frame_support::{
	BoundedBTreeMap, ensure, fail, storage::with_transaction, transactional,
	pallet_prelude::ConstU32,
	dispatch::{DispatchResult}
};
use pallet_common::{
	Error as CommonError,
	Event as CommonEvent, Pallet as PalletCommon,
};
use sp_runtime::{ArithmeticError, DispatchError, TransactionOutcome};
use sp_std::{vec::Vec, vec, collections::{btree_map::BTreeMap}};
use collection_primitives::{
	CollectionId, CollectionFlags, CollectionPropertiesVec,
	CreateCollectionData, MAX_ITEMS_PER_BATCH, MAX_PROPERTIES_PER_ITEM,
	MAX_REFUNGIBLE_PIECES, Property, PropertyKey, PropertyKeyPermission, PropertyPermission,
	PropertyScope, PropertyValue, TokenId, TrySetProperty,
};

use pallet_whitelist::{
	traits::WhitelistInterface
};

pub use weights::WeightInfo;

pub use pallet::*;
pub mod weights;
pub mod mock;
pub mod tests;

pub type TokenBalance = u128;

// LOG: CrossAccountId -> AccountId
#[derive(Derivative, Clone)]
pub struct CreateItemData<T: Config> { //CrossAccountId -> unique frontier
	#[derivative(Debug(format_with = "bounded::map_debug"))]
	pub users: BoundedBTreeMap<T::AccountId, TokenBalance, ConstU32<MAX_ITEMS_PER_BATCH>>,
	#[derivative(Debug(format_with = "bounded::vec_debug"))]
	pub properties: CollectionPropertiesVec, // CollectionProperties -> Properties -> PropertiesMap with bounded size, Key -> Value
}


impl<T: Config> CreateItemData<T> {
	pub fn try_get(user_balances: Vec<(T::AccountId, TokenBalance)>, token_properties: Vec<Property>) -> Result<Self, DispatchError> {
		let properties = CollectionPropertiesVec::truncate_from(token_properties);
		
		let user_balances_count = user_balances.len();
		let users = <BTreeMap<T::AccountId, TokenBalance>>::from_iter(user_balances);
		if users.len() != user_balances_count {
			fail!(<Error<T>>::UserDuplicatesGiven);
		}

		if let Ok(users) = BoundedBTreeMap::try_from(users) {
			Ok(Self { users, properties })
		} else {
			fail!(<Error<T>>::InvalidInput);
		}
	}

	pub fn try_get_multiple(users_balances: Vec<Vec<(T::AccountId, TokenBalance)>>, tokens_properties: Vec<Vec<Property>>) -> Result<Vec<Self>, DispatchError> {
		if users_balances.len() != tokens_properties.len() {
			fail!(<Error<T>>::NotCompleteItemsData);
		}

		let mut data = <Vec<Self>>::with_capacity(users_balances.len());

		for (user_balances, token_properties) in users_balances.iter().zip(tokens_properties) {
			let token_data = Self::try_get(user_balances.to_vec(), token_properties)?;
			data.push(token_data);
		}

		Ok(data)
	}
}

pub struct RefungibleHandle<T: Config>(pallet_common::CollectionHandle<T>);
impl<T: Config> RefungibleHandle<T> {
	pub fn cast(inner: pallet_common::CollectionHandle<T>) -> Self {
		Self(inner)
	}
	
	pub fn into_inner(self) -> pallet_common::CollectionHandle<T> {
		self.0
	}
	
	pub fn common_mut(&mut self) -> &mut pallet_common::CollectionHandle<T> {
		&mut self.0
	}
	
	pub fn try_get(collection_id: CollectionId) -> Result<Self, DispatchError> {
		let common_collection = pallet_common::CollectionHandle::<T>::try_get(collection_id)?;

		Ok(RefungibleHandle::cast(common_collection))
	}
}

impl<T: Config> Deref for RefungibleHandle<T> {
	type Target = pallet_common::CollectionHandle<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		Blake2_128, Blake2_128Concat, Twox64Concat, pallet_prelude::*, storage::Key,
		traits::StorageVersion,
	};
	use frame_system::pallet_prelude::*;
	use collection_primitives::{CollectionId, TokenId};
	use super::weights::WeightInfo;

	#[pallet::error]
	pub enum Error<T> {
		/// Undefined error during input parsing
		InvalidInput,
		/// When creating multiple items balances and properties must be defined for each item
		NotCompleteItemsData,
		/// User duplicates given in initial token balances
		UserDuplicatesGiven,
		/// Not whitelisted or not active investor given
		NeitherWhitelistedNorCollectionAdmin,
		/// Not Refungible item data used to mint in Refungible collection.
		NotRefungibleDataUsedToMintFungibleCollectionToken,
		/// Maximum refungibility exceeded.
		WrongRefungiblePieces,
		/// Refungible token can't be repartitioned by user who isn't owns all pieces.
		RepartitionWhileNotOwningAllPieces,
		/// Refungible token can't nest other tokens.
		RefungibleDisallowsNesting,
		/// Setting item properties is not allowed.
		SettingPropertiesNotAllowed,
	}

	// LOG: pallet_structure::Config unpluged
	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_common::Config
	{
		type WeightInfo: WeightInfo;
	}

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Total amount of minted tokens in a collection.
	#[pallet::storage]
	pub type TokensMinted<T: Config> =
		StorageMap<Hasher = Twox64Concat, Key = CollectionId, Value = u32, QueryKind = ValueQuery>;

	/// Amount of tokens burnt in a collection.
	#[pallet::storage]
	pub type TokensBurnt<T: Config> =
		StorageMap<Hasher = Twox64Concat, Key = CollectionId, Value = u32, QueryKind = ValueQuery>;

	/// Amount of pieces a refungible token is split into.
	#[pallet::storage]
	#[pallet::getter(fn token_properties)]
	pub type TokenProperties<T: Config> = StorageNMap<
		Key = (Key<Twox64Concat, CollectionId>, Key<Twox64Concat, TokenId>),
		Value = collection_primitives::Properties,
		QueryKind = ValueQuery,
		OnEmpty = collection_primitives::TokenProperties,
	>;

	/// Total amount of pieces for token
	#[pallet::storage]
	pub type TotalSupply<T: Config> = StorageNMap<
		Key = (Key<Twox64Concat, CollectionId>, Key<Twox64Concat, TokenId>),
		Value = TokenBalance,
		QueryKind = ValueQuery,
	>;

	/// Used to enumerate tokens owned by account.
	// LOG: CrossAccountId -> AccountId
	#[pallet::storage]
	pub type Owned<T: Config> = StorageNMap<
		Key = (
			Key<Twox64Concat, CollectionId>,
			Key<Blake2_128Concat, T::AccountId>,
			Key<Twox64Concat, TokenId>,
		),
		Value = bool,
		QueryKind = ValueQuery,
	>;

	/// Amount of tokens (not pieces) partially owned by an account within a collection.
	// LOG: CrossAccountId -> AccountId
	#[pallet::storage]
	pub type AccountBalance<T: Config> = StorageNMap<
		Key = (
			Key<Twox64Concat, CollectionId>,
			// Owner
			Key<Blake2_128Concat, T::AccountId>,
		),
		Value = u32,
		QueryKind = ValueQuery,
	>;

	/// Amount of token pieces owned by account.
	// LOG: CrossAccountId -> AccountId
	#[pallet::storage]
	pub type Balance<T: Config> = StorageNMap<
		Key = (
			Key<Twox64Concat, CollectionId>,
			Key<Twox64Concat, TokenId>,
			// Owner
			Key<Blake2_128Concat, T::AccountId>,
		),
		Value = TokenBalance,
		QueryKind = ValueQuery,
	>;

	/// Allowance set by a token owner for another user to perform one of certain transactions on a number of pieces of a token.
	// LOG: CrossAccountId -> AccountId
	#[pallet::storage]
	pub type Allowance<T: Config> = StorageNMap<
		Key = (
			Key<Twox64Concat, CollectionId>,
			Key<Twox64Concat, TokenId>,
			// Owner
			Key<Blake2_128, T::AccountId>,
			// Spender
			Key<Blake2_128Concat, T::AccountId>,
		),
		Value = TokenBalance,
		QueryKind = ValueQuery,
	>;

	// LOG: ItemData deprection runtime upgrade hook deleted
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::init_collection(
			data.token_property_permissions.len().try_into().unwrap(), 
			data.properties.len().try_into().unwrap()
		))]
		pub fn init_collection(
			origin: OriginFor<T>, 
			data: CreateCollectionData<T::AccountId>, 
			flags: CollectionFlags
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::_init_collection(who.clone(), who, data, flags)?;

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::destroy_collection())]
		pub fn destroy_collection(
			origin: OriginFor<T>, 
			collection_id: CollectionId
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_destroy_collection(collection, &who)
		}

		#[pallet::weight(<T as Config>::WeightInfo::set_collection_property())]
		pub fn set_collection_property(
			origin: OriginFor<T>, 
			collection_id: CollectionId,
			property: Property
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_set_collection_property(&collection, &who, property)
		}

		#[pallet::weight(<T as Config>::WeightInfo::delete_collection_property())]
		pub fn delete_collection_property(
			origin: OriginFor<T>, 
			collection_id: CollectionId,
			property_key: PropertyKey
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_delete_collection_property(&collection, &who, property_key)
		}

		#[pallet::weight(<T as Config>::WeightInfo::set_collection_properties(properties.len().try_into().unwrap_or(MAX_PROPERTIES_PER_ITEM)))]
		pub fn set_collection_properties(
			origin: OriginFor<T>, 
			collection_id: CollectionId,
			properties: Vec<Property>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_set_collection_properties(&collection, &who, properties)
		}

		#[pallet::weight(<T as Config>::WeightInfo::delete_collection_properties(property_keys.len().try_into().unwrap_or(MAX_PROPERTIES_PER_ITEM)))]
		pub fn delete_collection_properties(
			origin: OriginFor<T>, 
			collection_id: CollectionId,
			property_keys: Vec<PropertyKey>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_delete_collection_properties(&collection, &who, property_keys)
		}

		#[pallet::weight(<T as Config>::WeightInfo::set_property_permission())]
		pub fn set_property_permission(
			origin: OriginFor<T>, 
			collection_id: CollectionId,
			property_permission: PropertyKeyPermission
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_set_property_permission(&collection, &who, property_permission)
		}

		#[pallet::weight(<T as Config>::WeightInfo::create_item(
			user_balances.len().try_into().unwrap_or(MAX_ITEMS_PER_BATCH), 
			token_properties.len().try_into().unwrap_or(MAX_PROPERTIES_PER_ITEM)
		))]
		pub fn create_item(
			origin: OriginFor<T>, 
			collection_id: CollectionId,
			user_balances: Vec<(T::AccountId, TokenBalance)>,
			token_properties: Vec<Property> 
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			let data = <CreateItemData<T>>::try_get(user_balances, token_properties)?;

			Self::_create_item(&collection, &who, data)
		}

		#[pallet::weight(<T as Config>::WeightInfo::create_max_item().saturating_mul(users_balances.len().try_into().unwrap_or(MAX_ITEMS_PER_BATCH as u64)))]
		pub fn create_multiple_items(
			origin: OriginFor<T>, 
			collection_id: CollectionId,
			users_balances: Vec<Vec<(T::AccountId, TokenBalance)>>,
			tokens_properties: Vec<Vec<Property>>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			let data = <CreateItemData::<T>>::try_get_multiple(users_balances, tokens_properties)?;

			Self::_create_multiple_items(&collection, &who, data)
		}

		#[pallet::weight(<T as Config>::WeightInfo::set_token_property())]
		pub fn set_token_property(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
			property: Property
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_set_token_property(&collection, &who, token_id, property)
		}

		#[pallet::weight(<T as Config>::WeightInfo::set_token_properties(properties.len().try_into().unwrap_or(MAX_PROPERTIES_PER_ITEM)))]
		pub fn set_token_properties(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
			properties: Vec<Property>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_set_token_properties(&collection, &who, token_id, properties.into_iter(), false)
		}

		#[pallet::weight(<T as Config>::WeightInfo::delete_token_property())]
		pub fn delete_token_property(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
			property_key: PropertyKey
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_delete_token_property(&collection, &who, token_id, property_key)
		}

		#[pallet::weight(<T as Config>::WeightInfo::delete_token_properties(properties_keys.len().try_into().unwrap_or(MAX_PROPERTIES_PER_ITEM)))]
		pub fn delete_token_properties(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
			properties_keys: Vec<PropertyKey>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_delete_token_properties(&collection, &who, token_id, properties_keys.into_iter())
		}

		#[pallet::weight(<T as Config>::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
			to: T::AccountId,
			amount: TokenBalance
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_transfer(&collection, &from, &to, token_id, amount)
		}

		#[pallet::weight(<T as Config>::WeightInfo::transfer_from())]
		pub fn transfer_from(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
			from: T::AccountId,
			to: T::AccountId,
			amount: TokenBalance
		) -> DispatchResult {
			let spender = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_transfer_from(&collection, &spender, &from, &to, token_id, amount)
		}

		#[pallet::weight(<T as Config>::WeightInfo::set_allowance())]
		pub fn set_allowance(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
			spender: T::AccountId,
			amount: TokenBalance
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_set_allowance(&collection, &sender, &spender, token_id, amount)
		}

		#[pallet::weight(<T as Config>::WeightInfo::burn())]
		pub fn burn(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
			amount: TokenBalance
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_burn(&collection, &who, token_id, amount)
		}

		#[pallet::weight(<T as Config>::WeightInfo::burn_from())]
		pub fn burn_from(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
			from: T::AccountId,
			amount: TokenBalance
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_burn_from(&collection, &who, &from, token_id, amount)
		}

		#[pallet::weight(<T as Config>::WeightInfo::repartition())]
		pub fn repartition(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
			amount: TokenBalance
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_repartition(&collection, &who, token_id, amount)
		}

		#[pallet::weight(<T as Config>::WeightInfo::toggle_admin())]
		pub fn toggle_admin(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			user: T::AccountId,
			admin: bool
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection = RefungibleHandle::try_get(collection_id)?;

			Self::_toggle_admin(&collection, &who, &user, admin)
		}
	}
}

// Collection related methods
impl<T: Config> Pallet<T> {
	/// Create RFT collection
	///
	/// `init_collection` will take non-refundable deposit for collection creation.
	///
	/// - `data`: Contains settings for collection limits and permissions.
	// LOG: CrossAccountId -> AccountId
	fn _init_collection(
		owner: T::AccountId,
		payer: T::AccountId,
		data: CreateCollectionData<T::AccountId>,
		flags: CollectionFlags,
	) -> Result<CollectionId, DispatchError> {
		<PalletCommon<T>>::init_collection(owner, payer, data, flags)
	}

	/// Destroy RFT collection
	///
	/// `destroy_collection` will throw error if collection contains any tokens.
	/// Only owner can destroy collection.
	// LOG: CrossAccountId -> AccountId
	fn _destroy_collection(
		collection: RefungibleHandle<T>,
		sender: &T::AccountId,
	) -> DispatchResult {
		let id = collection.id;

		if Self::_collection_has_tokens(id) {
			return Err(<CommonError<T>>::CantDestroyNotEmptyCollection.into());
		}

		// =========

		PalletCommon::destroy_collection(collection.0, sender)?;

		<TokensMinted<T>>::remove(id);
		<TokensBurnt<T>>::remove(id);
		let _ = <TotalSupply<T>>::clear_prefix((id,), u32::MAX, None);
		let _ = <Balance<T>>::clear_prefix((id,), u32::MAX, None);
		let _ = <Allowance<T>>::clear_prefix((id,), u32::MAX, None);
		let _ = <Owned<T>>::clear_prefix((id,), u32::MAX, None);
		let _ = <AccountBalance<T>>::clear_prefix((id,), u32::MAX, None);
		Ok(())
	}

	fn _collection_has_tokens(collection_id: CollectionId) -> bool {
		<TotalSupply<T>>::iter_prefix((collection_id,))
			.next()
			.is_some()
	}

	// LOG: CrossAccountId -> AccountId
	fn _set_collection_property(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		property: Property,
	) -> DispatchResult {
		<PalletCommon<T>>::set_collection_property(collection, sender, property)
	}

	// LOG: CrossAccountId -> AccountId
	fn _delete_collection_property(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		property_key: PropertyKey,
	) -> DispatchResult {
		<PalletCommon<T>>::delete_collection_property(collection, sender, property_key)
	}

	// LOG: CrossAccountId -> AccountId
	fn _set_collection_properties(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		properties: Vec<Property>,
	) -> DispatchResult {
		<PalletCommon<T>>::set_collection_properties(collection, sender, properties)
	}

	// LOG: CrossAccountId -> AccountId
	fn _delete_collection_properties(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		property_keys: Vec<PropertyKey>,
	) -> DispatchResult {
		<PalletCommon<T>>::delete_collection_properties(collection, sender, property_keys)
	}

	fn _set_property_permission(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		property_permission: PropertyKeyPermission,
	) -> DispatchResult {
		<PalletCommon<T>>::set_property_permission(collection, sender, property_permission)
	}

	fn _toggle_admin(
		collection: &RefungibleHandle<T>, 
		who: &T::AccountId, 
		user: &T::AccountId,
		admin: bool
	) -> DispatchResult {
		<PalletCommon<T>>::toggle_admin(collection, who, user, admin)
	}
}

// Token related methods
impl<T: Config> Pallet<T> {
	/// Create RFT token.
	///
	/// The sender should be the owner/admin of the collection or collection should be configured
	/// to allow public minting.
	///
	/// - `data`: Contains list of users who will become the owners of the token pieces and amount
	///   of token pieces they will receive.
	// LOG: CrossAccountId -> AccountId
	// LOG: nesting_budget argument deleted
	fn _create_item(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		data: CreateItemData<T>,
	) -> DispatchResult {
		Self::_create_multiple_items(collection, sender, vec![data])
	}

	/// Batched operation to create multiple RFT tokens.
	///
	/// Same as `create_item` but creates multiple tokens.
	///
	/// - `data`: Same as 'data` in `create_item` but contains data for multiple tokens.
	// LOG: CrossAccountId -> AccountId
	// LOG: EVM events deleted
	// LOG: Nesting deleted
	// LOG: ensure_correct_receiver check (not zero eth address) deleted
	// LOG: nesting_budget argument deleted
	// LOG: Validation restricted to onwer or admin check
	//      Allowlist changed to WL active investor check
	// LOG: Added check that initial token total supply is bigger than 0
	fn _create_multiple_items(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		data: Vec<CreateItemData<T>>,
	) -> DispatchResult {

		collection.check_is_owner_or_admin(sender)?;
		Self::_verify_create_multiple_items_data(collection, &data)?;

		// Total pieces per tokens
		let totals = data
			.iter()
			.map(|data| {
				Ok(data
					.users
					.iter()
					.map(|u| u.1)
					.try_fold(0u128, |acc, v| acc.checked_add(*v))
					.ok_or(ArithmeticError::Overflow)?)
			})
			.collect::<Result<Vec<_>, DispatchError>>()?;
		for total in &totals {
			ensure!(
				*total <= MAX_REFUNGIBLE_PIECES && *total > 0,
				<Error<T>>::WrongRefungiblePieces
			);
		}

		let first_token_id = <TokensMinted<T>>::get(collection.id);
		let tokens_minted = first_token_id
			.checked_add(data.len() as u32)
			.ok_or(ArithmeticError::Overflow)?;
		ensure!(
			tokens_minted < collection.limits.token_limit(),
			<CommonError<T>>::CollectionTokenLimitExceeded
		);

		let mut balances = BTreeMap::new();
		for data in &data {
			for owner in data.users.keys() {
				let balance = balances
					.entry(owner)
					.or_insert_with(|| <AccountBalance<T>>::get((collection.id, owner)));
				*balance = balance.checked_add(1).ok_or(ArithmeticError::Overflow)?;

				ensure!(
					*balance <= collection.limits.account_token_ownership_limit(),
					<CommonError<T>>::AccountTokenLimitExceeded,
				);
			}
		}

		// =========

		with_transaction(|| {
			for (i, data) in data.iter().enumerate() {
				let token_id = first_token_id + i as u32 + 1;
				<TotalSupply<T>>::insert((collection.id, token_id), totals[i]);

				for (user, amount) in data.users.iter() {
					if *amount == 0 {
						continue;
					}
					<Balance<T>>::insert((collection.id, token_id, &user), amount);
					<Owned<T>>::insert((collection.id, &user, TokenId(token_id)), true);
				}

				if let Err(e) = Self::_set_token_properties(
					collection,
					sender,
					TokenId(token_id),
					data.properties.clone().into_iter(),
					true,
				) {
					return TransactionOutcome::Rollback(Err(e));
				}
			}
			TransactionOutcome::Commit(Ok(()))
		})?;

		<TokensMinted<T>>::insert(collection.id, tokens_minted);

		for (account, balance) in balances {
			<AccountBalance<T>>::insert((collection.id, account), balance);
		}

		for (i, token) in data.into_iter().enumerate() {
			let token_id = first_token_id + i as u32 + 1;

			let receivers = token
				.users
				.into_iter()
				.filter(|(_, amount)| *amount > 0)
				.collect::<Vec<_>>();

			for (user, amount) in receivers.into_iter() {
				<PalletCommon<T>>::deposit_event(CommonEvent::ItemCreated(
					collection.id,
					TokenId(token_id),
					user,
					amount,
				));
			}
		}
		Ok(())
	}

	// Check that all given users are either whitelisted or collection admins
	fn _verify_create_multiple_items_data(collection: &RefungibleHandle<T>, data: &Vec<CreateItemData<T>>) -> DispatchResult {
		for item in data.iter() {
			for user in item.users.keys() {
				Self::_check_whitelisted_or_collection_admin(collection, user)?;
			}
		}

		Ok(())
	}

	// Check that user is either whitelisted or collection admin
	fn _check_whitelisted_or_collection_admin(collection: &RefungibleHandle<T>, user: &T::AccountId) -> DispatchResult {
		ensure!(
			T::Whitelist::is_active_investor(user) || collection.is_owner_or_admin(user),
			<Error<T>>::NeitherWhitelistedNorCollectionAdmin
		);

		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	// LOG: nesting_budget argument deleted
	fn _set_token_properties(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		token_id: TokenId,
		properties: impl Iterator<Item = Property>,
		is_token_create: bool,
	) -> DispatchResult {
		Self::_modify_token_properties(
			collection,
			sender,
			token_id,
			properties.map(|p| (p.key, Some(p.value))),
			is_token_create,
		)
	}

	// LOG: CrossAccountId -> AccountId
	// LOG: nesting_budget argument deleted
	fn _set_token_property(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		token_id: TokenId,
		property: Property,
	) -> DispatchResult {
		let is_token_create = false;

		Self::_set_token_properties(
			collection,
			sender,
			token_id,
			[property].into_iter(),
			is_token_create,
		)
	}

	// LOG: CrossAccountId -> AccountId
	// LOG: nesting_budget argument deleted
	fn _delete_token_properties(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		token_id: TokenId,
		property_keys: impl Iterator<Item = PropertyKey>,
	) -> DispatchResult {
		let is_token_create = false;

		Self::_modify_token_properties(
			collection,
			sender,
			token_id,
			property_keys.into_iter().map(|key| (key, None)),
			is_token_create,
		)
	}

	// LOG: CrossAccountId -> AccountId
	// LOG: nesting_budget argument deleted
	fn _delete_token_property(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		token_id: TokenId,
		property_key: PropertyKey,
	) -> DispatchResult {
		Self::_delete_token_properties(
			collection,
			sender,
			token_id,
			[property_key].into_iter(),
		)
	}

	// LOG: CrossAccountId -> AccountId
	// LOG: Indirect token ownership check deleted. User owns token if holds
	// 		all the token pieces.
	// LOG: Balance getting in `is_token_owner` closure fixed
	// LOG: nesting_budget argument deleted
	// LOG: token_owner permission ignored
	// LOG: Access restricted to owner or admin
	// LOG: PropertyPermission check simplified.
	//		Now property can be modified only by owner or admins
	//		if mutable = true. By default mutable = false and 
	//		property can be set only while token creation.
	#[transactional]
	fn _modify_token_properties(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		token_id: TokenId,
		properties: impl Iterator<Item = (PropertyKey, Option<PropertyValue>)>,
		is_token_create: bool,
	) -> DispatchResult {
		collection.check_is_owner_or_admin(sender)?;

		for (key, value) in properties {
			let permission = <PalletCommon<T>>::property_permissions(collection.id)
				.get(&key)
				.cloned()
				.unwrap_or_else(PropertyPermission::unmutable);

			if !is_token_create {
				if let PropertyPermission { mutable: false, .. } = permission {
					fail!(<CommonError<T>>::NoPermission);
				}
			}

			match value {
				Some(value) => {
					<TokenProperties<T>>::try_mutate((collection.id, token_id), |properties| {
						properties.try_set(key.clone(), value)
					})
					.map_err(<CommonError<T>>::from)?;

					<PalletCommon<T>>::deposit_event(CommonEvent::TokenPropertySet(
						collection.id,
						token_id,
						key,
					));
				}
				None => {
					<TokenProperties<T>>::try_mutate((collection.id, token_id), |properties| {
						properties.remove(&key)
					})
					.map_err(<CommonError<T>>::from)?;

					<PalletCommon<T>>::deposit_event(CommonEvent::TokenPropertyDeleted(
						collection.id,
						token_id,
						key,
					));
				}
			}
		}

		Ok(())
	}

	/// Transfer RFT token pieces from one account to another.
	///
	/// If the sender is no longer owns any pieces after the `transfer` than she stops being an owner of the token.
	///
	/// - `from`: Owner of token pieces to transfer.
	/// - `to`: Recepient of transfered token pieces.
	/// - `amount`: Amount of token pieces to transfer.
	/// - `token`: Token whos pieces should be transfered
	/// - `collection`: Collection that contains the token
	// LOG: CrossAccountId -> AccountId
	// LOG: EVM events deleted
	// LOG: Nesting/unnesting deleted
	// LOG: ensure_correct_receiver check (not zero eth address) deleted
	// LOG: nesting_budget argument deleted
	fn _transfer(
		collection: &RefungibleHandle<T>,
		from: &T::AccountId,
		to: &T::AccountId,
		token: TokenId,
		amount: TokenBalance,
	) -> DispatchResult {
		Self::_check_transfer_allowed(collection, from, to)?;

		let initial_balance_from = <Balance<T>>::get((collection.id, token, from));
		let updated_balance_from = initial_balance_from
			.checked_sub(amount)
			.ok_or(<CommonError<T>>::TokenValueTooLow)?;
		let mut create_target = false;
		let from_to_differ = from != to;
		let updated_balance_to = if from != to {
			let old_balance = <Balance<T>>::get((collection.id, token, to));
			if old_balance == 0 {
				create_target = true;
			}
			Some(
				old_balance
					.checked_add(amount)
					.ok_or(ArithmeticError::Overflow)?,
			)
		} else {
			None
		};

		let account_balance_from = if updated_balance_from == 0 {
			Some(
				<AccountBalance<T>>::get((collection.id, from))
					.checked_sub(1)
					// Should not occur
					.ok_or(ArithmeticError::Underflow)?,
			)
		} else {
			None
		};
		// Account data is created in token, AccountBalance should be increased
		// But only if from != to as we shouldn't check overflow in this case
		let account_balance_to = if create_target && from_to_differ {
			let account_balance_to = <AccountBalance<T>>::get((collection.id, to))
				.checked_add(1)
				.ok_or(ArithmeticError::Overflow)?;
			ensure!(
				account_balance_to < collection.limits.account_token_ownership_limit(),
				<CommonError<T>>::AccountTokenLimitExceeded,
			);

			Some(account_balance_to)
		} else {
			None
		};

		// =========

		if let Some(updated_balance_to) = updated_balance_to {
			// from != to
			if updated_balance_from == 0 {
				<Balance<T>>::remove((collection.id, token, from));
			} else {
				<Balance<T>>::insert((collection.id, token, from), updated_balance_from);
			}
			<Balance<T>>::insert((collection.id, token, to), updated_balance_to);
			if let Some(account_balance_from) = account_balance_from {
				<AccountBalance<T>>::insert((collection.id, from), account_balance_from);
				<Owned<T>>::remove((collection.id, from, token));
			}
			if let Some(account_balance_to) = account_balance_to {
				<AccountBalance<T>>::insert((collection.id, to), account_balance_to);
				<Owned<T>>::insert((collection.id, to, token), true);
			}
		}

		<PalletCommon<T>>::deposit_event(CommonEvent::Transfer(
			collection.id,
			token,
			from.clone(),
			to.clone(),
			amount,
		));

		Ok(())
	}
	
	fn _check_transfer_allowed(collection: &RefungibleHandle<T>, from: &T::AccountId, to: &T::AccountId) -> DispatchResult {
		ensure!(
			collection.limits.transfers_enabled(),
			<CommonError<T>>::TransfersDisabled
		);

		Self::_check_whitelisted_or_collection_admin(collection, from)?;
		Self::_check_whitelisted_or_collection_admin(collection, to)?;

		Ok(())
	}

	/// Transfer RFT token pieces from one account to another.
	///
	/// Same as the [`transfer`] but spender doesn't needs to be an owner of the token pieces.
	/// The owner should set allowance for the spender to transfer pieces.
	///
	/// [`transfer`]: struct.Pallet.html#method.transfer
	// LOG: CrossAccountId -> AccountId
	// LOG: nesting_budget argument deleted
	fn _transfer_from(
		collection: &RefungibleHandle<T>,
		spender: &T::AccountId,
		from: &T::AccountId,
		to: &T::AccountId,
		token: TokenId,
		amount: TokenBalance,
	) -> DispatchResult {
		let allowance =
			Self::_check_allowed(collection, spender, from, token, amount)?;

		// =========

		Self::_transfer(collection, from, to, token, amount)?;
		if let Some(allowance) = allowance {
			Self::_set_allowance_unchecked(collection, from, spender, token, allowance);
		}
		Ok(())
	}

	/// Returns allowance, which should be set after transaction
	// LOG: CrossAccountId -> AccountId
	// LOG: Indirect ownership check deleted
	// LOG: AccountId comparison fixed
	// LOG: nesting_budget argument deleted
	fn _check_allowed(
		collection: &RefungibleHandle<T>,
		spender: &T::AccountId,
		from: &T::AccountId,
		token: TokenId,
		amount: TokenBalance,
	) -> Result<Option<TokenBalance>, DispatchError> {
		if spender == from {
			return Ok(None);
		}

		Self::_check_whitelisted_or_collection_admin(collection, spender)?;
		Self::_check_whitelisted_or_collection_admin(collection, from)?;

		let allowance =
			<Allowance<T>>::get((collection.id, token, from, &spender)).checked_sub(amount);
		if allowance.is_none() {
			ensure!(
				collection.ignores_allowance(spender),
				<CommonError<T>>::ApprovedValueTooLow
			);
		}
		Ok(allowance)
	}

	/// Set allowance for the spender to `transfer` or `burn` sender's token pieces.
	///
	/// - `amount`: Amount of token pieces the spender is allowed to `transfer` or `burn.
	// LOG: CrossAccountId -> AccountId
	// LOG: ensure_correct_receiver check (not zero eth address) deleted
	fn _set_allowance(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		spender: &T::AccountId,
		token: TokenId,
		amount: TokenBalance,
	) -> DispatchResult {
		Self::_check_whitelisted_or_collection_admin(collection, sender)?;
		Self::_check_whitelisted_or_collection_admin(collection, spender)?;

		if <Balance<T>>::get((collection.id, token, sender)) < amount {
			ensure!(
				collection.ignores_owned_amount(sender) && Self::token_exists(collection, token),
				<CommonError<T>>::CantApproveMoreThanOwned
			);
		}

		// =========

		Self::_set_allowance_unchecked(collection, sender, spender, token, amount);
		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	// LOG: EVM event deleted
	fn _set_allowance_unchecked(
		collection: &RefungibleHandle<T>,
		sender: &T::AccountId,
		spender: &T::AccountId,
		token: TokenId,
		amount: TokenBalance,
	) {
		if amount == 0 {
			<Allowance<T>>::remove((collection.id, token, sender, spender));
		} else {
			<Allowance<T>>::insert((collection.id, token, sender, spender), amount);
		}

		<PalletCommon<T>>::deposit_event(CommonEvent::Approved(
			collection.id,
			token,
			sender.clone(),
			spender.clone(),
			amount,
		))
	}

	/// Burn RFT token pieces
	///
	/// `burn` will decrease total amount of token pieces and amount owned by sender.
	/// `burn` can be called even if there are multiple owners of the RFT token.
	/// If sender wouldn't have any pieces left after `burn` than she will stop being
	/// one of the owners of the token. If there is no account that owns any pieces of
	/// the token than token will be burned too.
	///
	/// - `amount`: Amount of token pieces to burn.
	/// - `token`: Token who's pieces should be burned
	/// - `collection`: Collection that contains the token
	fn _burn(
		collection: &RefungibleHandle<T>,
		owner: &T::AccountId,
		token: TokenId,
		amount: TokenBalance,
	) -> DispatchResult {
		collection.check_is_owner_or_admin(owner)?;

		Self::_burn_permissionless(collection, owner, token, amount)
	}
	
	/// Burn RFT token pieces from the account.
	///
	/// Same as the [`burn`] but spender doesn't need to be an owner of the token pieces. The owner should
	/// set allowance for the spender to burn pieces
	///
	/// [`burn`]: struct.Pallet.html#method.burn
	// LOG: CrossAccountId -> AccountId
	// LOG: nesting_budget argument deleted
	fn _burn_from(
		collection: &RefungibleHandle<T>,
		spender: &T::AccountId,
		from: &T::AccountId,
		token: TokenId,
		amount: TokenBalance,
	) -> DispatchResult {
		collection.check_is_owner_or_admin(spender)?;

		let allowance =
			Self::_check_allowed(collection, spender, from, token, amount)?;

		// =========

		Self::_burn_permissionless(collection, from, token, amount)?;
		if let Some(allowance) = allowance {
			Self::_set_allowance_unchecked(collection, from, spender, token, allowance);
		}
		Ok(())
	}

	/// Burn RFT token pieces
	///
	/// `burn` will decrease total amount of token pieces and amount owned by sender.
	/// `burn` can be called even if there are multiple owners of the RFT token.
	/// If sender wouldn't have any pieces left after `burn` than she will stop being
	/// one of the owners of the token. If there is no account that owns any pieces of
	/// the token than token will be burned too.
	///
	/// - `amount`: Amount of token pieces to burn.
	/// - `token`: Token who's pieces should be burned
	/// - `collection`: Collection that contains the token
	// LOG: CrossAccountId -> AccountId
	// LOG: EVM events deleted
	// LOG: Unnesting deleted
	// LOG: Owner or admin check moved to _burn
	fn _burn_permissionless(
		collection: &RefungibleHandle<T>,
		owner: &T::AccountId,
		token: TokenId,
		amount: TokenBalance,
	) -> DispatchResult {
		let total_supply = <TotalSupply<T>>::get((collection.id, token))
			.checked_sub(amount)
			.ok_or(<CommonError<T>>::TokenValueTooLow)?;

		// This was probally last owner of this token?
		if total_supply == 0 {
			// Ensure user actually owns this amount
			ensure!(
				<Balance<T>>::get((collection.id, token, owner)) == amount,
				<CommonError<T>>::TokenValueTooLow
			);
			let account_balance = <AccountBalance<T>>::get((collection.id, owner))
				.checked_sub(1)
				// Should not occur
				.ok_or(ArithmeticError::Underflow)?;

			// =========

			<Owned<T>>::remove((collection.id, owner, token));
			<AccountBalance<T>>::insert((collection.id, owner), account_balance);
			Self::_destroy_item_unchecked(collection, token)?;
			<PalletCommon<T>>::deposit_event(CommonEvent::ItemDestroyed(
				collection.id,
				token,
				owner.clone(),
				amount,
			));
			return Ok(());
		}

		let balance = <Balance<T>>::get((collection.id, token, owner))
			.checked_sub(amount)
			.ok_or(<CommonError<T>>::TokenValueTooLow)?;
		let account_balance = if balance == 0 {
			<AccountBalance<T>>::get((collection.id, owner))
				.checked_sub(1)
				// Should not occur
				.ok_or(ArithmeticError::Underflow)?
		} else {
			0
		};

		// =========

		if balance == 0 {
			<Owned<T>>::remove((collection.id, owner, token));
			<Balance<T>>::remove((collection.id, token, owner));
			<AccountBalance<T>>::insert((collection.id, owner), account_balance);
		} else {
			<Balance<T>>::insert((collection.id, token, owner), balance);
		}
		<TotalSupply<T>>::insert((collection.id, token), total_supply);

		<PalletCommon<T>>::deposit_event(CommonEvent::ItemDestroyed(
			collection.id,
			token,
			owner.clone(),
			amount,
		));
		Ok(())
	}

	// LOG: CrossAccountId -> AccountId
	// LOG: EVM event deleted
	fn _destroy_item_unchecked(
		collection: &RefungibleHandle<T>,
		token_id: TokenId,
	) -> DispatchResult {
		let burnt = <TokensBurnt<T>>::get(collection.id)
			.checked_add(1)
			.ok_or(ArithmeticError::Overflow)?;

		<TokensBurnt<T>>::insert(collection.id, burnt);
		<TokenProperties<T>>::remove((collection.id, token_id));
		<TotalSupply<T>>::remove((collection.id, token_id));
		let _ = <Balance<T>>::clear_prefix((collection.id, token_id), u32::MAX, None);
		let _ = <Allowance<T>>::clear_prefix((collection.id, token_id), u32::MAX, None);
		
		Ok(())
	}

	/// Repartition RFT token.
	///
	/// `repartition` will set token balance of the sender and total amount of token pieces.
	/// Sender should own all of the token pieces. `repartition' could be done even if some
	/// token pieces were burned before.
	///
	/// - `amount`: Total amount of token pieces that the token will have after `repartition`.
	// LOG: CrossAccountId -> AccountId
	// LOG: EVM event deleted
	fn _repartition(
		collection: &RefungibleHandle<T>,
		owner: &T::AccountId,
		token: TokenId,
		amount: TokenBalance,
	) -> DispatchResult {
		collection.check_is_owner_or_admin(owner)?;

		ensure!(
			amount <= MAX_REFUNGIBLE_PIECES,
			<Error<T>>::WrongRefungiblePieces
		);
		ensure!(amount > 0, <CommonError<T>>::TokenValueTooLow);
		// Ensure user owns all pieces
		let _total_pieces = Self::_total_pieces(collection.id, token).unwrap_or(TokenBalance::MAX);
		let balance = <Balance<T>>::get((collection.id, token, owner));
		ensure!(
			_total_pieces == balance,
			<Error<T>>::RepartitionWhileNotOwningAllPieces
		);

		<Balance<T>>::insert((collection.id, token, owner), amount);
		<TotalSupply<T>>::insert((collection.id, token), amount);

		if amount > _total_pieces {
			let mint_amount = amount - _total_pieces;
			<PalletCommon<T>>::deposit_event(CommonEvent::ItemCreated(
				collection.id,
				token,
				owner.clone(),
				mint_amount,
			));
		} else if _total_pieces > amount {
			let burn_amount = _total_pieces - amount;
			<PalletCommon<T>>::deposit_event(CommonEvent::ItemDestroyed(
				collection.id,
				token,
				owner.clone(),
				burn_amount,
			));
		}

		Ok(())
	}

	fn _total_pieces(collection_id: CollectionId, token_id: TokenId) -> Option<TokenBalance> {
		<TotalSupply<T>>::try_get((collection_id, token_id)).ok()
	}
}

// Support methods
impl<T: Config> Pallet<T> {
	/// Get number of RFT tokens in collection
	pub fn total_supply(collection: &RefungibleHandle<T>) -> u32 {
		<TokensMinted<T>>::get(collection.id) - <TokensBurnt<T>>::get(collection.id)
	}

	/// Check that RFT token exists
	///
	/// - `token`: Token ID.
	fn token_exists(collection: &RefungibleHandle<T>, token: TokenId) -> bool {
		<TotalSupply<T>>::contains_key((collection.id, token))
	}

	pub fn set_scoped_token_property(
		collection_id: CollectionId,
		token_id: TokenId,
		scope: PropertyScope,
		property: Property,
	) -> DispatchResult {
		TokenProperties::<T>::try_mutate((collection_id, token_id), |properties| {
			properties.try_scoped_set(scope, property.key, property.value)
		})
		.map_err(<CommonError<T>>::from)?;

		Ok(())
	}

	pub fn set_scoped_token_properties(
		collection_id: CollectionId,
		token_id: TokenId,
		scope: PropertyScope,
		properties: impl Iterator<Item = Property>,
	) -> DispatchResult {
		TokenProperties::<T>::try_mutate((collection_id, token_id), |stored_properties| {
			stored_properties.try_scoped_set_from_iter(scope, properties)
		})
		.map_err(<CommonError<T>>::from)?;

		Ok(())
	}
}