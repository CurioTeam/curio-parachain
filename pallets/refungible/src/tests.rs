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

use crate::mock::*;
use crate::{
    Error,
    TotalSupply, Balance, Allowance,
    Owned, AccountBalance,
    TokensMinted, TokensBurnt,
    TokenProperties
};

// Simple redirection to corresponding common pallet's method
// So most of testing are made there
#[test]
pub fn init_collection_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let data = default_create_collection_data::<MockRuntime>();
        let flags = default_collection_flags();

        assert_ok!(Refungible::init_collection(RuntimeOrigin::signed(ADMIN_1), data.clone(), flags));
    });
}

#[test]
pub fn init_collection_fails_if_not_wl_admin() {
    ExtBuilder::new()
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let data = default_create_collection_data::<MockRuntime>();
        let flags = default_collection_flags();

        assert_noop!(
            Refungible::init_collection(RuntimeOrigin::signed(ADMIN_1), data.clone(), flags),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

// Simple redirection to corresponding common pallet's method
// So most of testing are made there
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

        assert_ok!(Refungible::destroy_collection(RuntimeOrigin::signed(ADMIN_1), collection_id));

        assert_eq!(<TokensMinted<MockRuntime>>::contains_key(collection_id), false);
        assert_eq!(<TokensBurnt<MockRuntime>>::contains_key(collection_id), false);
        assert_eq!(<TotalSupply<MockRuntime>>::iter_prefix_values((collection_id,)).next(), None);
        assert_eq!(<Balance<MockRuntime>>::iter_prefix_values((collection_id,)).next(), None);
        assert_eq!(<Allowance<MockRuntime>>::iter_prefix_values((collection_id,)).next(), None);
        assert_eq!(<Owned<MockRuntime>>::iter_prefix_values((collection_id,)).next(), None);
        assert_eq!(<AccountBalance<MockRuntime>>::iter_prefix_values((collection_id,)).next(), None);
    });
}

#[test]
pub fn destroy_collection_fails_if_not_owner() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        assert_noop!(
            Refungible::destroy_collection(RuntimeOrigin::signed(ADMIN_2), collection_id),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

#[test]
pub fn destroy_collection_fails_if_collection_has_tokens() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let _ = TokenInitializer::new()
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_noop!(
            Refungible::destroy_collection(RuntimeOrigin::signed(ADMIN_1), collection_id),
            CommonError::<MockRuntime>::CantDestroyNotEmptyCollection
        );
    });
}

// Simple redirection to corresponding common pallet's method
// So most of testing are made there
#[test]
pub fn set_collection_property_works() {
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

        let property = create_property("PropertyKey", "PropertyValue");

        assert_ok!(Refungible::set_collection_property(RuntimeOrigin::signed(ADMIN_1), collection_id, property));
    });
}

#[test]
pub fn set_collection_property_fails_if_not_collection_admin() {
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

        let property = create_property("PropertyKey", "PropertyValue");

        assert_noop!(
            Refungible::set_collection_property(RuntimeOrigin::signed(ADMIN_2), collection_id, property),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

// Simple redirection to corresponding common pallet's method
// So most of testing are made there
#[test]
pub fn delete_collection_property_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .properties(vec![
                (create_property("PropertyKey", "PropertyValue"), false),
            ])
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let key = create_property_key("PropertyKey");

        assert_ok!(Refungible::delete_collection_property(RuntimeOrigin::signed(ADMIN_1), collection_id, key));
    });
}

#[test]
pub fn delete_collection_property_fails_if_not_collection_admin() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ADMIN_2, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .properties(vec![
                (create_property("PropertyKey", "PropertyValue"), false),
            ])
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let key = create_property_key("PropertyKey");

        assert_noop!(
            Refungible::delete_collection_property(RuntimeOrigin::signed(ADMIN_2), collection_id, key),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

// Simple redirection to corresponding common pallet's method
// So most of testing are made there
#[test]
pub fn set_collection_properties_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .mutable_property_keys(vec![
                create_property_key("PropertyKey1"),
                create_property_key("PropertyKey2")
            ])
            .init(ADMIN_1)
            .expect("Collection init failed");

        let properties = create_properties(
            vec!["PropertyKey1", "PropertyKey2"], 
            vec!["PropertyValue1", "PropertyValue2"]
        );

        assert_ok!(Refungible::set_collection_properties(RuntimeOrigin::signed(ADMIN_1), collection_id, properties));
    });
}

