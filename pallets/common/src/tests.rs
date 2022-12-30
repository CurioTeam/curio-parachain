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

#![cfg(test)]

use frame_support::{
    assert_ok, assert_noop
};

use pallet_whitelist::Error as WhitelistError;

use crate::mock::*;
use crate::{
    CollectionById, CreatedCollectionCount, DestroyedCollectionCount, Error,
    CollectionHandle, AdminAmount, CollectionProperties, IsAdmin
};

// TODO: different init parameters
#[test]
pub fn init_collection_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let data = default_create_collection_data::<MockRuntime>();
        let flags = default_collection_flags();
        let prev_collection_count = CreatedCollectionCount::<MockRuntime>::get();

        let id = Common::init_collection(ADMIN_1, ADMIN_1, data.clone(), flags)
            .expect("Init collection failed");

        let collection = CollectionById::<MockRuntime>::get(id).expect("Collection not found");
        let collectoin_count = CreatedCollectionCount::<MockRuntime>::get();

        assert_eq!(collectoin_count.0, prev_collection_count.0 + 1);
        assert_eq!(collection.owner, ADMIN_1);
        assert_eq!(collection.limits, CollectionLimits::default());
        assert_eq!(collection.sponsorship, SponsorshipState::Disabled);
        assert_eq!(collection.permissions, CollectionPermissions::default());

        System::assert_last_event(RuntimeEvent::Common(crate::Event::CollectionCreated(
            id,
            data.mode.id(),
            ADMIN_1,
        )));
    });
}

#[test]
pub fn init_collection_fails_if_not_whitelist_admin() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let data = default_create_collection_data::<MockRuntime>();

        let flags = default_collection_flags();

        assert_noop!(
            Common::init_collection(ALICE, ALICE, data.clone(), flags),
            Error::<MockRuntime>::NoPermission
        );
    });
}

#[test]
pub fn init_collection_fails_if_not_sufficient_founds() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10 * DOLLARS)])
    .build()
    .execute_with(|| {
        let data = default_create_collection_data::<MockRuntime>();
        let flags = default_collection_flags();

        assert_noop!(
            Common::init_collection(ADMIN_1, ADMIN_1, data.clone(), flags),
            Error::<MockRuntime>::NotSufficientFounds
        );
    });
}

#[test]
pub fn destroy_collection_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");
        let collection_handle = CollectionHandle::new(collection_id).expect("Collection not found");
        let prev_destroyed_count = DestroyedCollectionCount::<MockRuntime>::get();

        Common::destroy_collection(collection_handle, &ADMIN_1).expect("Collection destroing failed");

        let destroyed_count = DestroyedCollectionCount::<MockRuntime>::get();

        assert_eq!(destroyed_count.0, prev_destroyed_count.0 + 1);
        assert_eq!(CollectionById::<MockRuntime>::contains_key(collection_id), false);
        assert_eq!(AdminAmount::<MockRuntime>::contains_key(collection_id), false);
        assert_eq!(IsAdmin::<MockRuntime>::iter_key_prefix((collection_id,)).count(), 0);
        assert_eq!(CollectionProperties::<MockRuntime>::contains_key(collection_id), false);

        System::assert_last_event(RuntimeEvent::Common(crate::Event::CollectionDestroyed(collection_id)));
    });
}

#[test]
pub fn destroy_collection_fails_if_not_owner() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ADMIN_2, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");
        let collection_handle = CollectionHandle::new(collection_id).expect("Collection not found");

        assert_noop!(
            Common::destroy_collection(collection_handle, &ADMIN_2),
            Error::<MockRuntime>::NoPermission
        );
    });
}

#[test]
pub fn set_collection_property_if_property_exists_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let property = create_property("PropertyKey", "PropertyValue");
        
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .properties(vec![
                (property.clone(), true)
            ])
            .init(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");

        assert_eq!(
            <CollectionProperties<MockRuntime>>::get(collection_id).get(&property.key).unwrap(),
            &property.value
        );

        let property = create_property("PropertyKey", "NewPropertyValue");

        assert_ok!(Common::set_collection_property(&collection, &ADMIN_1, property.clone()));
        
        assert_eq!(
            <CollectionProperties<MockRuntime>>::get(collection_id).get(&property.key).unwrap(),
            &property.value
        );
        
        System::assert_last_event(RuntimeEvent::Common(crate::Event::CollectionPropertySet(
            collection_id, 
            property.key
        )));

    });
}

