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

use pallet_common::Event as PalletEvent;
use pallet_refungible::CreateItemData;
use pallet_whitelist::{Investor, traits::WhitelistInterface};

use crate::{AccountId, Runtime, RuntimeEvent, RuntimeOrigin, System, Refungible, Whitelist};
use collection_primitives::{
	CollectionMode, CollectionId, CreateCollectionData,	PropertiesPermissionsVec, CollectionPropertiesVec,
	PropertyKeyPermission, Property, CollectionTokenPrefix, CollectionDescription, CollectionName,
	TokenId, MAX_ITEMS_PER_BATCH, MAX_PROPERTIES_PER_ITEM, PropertyValue, PropertyKey,
	MAX_COLLECTION_NAME_LENGTH, MAX_COLLECTION_DESCRIPTION_LENGTH, MAX_TOKEN_PREFIX_LENGTH,
	CollectionLimits, SponsoringRateLimit, MAX_PROPERTY_KEY_LENGTH, PropertyPermission
};

use codec::alloc::string::ToString;
use core::{convert::TryInto, iter::IntoIterator};
use sp_std::prelude::*;

use orml_benchmarking::runtime_benchmarks;
use super::utils::assert_last_event;

fn get_individual_property_key(i: usize) -> Vec<u8> {
	let mut vec:Vec<u8> = Vec::from([97u8; MAX_PROPERTY_KEY_LENGTH as usize]);
	let n: Vec<u8> = i.to_string().as_bytes().into();
	for j in 0..n.len(){
		vec[j] = n[j];
	}
	vec
}

fn get_individual_account_id(i: usize) -> [u8; 32] {
	let mut vec:Vec<u8> = Vec::from([100u8; 32 as usize]);
	let n: Vec<u8> = i.to_string().as_bytes().into();
	for j in 0..n.len(){
		vec[j] = n[j];
	}
	vec.try_into().unwrap()
}

fn property_permissions_from_size(size: usize) -> Vec<PropertyKeyPermission> {
    let mut property_premissions = Vec::<PropertyKeyPermission>::with_capacity(size.into());

	for i in 0..size {
		let vec = get_individual_property_key(i);
		property_premissions.push(PropertyKeyPermission {
			key: PropertyKey::truncate_from(vec),
			permission: PropertyPermission {
				mutable: true,
			},
		});
	}

	property_premissions
}

fn property_from_size(size: usize) -> Vec<Property> {
    let mut property = Vec::<Property>::with_capacity(size);

	for i in 0..size {
		let vec = get_individual_property_key(i);
		property.push(Property {
			key: PropertyKey::truncate_from(vec.clone()),
			value: PropertyValue::truncate_from(vec)});
	}

	property
}

fn property_key_from_size(size: usize) -> Vec<PropertyKey> {
    let mut property_key = Vec::<PropertyKey>::with_capacity(size.into());

	for i in 0..size {
		let vec = get_individual_property_key(i);
		property_key.push(PropertyKey::truncate_from(vec));
	}

	property_key
}

fn default_create_collection_data(property_premissions_size: usize, property_size: usize) -> CreateCollectionData<AccountId> {

	let limits = CollectionLimits {
		account_token_ownership_limit: None,
		sponsored_data_size: Some(100),
		sponsored_data_rate_limit: Some(SponsoringRateLimit::Blocks(100)),
		token_limit: None,
		sponsor_transfer_timeout: Some(100),
		sponsor_approve_timeout: Some(100),
		owner_can_transfer: Some(true),
		owner_can_destroy: Some(true),
		transfers_enabled: Some(true),
	};

	CreateCollectionData::<AccountId> {
        mode: CollectionMode::ReFungible,
        name: CollectionName::truncate_from(Vec::from([98u16; MAX_COLLECTION_NAME_LENGTH as usize])),
        description: CollectionDescription::truncate_from(Vec::from([98u16; MAX_COLLECTION_DESCRIPTION_LENGTH as usize])),
        token_prefix: CollectionTokenPrefix::truncate_from(Vec::from([98u8; MAX_TOKEN_PREFIX_LENGTH as usize])),
        pending_sponsor: None,
        limits: Some(limits),
        property_permissions: PropertiesPermissionsVec::truncate_from(
            property_permissions_from_size(property_premissions_size),
        ),
        properties: CollectionPropertiesVec::truncate_from(property_from_size(property_size)),
    }
}

