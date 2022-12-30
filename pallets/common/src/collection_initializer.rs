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

use frame_support::dispatch::DispatchError;

use sp_std::prelude::*;
use sp_std::collections::btree_set::BTreeSet;

use collection_primitives::{
	Property, PropertyKey, PropertyKeyPermission, PropertyPermission,
	CollectionId, CollectionPropertiesVec, CollectionPropertiesPermissionsVec,
	CollectionMode, CollectionName, CollectionDescription, CollectionTokenPrefix,
	CreateCollectionData
};

use mock_support::collections::*;

/// Support struct for collection initialization in tests and benchmarks
pub struct CollectionInitializer<T: crate::Config> {
	name: CollectionName,
	description: CollectionDescription,
	token_prefix: CollectionTokenPrefix,
	properties: Vec<Property>,
	property_permissions: Vec<PropertyKeyPermission>,
	admins: Vec<T::AccountId>
}

impl<T: crate::Config> CollectionInitializer<T> {
	pub fn new() -> Self {
		Self {
			name: CollectionName::truncate_from("Collection".encode_utf16().collect()),
			description: CollectionDescription::truncate_from("Description".encode_utf16().collect()),
			token_prefix: CollectionTokenPrefix::truncate_from("STP".as_bytes().into()),
			properties: vec![],
			property_permissions: vec![],
			admins: vec![]
		}
	}

	pub fn admins(self, admins: Vec<T::AccountId>) -> Self {
		Self {
			admins,
			..self
		}
	}

	pub fn properties(self, properties: Vec<(Property, bool)>) -> Self {
		let keys: Vec<_> = properties
			.iter()
			.map(|(p, _)| p.key.clone())
			.collect();

		self.ensure_no_key_duplicates(&keys);

		let property_permissions= properties
			.iter()
			.map(|(property, mutable)| {
				PropertyKeyPermission { 
					key: property.key.clone(), 
					permission: PropertyPermission {
						mutable: *mutable, 
						collection_admin: false,
						token_owner: false
					} 
				}
			})
			.collect();

		let properties: Vec<_> = properties
			.iter()
			.map(|p| p.0.clone())
			.collect();

		Self {
			properties,
			property_permissions,
			..self
		}
	}

	pub fn mutable_property_keys(self, keys: Vec<PropertyKey>) -> Self {
		self.ensure_no_key_duplicates(&keys);

		let property_permissions = keys
			.iter()
			.map(|k| PropertyKeyPermission {
				key: k.clone(),
				permission: PropertyPermission {
					mutable: true,
					collection_admin: false,
					token_owner: false
				}
			})
			.into_iter();

		let property_permissions: Vec<_> = self.property_permissions
			.iter()
			.cloned()
			.chain(property_permissions)
			.collect();

		Self {
			property_permissions,
			..self
		}
	}

	fn ensure_no_key_duplicates(&self, new_keys: &Vec<PropertyKey>) {
		let old_keys = self.property_permissions
			.iter()
			.map(|p| &p.key)
			.into_iter();

		let expected_len = old_keys.len() + new_keys.len();
		let set = BTreeSet::from_iter(old_keys.chain(new_keys.iter()));
		if set.len() != expected_len {
			panic!("Key duplicates!");
		}
	}

	pub fn init(self, owner: T::AccountId) -> Result<CollectionId, DispatchError> {
		let data = CreateCollectionData::<T::AccountId> {
			mode: CollectionMode::ReFungible,
			name: self.name,
			description: self.description,
			token_prefix: self.token_prefix,
			access: None,
			limits: None,
			permissions: None,
			pending_sponsor: None,
			properties: CollectionPropertiesVec::truncate_from(self.properties),
			token_property_permissions: CollectionPropertiesPermissionsVec::truncate_from(
				self.property_permissions
			),
		};
		
		let flags = default_collection_flags();

		let collection_id = crate::Pallet::<T>::init_collection(owner.clone(), owner.clone(), data, flags)?;
		let collection = crate::CollectionHandle::<T>::new(collection_id).unwrap();
		
		for admin in self.admins {
			crate::Pallet::<T>::toggle_admin(&collection, &owner, &admin, true)?;
		}

		Ok(collection_id)
	}

	pub fn init_default(self, owner: T::AccountId) -> Result<CollectionId, DispatchError> {
		let data = default_create_collection_data::<T>();
		let flags = default_collection_flags();

		crate::Pallet::<T>::init_collection(owner.clone(), owner, data, flags)
	}
}