#[test]
pub fn set_collection_properties_fails_if_not_collection_admin() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ADMIN_2, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let properties = create_properties(
            vec!["PropertyKey1", "PropertyKey2"], 
            vec!["PropertyValue1", "PropertyValue2"]
        );

        assert_noop!(
            Refungible::set_collection_properties(RuntimeOrigin::signed(ADMIN_2), collection_id, properties),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

// Simple redirection to corresponding common pallet's method
// So most of testing are made there
#[test]
pub fn delete_collection_properties_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .properties(vec![
                (create_property("PropertyKey1", "PropertyValue1"), false),
                (create_property("PropertyKey2", "PropertyValue2"), false)
            ])
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let keys = vec![
            create_property_key("PropertyKey1"),
            create_property_key("PropertyKey2")
        ];

        assert_ok!(Refungible::delete_collection_properties(RuntimeOrigin::signed(ADMIN_1), collection_id, keys));
    });
}

#[test]
pub fn delete_collection_properties_fails_if_not_collection_admin() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ADMIN_2, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .properties(vec![
                (create_property("PropertyKey1", "PropertyValue1"), false),
                (create_property("PropertyKey2", "PropertyValue2"), false)
            ])
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let keys = vec![
            create_property_key("PropertyKey1"),
            create_property_key("PropertyKey2")
        ];

        assert_noop!(
            Refungible::delete_collection_properties(RuntimeOrigin::signed(ADMIN_2), collection_id, keys),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

// Simple redirection to corresponding common pallet's method
// So most of testing are made there
#[test]
pub fn set_property_permission_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let property = create_property("PropertyKey", "PropertyValue");

        assert_noop!(
            Refungible::set_collection_property(RuntimeOrigin::signed(ADMIN_1), collection_id, property.clone()),
            CommonError::<MockRuntime>::UnmutableProperty
        );

        let property_permission = PropertyKeyPermission {
            key: property.key.clone(),
            permission: PropertyPermission::mutable()
        };

        assert_ok!(Refungible::set_property_permission(RuntimeOrigin::signed(ADMIN_1), collection_id, property_permission));

        assert_ok!(Refungible::set_collection_property(RuntimeOrigin::signed(ADMIN_1), collection_id, property));
    });
}

// Simple redirection to create_multiple_items
// so most of testing made there
#[test]
pub fn create_item_works() {
    ExtBuilder::new()
    .investors(vec![ALICE, BOB])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let user_balances = vec![
            (ALICE, 100),
            (BOB, 150)
        ];
        let token_properties = create_properties(
            vec!["PropertyKey1", "PropertyKey2"], 
            vec!["PropertyValue1", "PropertyValue2"]
        );

        assert_ok!(Refungible::create_item(RuntimeOrigin::signed(ADMIN_1), collection_id, user_balances.clone(), token_properties));
    });
}

#[test]
pub fn create_multiples_items_works() {
    ExtBuilder::new()
    .investors(vec![ALICE, BOB])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let users_balances = vec![
            vec![(ALICE, 100),(BOB, 150)],
            vec![(ALICE, 50),(BOB, 250)]
        ];
        let tokens_properties = vec![
            create_properties(
                vec!["PropertyKey1", "PropertyKey2"], 
                vec!["PropertyValue1", "PropertyValue2"]
            ),
            create_properties(
                vec!["PropertyKey3", "PropertyKey4"], 
                vec!["PropertyValue3", "PropertyValue4"]
            )
        ];

        assert_ok!(Refungible::create_multiple_items(RuntimeOrigin::signed(ADMIN_1), collection_id, users_balances.clone(), tokens_properties.clone()));

        let last_token_id = <TokensMinted<MockRuntime>>::get(collection_id);
        let token_ids = vec![last_token_id - 1, last_token_id];
        

        for (i, token_id) in token_ids.iter().enumerate() {
            let token_id = TokenId(*token_id);

            for property in &tokens_properties[i] {
                System::assert_has_event(RuntimeEvent::Common(CommonEvent::TokenPropertySet(
                    collection_id, 
                    token_id, 
                    property.key.clone()
                )));
            }

            for (user, balance) in users_balances[i].clone() {
                System::assert_has_event(RuntimeEvent::Common(CommonEvent::ItemCreated(
                    collection_id, 
                    token_id, 
                    user, 
                    balance
                )));
    
                assert_eq!(<Balance<MockRuntime>>::get((collection_id, token_id, user)), balance);
                assert_eq!(<Owned<MockRuntime>>::get((collection_id, user, token_id)), true);
            } 
        } 
    });
}