#[test]
pub fn set_collection_property_for_new_property_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let property = create_property("PropertyKey", "PropertyValue");
        
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .mutable_property_keys(
                vec![create_property_key("PropertyKey")]
            )
            .init(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");

        assert_eq!(
            <CollectionProperties<MockRuntime>>::get(collection_id).get(&property.key),
            None
        );

        let property = create_property("PropertyKey", "NewPropertyValue");

        assert_ok!(Common::set_collection_property(&collection, &ADMIN_1, property.clone()));
        
        assert_eq!(
            <CollectionProperties<MockRuntime>>::get(collection_id).get(&property.key).unwrap(),
            &property.value
        );
        
        System::assert_last_event(RuntimeEvent::Common(crate::Event::CollectionPropertySet(
            collection_id, 
            property.key
        )));

    });
}

#[test]
pub fn set_collection_property_fails_if_not_mutable() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let property = create_property("PropertyKey", "PropertyValue");
        
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");

        assert_eq!(
            <CollectionProperties<MockRuntime>>::get(collection_id).get(&property.key),
            None
        );

        assert_noop!(
            Common::set_collection_property(&collection, &ADMIN_1, property.clone()),
            Error::<MockRuntime>::UnmutableProperty
        );
    });
}

#[test]
pub fn set_collection_property_fails_if_not_collection_admin() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ADMIN_2, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let property = create_property("PropertyKey", "PropertyValue");
        
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .properties(vec![
                (property.clone(), true)
            ])
            .init(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");

        let property = create_property("PropertyKey", "NewPropertyValue");

        assert_noop!(
            Common::set_collection_property(&collection, &ADMIN_2, property.clone()),
            Error::<MockRuntime>::NoPermission
        );
    });
}

// Only simple check as it uses set_collection_property internally
#[test]
pub fn set_property_permission_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");
        let property = create_property("PropertyKey", "PropertyValue");
        let mut property_key_permission = PropertyKeyPermission {
            key: property.key.clone(),
            permission: PropertyPermission::mutable()
        };

        assert_eq!(
            Common::get_collection_property_permission(collection_id, &property.key),
            None
        );
        assert_ok!(Common::set_property_permission(&collection, &ADMIN_1, property_key_permission.clone()));
        System::assert_last_event(RuntimeEvent::Common(crate::Event::PropertyPermissionSet(
            collection_id, 
            property.key.clone()
        )));
        assert_eq!(
            Common::get_collection_property_permission(collection_id, &property.key).unwrap(),
            PropertyPermission::mutable()
        );
        
        assert_eq!(
            Common::get_collection_property(collection_id, &property.key),
            None
        );
        assert_ok!(Common::set_collection_property(&collection, &ADMIN_1, property.clone()));
        assert_eq!(
            Common::get_collection_property(collection_id, &property.key).unwrap(),
            property.value
        );
        
        property_key_permission.permission = PropertyPermission::unmutable();
        assert_ok!(Common::set_property_permission(&collection, &ADMIN_1, property_key_permission.clone()));
        System::assert_last_event(RuntimeEvent::Common(crate::Event::PropertyPermissionSet(
            collection_id, 
            property.key.clone()
        )));

        assert_noop!(
            Common::set_collection_property(&collection, &ADMIN_1, property.clone()),
            Error::<MockRuntime>::UnmutableProperty
        );
    });
}

#[test]
pub fn set_property_permission_fails_if_not_collection_owner() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ADMIN_2, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .admins(vec![ADMIN_2])
            .init(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");
        let property = create_property("PropertyKey", "PropertyValue");
        let property_key_permission = PropertyKeyPermission {
            key: property.key.clone(),
            permission: PropertyPermission::mutable()
        };

        assert_noop!(
            Common::set_property_permission(&collection, &ADMIN_2, property_key_permission.clone()),
            Error::<MockRuntime>::NoPermission
        );
    });
}

