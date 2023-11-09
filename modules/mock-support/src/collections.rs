use sp_std::prelude::*;

use collection_primitives::{
	CreateCollectionData, CollectionMode, CollectionName,
	CollectionDescription, CollectionTokenPrefix, PropertiesPermissionsVec,
	CollectionPropertiesVec, Property, PropertyKeyPermission,
	PropertyValue, PropertyKey
};

use crate::primitives::*;
use crate::accounts::*;

pub fn default_create_collection_data<T: frame_system::Config>() -> CreateCollectionData<T::AccountId> {
	get_create_collection_data::<T>(
		"Default collection name", 
		"Default collection description", 
		"Default collection token prefix", 
		vec![], 
		vec![]
	)
}

pub fn get_create_collection_data<T: frame_system::Config>(
    name: &str,
    description: &str,
    token_prefix: &str,
    properties: Vec<Property>,
    property_permissions: Vec<PropertyKeyPermission>,
) -> CreateCollectionData<T::AccountId> {
    let name_u16_raw: Vec<u16> = name.encode_utf16().collect();
    let description_u16_raw: Vec<u16> = description.encode_utf16().collect();
    let token_prefix_u8_raw: Vec<u8> = token_prefix.as_bytes().into();

    CreateCollectionData::<T::AccountId> {
        mode: CollectionMode::ReFungible,
        name: CollectionName::truncate_from(name_u16_raw),
        description: CollectionDescription::truncate_from(description_u16_raw),
        token_prefix: CollectionTokenPrefix::truncate_from(token_prefix_u8_raw),
        limits: None,
        pending_sponsor: None,
        properties: CollectionPropertiesVec::truncate_from(properties),
        property_permissions: PropertiesPermissionsVec::truncate_from(
            property_permissions,
        ),
    }
}

pub fn create_property_key(key: &str) -> PropertyKey {
	let key_u8_raw: Vec<u8> = key.as_bytes().into();
	PropertyKey::truncate_from(key_u8_raw)
}

pub fn create_property(key: &str, value: &str) -> Property {
	let key = create_property_key(key);

	let value_u8_raw: Vec<u8> = value.as_bytes().into();
	let value = PropertyValue::truncate_from(value_u8_raw);

	Property{ key, value }
}

pub fn create_properties(keys: Vec<&str>, values: Vec<&str>) -> Vec<Property> {
	assert!(
		keys.len() == values.len(),
		"Lengths must be the same"
	);

	let mut properties = <Vec<Property>>::with_capacity(keys.len());

	for (key, value) in keys.iter().zip(values) {
		properties.push(create_property(key, value));
	}

	properties
}

pub fn default_token_balances() -> Vec<(AccountId, u128)> {
	vec![(ALICE, 100),(BOB, 150)]
}

pub fn default_token_properties() -> Vec<Property> {
	create_properties(
		vec!["PropertyKey1", "PropertyKey2"], 
		vec!["PropertyValue1", "PropertyValue2"]
	)
}