#[test]
pub fn create_multiples_items_works_for_collection_admins() {
    ExtBuilder::new()
    .investors(vec![ALICE, BOB])
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        assert_ok!(Refungible::toggle_admin(RuntimeOrigin::signed(ADMIN_1), collection_id, ADMIN_2, true));

        let users_balances = vec![
            vec![(ADMIN_1, 100),(ADMIN_2, 150)],
            vec![(ADMIN_1, 50),(ADMIN_2, 250)]
        ];

        assert_ok!(Refungible::create_multiple_items(RuntimeOrigin::signed(ADMIN_1), collection_id, users_balances.clone(), vec![vec![], vec![]]));
    });
}

#[test]
pub fn create_multiples_items_fails_if_user_duplicates_for_one_item_given() {
    ExtBuilder::new()
    .investors(vec![ALICE, BOB])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let users_balances = vec![
            vec![(ALICE, 100),(ALICE, 150)],
            vec![(ALICE, 50),(BOB, 250)]
        ];

        assert_noop!(
            Refungible::create_multiple_items(RuntimeOrigin::signed(ADMIN_1), collection_id, users_balances, vec![vec![], vec![]]),
            Error::<MockRuntime>::UserDuplicatesGiven
        ); 
    });
}

#[test]
pub fn create_multiples_items_fails_if_not_admin() {
    ExtBuilder::new()
    .investors(vec![ALICE, BOB])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let users_balances = vec![
            vec![(ALICE, 100),(BOB, 150)],
            vec![(ALICE, 50),(BOB, 250)]
        ];

        assert_noop!(
            Refungible::create_multiple_items(RuntimeOrigin::signed(ADMIN_2), collection_id, users_balances, vec![vec![], vec![]]),
            CommonError::<MockRuntime>::NoPermission
        ); 
    });
}

#[test]
pub fn create_multiples_items_fails_if_not_whitelisted_investor() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let users_balances = vec![
            vec![(ALICE, 100),(BOB, 150)],
            vec![(ALICE, 50),(BOB, 250)]
        ];

        assert_noop!(
            Refungible::create_multiple_items(RuntimeOrigin::signed(ADMIN_1), collection_id, users_balances, vec![vec![], vec![]]),
            Error::<MockRuntime>::NeitherWhitelistedNorCollectionAdmin
        ); 
    });
}

#[test]
pub fn set_token_property_works_if_property_exists_since_initialization() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .properties(vec![(create_property("Key", "Old value"), true)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        let new_property = create_property("Key", "New value");

        assert_ok!(Refungible::set_token_property(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id, 
            new_property.clone()
        ));

        System::assert_last_event(RuntimeEvent::Common(CommonEvent::TokenPropertySet(
            collection_id, 
            token_id, 
            new_property.key.clone()
        )));

        assert_eq!(
            TokenProperties::<MockRuntime>::get((collection_id, token_id)).get(&new_property.key).unwrap(),
            &new_property.value
        );
    });
}

#[test]
pub fn set_token_property_works_if_property_not_exists() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .mutable_property_keys(vec![
                create_property_key("PropertyKey")
            ])
            .init(ADMIN_1)
            .expect("Collection init failed");
        let token_id = TokenInitializer::new()
            .init(collection_id, ADMIN_1)
            .unwrap();
            
        let new_property = create_property("PropertyKey", "PropertyValue");

        assert_ok!(Refungible::set_token_property(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id, 
            new_property.clone()
        ));

        System::assert_last_event(RuntimeEvent::Common(CommonEvent::TokenPropertySet(
            collection_id, 
            token_id, 
            new_property.key.clone()
        )));

        assert_eq!(
            TokenProperties::<MockRuntime>::get((collection_id, token_id)).get(&new_property.key).unwrap(),
            &new_property.value
        );
    });
}