#[test]
pub fn delete_collection_property_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .mutable_property_keys(vec![
                create_property_key("PropertyKey"),
            ])
            .init(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");
        let property = create_property("PropertyKey", "PropertyValue");

        assert_ok!(Common::set_collection_property(&collection, &ADMIN_1, property.clone()));

        assert_eq!(
            <CollectionProperties<MockRuntime>>::get(collection_id).get(&property.key).unwrap(),
            &property.value
        );

        assert_ok!(Common::delete_collection_property(&collection, &ADMIN_1, property.key.clone()));

        assert_eq!(
            <CollectionProperties<MockRuntime>>::get(collection_id).get(&property.key),
            None
        );

        System::assert_last_event(RuntimeEvent::Common(crate::Event::CollectionPropertyDeleted(
            collection_id, 
            property.key
        )));
    });
}

#[test]
pub fn delete_collection_property_fails_if_not_admin() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ADMIN_2, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .mutable_property_keys(vec![
                create_property_key("PropertyKey"),
            ])
            .init(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");
        let property = create_property("PropertyKey", "PropertyValue");

        assert_ok!(Common::set_collection_property(&collection, &ADMIN_1, property.clone()));

        assert_eq!(
            <CollectionProperties<MockRuntime>>::get(collection_id).get(&property.key).unwrap(),
            &property.value
        );

        assert_noop!(
            Common::delete_collection_property(&collection, &ADMIN_2, property.key.clone()),
            Error::<MockRuntime>::NoPermission
        );
    });
}

// Only simple check as it uses delete_collection_property internally
#[test]
pub fn delete_collection_properties_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .mutable_property_keys(vec![
                create_property_key("PropertyKey1"),
                create_property_key("PropertyKey2"),
            ])
            .init(ADMIN_1)
            .expect("Collection init failed");

        let collection = CollectionHandle::new(collection_id).expect("Collection not found");
        let properties = create_properties(
            vec!["PropertyKey1", "PropertyKey2"], 
            vec!["PropertyValue1", "PropertyValue2"]
        );

        assert_ok!(Common::set_collection_properties(&collection, &ADMIN_1, properties.clone()));

        let keys: Vec<_> = properties.iter().map(|p| p.key.clone()).collect();

        assert_ok!(Common::delete_collection_properties(&collection, &ADMIN_1, keys.clone()));

        for key in keys {
            assert_eq!(
                <CollectionProperties<MockRuntime>>::get(collection_id).get(&key),
                None
            );
            
            System::assert_has_event(RuntimeEvent::Common(crate::Event::CollectionPropertyDeleted(
                collection_id, 
                key
            )));
        }
    });
}

#[test]
pub fn toggle_admin_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");

        assert_eq!(
            <IsAdmin<MockRuntime>>::get((collection_id, ADMIN_2)),
            false
        );
        assert_ok!(Common::toggle_admin(&collection, &ADMIN_1, &ADMIN_2, true));
        System::assert_last_event(RuntimeEvent::Common(crate::Event::AdminToggled(
            ADMIN_2, 
            true
        )));
        assert_eq!(
            <IsAdmin<MockRuntime>>::get((collection_id, ADMIN_2)),
            true
        );

        assert_ok!(Common::toggle_admin(&collection, &ADMIN_1, &ADMIN_2, false));
        System::assert_last_event(RuntimeEvent::Common(crate::Event::AdminToggled(
            ADMIN_2, 
            false
        )));
        assert_eq!(
            <IsAdmin<MockRuntime>>::get((collection_id, ADMIN_2)),
            false
        );
    });
}

#[test]
pub fn toggle_admin_fails_if_not_owner() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2, ADMIN_3])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ADMIN_2, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");

        assert_eq!(
            <IsAdmin<MockRuntime>>::get((collection_id, ADMIN_3)),
            false
        );

        assert_noop!(
            Common::toggle_admin(&collection, &ADMIN_2, &ADMIN_3, true),
            Error::<MockRuntime>::NoPermission
        );
    });
}

#[test]
pub fn toggle_admin_fails_if_not_whitelist_admin() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {
        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");
        let collection = CollectionHandle::new(collection_id).expect("Collection not found");

        assert_eq!(
            <IsAdmin<MockRuntime>>::get((collection_id, ALICE)),
            false
        );

        assert_noop!(
            Common::toggle_admin(&collection, &ADMIN_1, &ALICE, true),
            WhitelistError::<MockRuntime>::NotWhitelistAdmin
        );
    });
}