fn create_item_data(users: Vec<(AccountId, u128)>, property_keys: Vec<Property>) -> CreateItemData<AccountId> {
	CreateItemData::<AccountId> {
		balances: users,
		properties: property_keys
	}
}

fn get_collection_id_from_last_event() -> CollectionId {
    match System::events().last().unwrap().event {
        RuntimeEvent::Common(PalletEvent::CollectionCreated(id, _, _)) => {
            id
        },
        _ => {
            panic!("Unexpected event");
        }
    }
}

fn get_token_id_from_last_event() -> TokenId {
	for event_record in System::events() {
		match event_record.event {
			RuntimeEvent::Common(PalletEvent::ItemCreated(_, token_id, _, _)) => {
				return token_id;
			},
			_ => {}
		}
	}
	
	panic!("ItemCreated event not found");
}

fn create_vec_of_users_with_balances(size: usize) -> Vec<(AccountId, u128)> {
	let mut users = Vec::<(AccountId, u128)>::with_capacity(size.into());
	for i in 0..size {
		let account_id = create_investor(i);
		users.push((account_id, 1));
	}
	users
}

fn create_vec_of_one_user_with_balance(user_number: usize, balance: u128) -> Vec<(AccountId, u128)> {
	let account_id = create_investor(user_number);
	vec![(account_id, balance)]
}

fn create_admin(account_number: usize) -> AccountId {
	let account_id = AccountId::new(get_individual_account_id(account_number));
	if !Whitelist::is_admin(account_id.clone()) {
		Whitelist::add_admin(RuntimeOrigin::root(), account_id.clone()).unwrap();
	}
	account_id
}

fn create_investor(account_number: usize) -> AccountId {
	let admin = create_admin(100);
	let account_id = get_individual_account_id(account_number);
	let account_vec = vec![(account_id.clone(), Investor {
		account: AccountId::new(account_id.clone()), 
		is_active:  true
	})];
	if !Whitelist::is_active_investor(&AccountId::new(account_id.clone())) {
		Whitelist::add_investors(RuntimeOrigin::signed(admin.clone()), account_vec).unwrap();
	}
	AccountId::new(account_id)
}

fn default_init_collection(admin: &AccountId) -> CollectionId {
	let data = default_create_collection_data(
		MAX_PROPERTIES_PER_ITEM as usize, 
		MAX_PROPERTIES_PER_ITEM as usize
	);

	Refungible::init_collection(RuntimeOrigin::signed(admin.clone()), data).unwrap();
	get_collection_id_from_last_event()
}