#[test]
pub fn set_token_property_fails_if_not_mutable() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .properties(vec![(create_property("Key", "Old value"), false)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        let new_property = create_property("Key", "New value");

        assert_noop!(
            Refungible::set_token_property(
                RuntimeOrigin::signed(ADMIN_1), 
                collection_id, 
                token_id, 
                new_property.clone()
            ),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

#[test]
pub fn set_token_property_fails_if_not_collection_admin() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ADMIN_2, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .properties(vec![(create_property("Key", "Old value"), false)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        let new_property = create_property("Key", "New value");

        assert_noop!(
            Refungible::set_token_property(
                RuntimeOrigin::signed(ADMIN_2), 
                collection_id, 
                token_id, 
                new_property.clone()
            ),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

// Tests to set_token_property also applicable to set_token_properties as
// set_token_property is simple redirection with one property
// So only simple check made
#[test]
pub fn set_token_properties_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .properties(vec![
                (create_property("First", "Old value"), true),
                (create_property("Second", "Old value"), true)
            ])
            .init(collection_id, ADMIN_1)
            .unwrap();

        let new_properties = vec![
            create_property("First", "New value"),
            create_property("Second", "New value")
        ];

        assert_ok!(Refungible::set_token_properties(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id, 
            new_properties.clone()
        ));

        for property in new_properties {
            System::assert_has_event(RuntimeEvent::Common(CommonEvent::TokenPropertySet(
                collection_id, 
                token_id, 
                property.key.clone()
            )));

            assert_eq!(
                TokenProperties::<MockRuntime>::get((collection_id, token_id)).get(&property.key).unwrap(),
                &property.value
            );
        }
    });
}

#[test]
pub fn delete_token_property_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let property = create_property("Key", "Value");

        let token_id = TokenInitializer::new()
            .properties(vec![(property.clone(), true)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_ok!(Refungible::delete_token_property(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id,
            property.key.clone()
        ));

        System::assert_last_event(RuntimeEvent::Common(CommonEvent::TokenPropertyDeleted(
            collection_id, 
            token_id, 
            property.key.clone()
        )));

        assert_eq!(
            TokenProperties::<MockRuntime>::get((collection_id, token_id)).get(&property.key),
            None
        );
    });
}

#[test]
pub fn delete_token_property_fails_if_not_collection_admin() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ADMIN_2, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let property = create_property("Key", "Value");

        let token_id = TokenInitializer::new()
            .properties(vec![(property.clone(), true)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_noop!(
            Refungible::delete_token_property(
                RuntimeOrigin::signed(ADMIN_2), 
                collection_id, 
                token_id, 
                property.key
            ),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

#[test]
pub fn delete_token_properties_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let properties = vec![
            create_property("First", "Value"),
            create_property("Second", "Value")
        ];

        let props_with_flag: Vec<(Property, bool)> = properties.iter()
            .map(|p| (p.clone(), true))
            .collect();

        let token_id = TokenInitializer::new()
            .properties(props_with_flag)
            .init(collection_id, ADMIN_1)
            .unwrap();

        let keys: Vec<PropertyKey> = properties.iter()
            .map(|p| p.key.clone())
            .collect();

        assert_ok!(Refungible::delete_token_properties(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id,
            keys
        ));

        for property in properties {
            System::assert_has_event(RuntimeEvent::Common(CommonEvent::TokenPropertyDeleted(
                collection_id, 
                token_id, 
                property.key.clone()
            )));
    
            assert_eq!(
                TokenProperties::<MockRuntime>::get((collection_id, token_id)).get(&property.key),
                None
            );
        }
    });
}

#[test]
pub fn transfer_works_between_whitelisted_investors() {
    ExtBuilder::new()
    .investors(vec![ALICE, BOB])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![
        (ADMIN_1, 10_000 * DOLLARS),
        (ALICE, 10_000 * DOLLARS),
    ])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ALICE)), 10);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, BOB)), 0);

        assert_ok!(Refungible::transfer(
            RuntimeOrigin::signed(ALICE), 
            collection_id, 
            token_id,
            BOB,
            10
        ));

        System::assert_last_event(RuntimeEvent::Common(CommonEvent::Transfer(
            collection_id, 
            token_id, 
            ALICE,
            BOB,
            10
        )));
        
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ALICE)), 0);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, BOB)), 10);
    });
}

pub fn transfer_works_for_collection_admins() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![
        (ADMIN_1, 10_000 * DOLLARS),
        (ADMIN_2, 10_000 * DOLLARS),
        (ALICE, 10_000 * DOLLARS),
    ])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![
                (ALICE, 10),
                (ADMIN_1, 10)
            ])
            .init(collection_id, ADMIN_1)
            .unwrap();

        // add second whitelist admin to collection admins
        assert_ok!(Refungible::toggle_admin(RuntimeOrigin::signed(ADMIN_1), collection_id, ADMIN_2, true));

        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ALICE)), 10);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ADMIN_1)), 10);

        // wl to admin
        assert_ok!(Refungible::transfer(
            RuntimeOrigin::signed(ALICE), 
            collection_id, 
            token_id,
            ADMIN_1,
            2
        ));

        // admin to wl
        assert_ok!(Refungible::transfer(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id,
            ALICE,
            3
        ));

        // admin to admin
        assert_ok!(Refungible::transfer(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id,
            ADMIN_2,
            3
        ));

        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ALICE)), 11);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ADMIN_1)), 6);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ADMIN_2)), 3);
    });
}

#[test]
pub fn transfer_fails_if_not_whitelisted_investor() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![
        (ADMIN_1, 10_000 * DOLLARS),
        (ALICE, 10_000 * DOLLARS),
    ])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        // not wl investor
        assert_noop!(
            Refungible::transfer(
                RuntimeOrigin::signed(ALICE), 
                collection_id, 
                token_id,
                BOB,
                10
            ),
            Error::<MockRuntime>::NeitherWhitelistedNorCollectionAdmin
        );

        // wl admin but not collection admin
        assert_noop!(
            Refungible::transfer(
                RuntimeOrigin::signed(ALICE), 
                collection_id, 
                token_id,
                ADMIN_2,
                10
            ),
            Error::<MockRuntime>::NeitherWhitelistedNorCollectionAdmin
        );
    });
}

#[test]
pub fn transfer_fails_if_not_sufficient_balance() {
    ExtBuilder::new()
    .investors(vec![ALICE, BOB])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![
        (ADMIN_1, 10_000 * DOLLARS),
        (ALICE, 10_000 * DOLLARS),
    ])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_noop!(
            Refungible::transfer(
                RuntimeOrigin::signed(ALICE), 
                collection_id, 
                token_id,
                BOB,
                11
            ),
            CommonError::<MockRuntime>::TokenValueTooLow
        );
    });
}

#[test]
pub fn set_allowance_works_between_whitelisted_investors() {
    ExtBuilder::new()
    .investors(vec![ALICE, BOB])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_ok!(Refungible::set_allowance(
            RuntimeOrigin::signed(ALICE), 
            collection_id, 
            token_id,
            BOB,
            5
        ));

        System::assert_last_event(RuntimeEvent::Common(CommonEvent::Approved(
            collection_id, 
            token_id, 
            ALICE,
            BOB,
            5
        )));

        assert_eq!(Allowance::<MockRuntime>::get((collection_id, token_id, ALICE, BOB)), 5);
    });
}

#[test]
pub fn set_allowance_works_for_collection_admins() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10), (ADMIN_1, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_ok!(Refungible::toggle_admin(RuntimeOrigin::signed(ADMIN_1), collection_id, ADMIN_2, true));
        
        // wl to collection admin
        assert_ok!(Refungible::set_allowance(
            RuntimeOrigin::signed(ALICE), 
            collection_id, 
            token_id,
            ADMIN_2,
            5
        ));

        // collection admin to wl
        assert_ok!(Refungible::set_allowance(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id,
            ALICE,
            5
        ));

        // collection admin to collection admin
        assert_ok!(Refungible::set_allowance(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id,
            ADMIN_2,
            5
        ));

        assert_eq!(Allowance::<MockRuntime>::get((collection_id, token_id, ALICE, ADMIN_2)), 5);
        assert_eq!(Allowance::<MockRuntime>::get((collection_id, token_id, ADMIN_1, ALICE)), 5);
        assert_eq!(Allowance::<MockRuntime>::get((collection_id, token_id, ADMIN_1, ADMIN_2)), 5);
    });
}

#[test]
pub fn set_allowance_fails_if_not_whitelisted_investors() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1, ADMIN_2])
    .balances(vec![(ADMIN_1, 1_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        // not wl
        assert_noop!(
            Refungible::set_allowance(
                RuntimeOrigin::signed(ALICE), 
                collection_id, 
                token_id,
                BOB,
                5
            ),
            Error::<MockRuntime>::NeitherWhitelistedNorCollectionAdmin
        );

        // wl admin but not collection admin 
        assert_noop!(
            Refungible::set_allowance(
                RuntimeOrigin::signed(ALICE), 
                collection_id, 
                token_id,
                ADMIN_2,
                5
            ),
            Error::<MockRuntime>::NeitherWhitelistedNorCollectionAdmin
        );
    });
}