runtime_benchmarks! {
    {Runtime, pallet_refungible}
	
    init_collection {
		let i in 1..MAX_PROPERTIES_PER_ITEM.into();
		let j in 1..MAX_PROPERTIES_PER_ITEM.into();
		let account_id = create_admin(0);
		let data = default_create_collection_data(i as usize, j as usize);		
    }: _(RuntimeOrigin::signed(account_id.clone()), data.clone())
	verify {
		let collection_id = get_collection_id_from_last_event();
		assert_last_event(PalletEvent::CollectionCreated(collection_id, CollectionMode::ReFungible.id(), account_id).into());
	}

	destroy_collection {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);
	}: _(RuntimeOrigin::signed(account_id), collection_id.clone())
	verify {
		assert_last_event(PalletEvent::CollectionDestroyed(collection_id).into());
	}

	set_collection_property {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);
		let properties = property_from_size(1);
	}: _(RuntimeOrigin::signed(account_id), collection_id.clone(), properties[0].clone())
	verify {
		assert_last_event(PalletEvent::CollectionPropertySet(collection_id, properties[0].key.clone()).into());
	}

	delete_collection_property {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);
		let property_keys = property_key_from_size(1);
	}: _(RuntimeOrigin::signed(account_id), collection_id.clone(), property_keys[0].clone())
	verify {
		assert_last_event(PalletEvent::CollectionPropertyDeleted(collection_id, property_keys[0].clone()).into());
	}

	set_collection_properties {
		let k in 1..MAX_PROPERTIES_PER_ITEM.into();
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);
		let properties = property_from_size(k as usize);
	}: _(RuntimeOrigin::signed(account_id), collection_id.clone(), properties.clone())
	verify {
		for i in 0..k {
			System::assert_has_event(PalletEvent::CollectionPropertySet(collection_id, properties[i as usize].clone().key).into())
		}
	}

	delete_collection_properties {
		let k in 1..MAX_PROPERTIES_PER_ITEM.into();
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);
		let property_keys = property_key_from_size(k as usize);
	}: _(RuntimeOrigin::signed(account_id), collection_id.clone(), property_keys.clone())
	verify {
		for i in 0..k {
			System::assert_has_event(PalletEvent::CollectionPropertyDeleted(collection_id, property_keys[i as usize].clone()).into())
		}
	}

	set_property_permission {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);
		let properties = property_permissions_from_size(1);
	}: _(RuntimeOrigin::signed(account_id), collection_id.clone(), properties[0].clone())
	verify {
		assert_last_event(PalletEvent::PropertyPermissionSet(collection_id, properties[0].key.clone()).into());
	}

	create_item {
		let l in 1..MAX_ITEMS_PER_BATCH; //Max users per item
		let p in 1..MAX_PROPERTIES_PER_ITEM; //Max amount of property

		System::set_block_number(1);
		let account_id = create_admin(0);

		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_users_with_balances(l as usize);
		let property_keys = property_from_size(p as usize);

		let data = create_item_data(users, property_keys);
	}: _(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data.clone())
	verify {
		let token_id = get_token_id_from_last_event();
		for i in 0..l {
			System::assert_has_event(PalletEvent::ItemCreated(collection_id, token_id, data.balances[i as usize].0.clone(), 1).into())
		}
	}

	create_max_item {
		let l = MAX_ITEMS_PER_BATCH;//Max users per item
		let p = MAX_PROPERTIES_PER_ITEM;//Max amount of property

		System::set_block_number(1);
		let account_id = create_admin(0);

		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_users_with_balances(l as usize);
		let property_keys = property_from_size(p as usize);

		let data = create_item_data(users, property_keys);
	}: {Refungible::create_item(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data.clone())}
	verify {
		let token_id = get_token_id_from_last_event();
		for i in 0..l {
			System::assert_has_event(PalletEvent::ItemCreated(collection_id, token_id, data.balances[i as usize].0.clone(), 1).into())
		}
	}
	
	set_token_property {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_users_with_balances(MAX_ITEMS_PER_BATCH as usize);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);

		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();
		let property = Property {
			key: PropertyKey::truncate_from(get_individual_property_key(1)),
			value: PropertyValue::truncate_from(get_individual_property_key(1))
		};
	}: _(RuntimeOrigin::signed(account_id), collection_id.clone(), token_id.clone(), property.clone())
	verify {
		assert_last_event(PalletEvent::TokenPropertySet(collection_id, token_id, property.key).into());
	}

	set_token_properties {
		let i in 1..MAX_PROPERTIES_PER_ITEM;
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_users_with_balances(MAX_ITEMS_PER_BATCH as usize);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);

		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();
		let property = property_from_size(i as usize);
	}: _(RuntimeOrigin::signed(account_id), collection_id.clone(), token_id.clone(), property.clone())
	verify {
		for j in 0..i {
			System::assert_has_event(PalletEvent::TokenPropertySet(collection_id, token_id, property[j as usize].clone().key).into())
		}
	}

	delete_token_property {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_users_with_balances(MAX_ITEMS_PER_BATCH as usize);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);
		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();
		let property = PropertyKey::truncate_from(get_individual_property_key(1));
	}: _(RuntimeOrigin::signed(account_id), collection_id.clone(), token_id.clone(), property.clone())
	verify {
		assert_last_event(PalletEvent::TokenPropertyDeleted(collection_id, token_id, property).into());
	}

	delete_token_properties {
		let i in 1..MAX_PROPERTIES_PER_ITEM;
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_users_with_balances(MAX_ITEMS_PER_BATCH as usize);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);
		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();
		let property = property_key_from_size(i as usize);
	}: _(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), token_id.clone(), property.clone())
	verify {
		for j in 0..i {
			System::assert_has_event(PalletEvent::TokenPropertyDeleted(collection_id, token_id, property[j as usize].clone()).into())
		}
	}

	transfer {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_users_with_balances(MAX_ITEMS_PER_BATCH as usize);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);
		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();

		let to = create_admin(1);

	}: _(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), token_id.clone(), to.clone(), 1)
	verify {
		assert_last_event(PalletEvent::Transfer(collection_id, token_id, account_id, to, 1).into());
	}

	set_allowance {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_users_with_balances(MAX_ITEMS_PER_BATCH as usize);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);
		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();

		let sender = create_admin(1);

	}: _(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), token_id.clone(), sender.clone(), 1)
	verify {
		assert_last_event(PalletEvent::Approved(collection_id, token_id, account_id, sender, 1).into());
	}

	transfer_from {
		System::set_block_number(1);
		let from = create_admin(0);
		let collection_id = default_init_collection(&from);

		let users = create_vec_of_users_with_balances(MAX_ITEMS_PER_BATCH as usize);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);
		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(from.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();

		let who = create_admin(1);

		let to = create_admin(2);

		Refungible::set_allowance(RuntimeOrigin::signed(from.clone()), collection_id.clone(), token_id.clone(), who.clone(), 1).unwrap();

	}: _(RuntimeOrigin::signed(who), collection_id.clone(), token_id.clone(), from.clone(), to.clone(), 1)
	verify {
		System::assert_has_event(PalletEvent::Transfer(collection_id, token_id, from, to, 1).into());
	}

	burn {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_users_with_balances(MAX_ITEMS_PER_BATCH as usize);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);
		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();

	}: _(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), token_id.clone(), 1)
	verify {
		assert_last_event(PalletEvent::ItemDestroyed(collection_id, token_id, account_id, 1).into());
	}

	burn_from {
		System::set_block_number(1);
		let from = create_admin(0);
		let collection_id = default_init_collection(&from);

		let users = create_vec_of_users_with_balances(MAX_ITEMS_PER_BATCH as usize);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);
		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(from.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();
		let who = create_admin(1);
		Refungible::set_allowance(RuntimeOrigin::signed(who.clone()), collection_id.clone(), token_id.clone(), from.clone(), 1).unwrap();


	}: _(RuntimeOrigin::signed(from.clone()), collection_id.clone(), token_id.clone(), who.clone(), 1)
	verify {
		System::assert_has_event(PalletEvent::ItemDestroyed(collection_id, token_id, who, 1).into());
	}

	repartition {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_one_user_with_balance(0, 100);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);
		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();

	}: _(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), token_id.clone(), 1)
	verify {
		assert_last_event(PalletEvent::ItemDestroyed(collection_id, token_id, account_id, 99).into());
	}

	toggle_admin {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);

		let users = create_vec_of_users_with_balances(MAX_ITEMS_PER_BATCH as usize);
		let property_keys = property_from_size(MAX_PROPERTIES_PER_ITEM as usize);
		let data = create_item_data(users, property_keys);
		Refungible::create_item(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), data).unwrap();

		let token_id = get_token_id_from_last_event();

		let person = create_admin(1);

	}: _(RuntimeOrigin::signed(account_id), collection_id, person.clone(), true)
	verify {
		assert_last_event(PalletEvent::AdminToggled(person, true).into());
	}

	set_sponsor {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);
		let sponsor = AccountId::new(get_individual_account_id(1));
	}: _(RuntimeOrigin::signed(account_id), collection_id, sponsor.clone())
	verify {
		assert_last_event(PalletEvent::SponsorSet(collection_id, sponsor).into());
	}

	confirm_sponsorship {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);
		let sponsor = AccountId::new(get_individual_account_id(1));
		Refungible::set_sponsor(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), sponsor.clone()).unwrap();
	}: _(RuntimeOrigin::signed(sponsor.clone()), collection_id.clone())
	verify {
		assert_last_event(PalletEvent::SponsorhipConfirmed(collection_id, sponsor).into());
	}

	remove_sponsor {
		System::set_block_number(1);
		let account_id = create_admin(0);
		let collection_id = default_init_collection(&account_id);
		let sponsor = AccountId::new(get_individual_account_id(1));
		Refungible::set_sponsor(RuntimeOrigin::signed(account_id.clone()), collection_id.clone(), sponsor.clone()).unwrap();
		Refungible::confirm_sponsorship(RuntimeOrigin::signed(sponsor), collection_id.clone()).unwrap();
	}: _(RuntimeOrigin::signed(account_id), collection_id.clone())
	verify {
		assert_last_event(PalletEvent::SponsorshipRemoved(collection_id).into());
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use orml_benchmarking::impl_benchmark_test_suite;
    use crate::benchmarking::utils::tests::new_test_ext;

    impl_benchmark_test_suite!(new_test_ext(),);
}