// Make allowance check and use 'transfer' internaly
// So only success case check and fail with insufficient allowance case
#[test]
pub fn transfer_from_works() {
    ExtBuilder::new()
    .investors(vec![ALICE, BOB])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_ok!(Refungible::set_allowance(
            RuntimeOrigin::signed(ALICE), 
            collection_id, 
            token_id,
            BOB,
            5
        ));

        assert_eq!(Allowance::<MockRuntime>::get((collection_id, token_id, ALICE, BOB)), 5);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ALICE)), 10);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, BOB)), 0);

        assert_ok!(Refungible::transfer_from(
            RuntimeOrigin::signed(BOB), 
            collection_id, 
            token_id,
            ALICE,
            BOB,
            5
        ));

        assert_eq!(Allowance::<MockRuntime>::get((collection_id, token_id, ALICE, BOB)), 0);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ALICE)), 5);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, BOB)), 5);
    });
}

#[test]
pub fn transfer_from_fails_if_insufficient_allowance() {
    ExtBuilder::new()
    .investors(vec![ALICE, BOB])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_ok!(Refungible::set_allowance(
            RuntimeOrigin::signed(ALICE), 
            collection_id, 
            token_id,
            BOB,
            5
        ));

        assert_eq!(Allowance::<MockRuntime>::get((collection_id, token_id, ALICE, BOB)), 5);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ALICE)), 10);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, BOB)), 0);

        assert_noop!(
            Refungible::transfer_from(
                RuntimeOrigin::signed(BOB), 
                collection_id, 
                token_id,
                ALICE,
                BOB,
                6
            ),
            CommonError::<MockRuntime>::ApprovedValueTooLow
        );
    });
}

#[test]
pub fn burn_works_when_token_continue_to_exist() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ADMIN_1, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_eq!(TotalSupply::<MockRuntime>::get((collection_id, token_id)), 10);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ADMIN_1)), 10);

        assert_ok!(Refungible::burn(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id,
            5
        ));

        System::assert_last_event(RuntimeEvent::Common(CommonEvent::ItemDestroyed(
            collection_id, 
            token_id, 
            ADMIN_1,
            5
        )));

        assert_eq!(TotalSupply::<MockRuntime>::get((collection_id, token_id)), 5);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ADMIN_1)), 5);
    });
}

#[test]
pub fn burn_works_when_all_pieces_burned() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ADMIN_1, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_eq!(TotalSupply::<MockRuntime>::get((collection_id, token_id)), 10);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ADMIN_1)), 10);

        let token_burnt = TokensBurnt::<MockRuntime>::get(collection_id);
        let account_balance = AccountBalance::<MockRuntime>::get((collection_id, ADMIN_1));

        assert_ok!(Refungible::burn(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id,
            10
        ));

        System::assert_last_event(RuntimeEvent::Common(CommonEvent::ItemDestroyed(
            collection_id, 
            token_id, 
            ADMIN_1,
            10
        )));

        assert_eq!(<TokensBurnt<MockRuntime>>::get(collection_id), token_burnt.checked_add(1).unwrap());
        assert_eq!(<AccountBalance<MockRuntime>>::get((collection_id, ADMIN_1)), account_balance.checked_sub(1).unwrap());
        assert_eq!(<TotalSupply<MockRuntime>>::contains_key((collection_id, token_id)), false);
        assert_eq!(<Owned<MockRuntime>>::contains_key((collection_id, ADMIN_1, token_id)), false);
        assert_eq!(<TokenProperties<MockRuntime>>::contains_key((collection_id, token_id)), false);
        assert_eq!(<Balance<MockRuntime>>::iter_prefix_values((collection_id, token_id,)).next(), None);
        assert_eq!(<Allowance<MockRuntime>>::iter_prefix_values((collection_id, token_id,)).next(), None);
    });
}

#[test]
pub fn burn_fails_if_insufficient_balance() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ADMIN_1, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_noop!(
            Refungible::burn(
                RuntimeOrigin::signed(ADMIN_1), 
                collection_id, 
                token_id,
                11
            ),
            CommonError::<MockRuntime>::TokenValueTooLow
        );
    });
}

#[test]
pub fn burn_fails_if_not_collection_admin() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_noop!(
            Refungible::burn(
                RuntimeOrigin::signed(ALICE), 
                collection_id, 
                token_id,
                5
            ),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

#[test]
pub fn burn_from_works() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_ok!(Refungible::set_allowance(
            RuntimeOrigin::signed(ALICE), 
            collection_id, 
            token_id,
            ADMIN_1,
            5
        ));

        assert_ok!(Refungible::burn_from(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id,
            ALICE,
            5
        ));

        // Allowance is changed after burning, so ItemDestroyed is not the latest event
        System::assert_has_event(RuntimeEvent::Common(CommonEvent::ItemDestroyed(
            collection_id, 
            token_id, 
            ALICE,
            5
        )));

        assert_eq!(TotalSupply::<MockRuntime>::get((collection_id, token_id)), 5);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ALICE)), 5);
    });
}

#[test]
pub fn burn_from_fails_if_insufficient_allowance() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_ok!(Refungible::set_allowance(
            RuntimeOrigin::signed(ALICE), 
            collection_id, 
            token_id,
            ADMIN_1,
            5
        ));

        assert_noop!(
            Refungible::burn_from(
                RuntimeOrigin::signed(ADMIN_1), 
                collection_id, 
                token_id,
                ALICE,
                6
            ),
            CommonError::<MockRuntime>::ApprovedValueTooLow
        );
    });
}

#[test]
pub fn burn_from_fails_if_not_collection_admin() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_ok!(Refungible::set_allowance(
            RuntimeOrigin::signed(ALICE), 
            collection_id, 
            token_id,
            ADMIN_1,
            5
        ));

        assert_noop!(
            Refungible::burn_from(
                RuntimeOrigin::signed(ALICE), 
                collection_id, 
                token_id,
                ALICE,
                5
            ),
            CommonError::<MockRuntime>::NoPermission
        );
    });
}

#[test]
pub fn repartition_works() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ADMIN_1, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_ok!(Refungible::repartition(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            token_id,
            100
        ));

        System::assert_last_event(RuntimeEvent::Common(CommonEvent::ItemCreated(
            collection_id, 
            token_id, 
            ADMIN_1,
            90
        )));

        assert_eq!(TotalSupply::<MockRuntime>::get((collection_id, token_id)), 100);
        assert_eq!(Balance::<MockRuntime>::get((collection_id, token_id, ADMIN_1)), 100);
    });
}

#[test]
pub fn repartition_fails_if_there_are_other_holders() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ADMIN_1, 10), (ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_noop!(
            Refungible::repartition(
                RuntimeOrigin::signed(ADMIN_1), 
                collection_id, 
                token_id,
                100
           ),
           Error::<MockRuntime>::RepartitionWhileNotOwningAllPieces
        );
    });
}

#[test]
pub fn repartition_fails_if_not_collection_admin() {
    ExtBuilder::new()
    .investors(vec![ALICE])
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ALICE, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_noop!(
            Refungible::repartition(
                RuntimeOrigin::signed(ALICE), 
                collection_id, 
                token_id,
                100
           ),
           CommonError::<MockRuntime>::NoPermission
        );
    });
}

#[test]
pub fn repartition_fails_if_max_refungible_pieces_exceeded() {
    ExtBuilder::new()
    .wl_admins(vec![ADMIN_1])
    .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
    .build()
    .execute_with(|| {

        let collection_id = CollectionInitializer::<MockRuntime>::new()
            .init_default(ADMIN_1)
            .expect("Collection init failed");

        let token_id = TokenInitializer::new()
            .balances(vec![(ADMIN_1, 10)])
            .init(collection_id, ADMIN_1)
            .unwrap();

        assert_noop!(
            Refungible::repartition(
                RuntimeOrigin::signed(ADMIN_1), 
                collection_id, 
                token_id,
                MAX_REFUNGIBLE_PIECES.checked_add(1).unwrap()
           ),
           Error::<MockRuntime>::WrongRefungiblePieces
        );
    });
}

// Simple redirection to corresponding common pallet's method
// So most of testing made there
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

        assert_ok!(Refungible::toggle_admin(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            ADMIN_2,
            true
        ));

        assert_ok!(Refungible::toggle_admin(
            RuntimeOrigin::signed(ADMIN_1), 
            collection_id, 
            ADMIN_2,
            false
        ));
    });
}