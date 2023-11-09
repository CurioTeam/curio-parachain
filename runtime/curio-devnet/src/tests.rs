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
use frame_support::{ assert_ok, assert_err, dispatch::{ Dispatchable, GetDispatchInfo } };
use sp_runtime::{
    traits::SignedExtension,
    transaction_validity::{ TransactionValidityError, InvalidTransaction },
};
use crate::mock::*;
use pallet_charge_transaction::ChargeTransactionPayment;
use codec::Encode;
use pallet_common::collection_initializer::CollectionInitializer;

/// This function is used to do the real call to runtime with pre/post dispatch. \
/// `call` - the parameter that is directly responsible for runtime call. \
/// `from` - the parameter responsible for the account from which the runtime function is called
fn call_wrapper(call: RefCall<MockRuntime>, from: AccountId) {
    let call: <MockRuntime as frame_system::Config>::RuntimeCall = RuntimeCall::Refungible(call);
    let info = call.get_dispatch_info();
    let len = call.encode().len();
    let pre_d = <ChargeTransactionPayment<MockRuntime>>
        ::new(0)
        .pre_dispatch(&from, &call.clone(), &info, len)
        .expect("pre_dispatch error");
    let post_result = call.dispatch(RuntimeOrigin::signed(from)).expect("dispatch failure");
    assert_ok!(
        ChargeTransactionPayment::<MockRuntime>::post_dispatch(
            Some(pre_d),
            &info,
            &post_result,
            len,
            &Ok(())
        )
    );
}
/// This function is used for testing. Checking the event BalancesEvent::Withdraw. \
/// `checking_who` - the parameter with which the real account from which the funds were withdrawn is compared
fn event_check_wrapper(checking_who: AccountId) {
    let events = System::events();
    let who = events
        .iter()
        .rev()
        .find_map(|rec| {
            if let RuntimeEvent::Balances(BalancesEvent::Withdraw { who, amount: _ }) = &rec.event {
                Some(who)
            } else {
                None
            }
        })
        .expect("unable to find from who were withdrawed fees");
    assert_eq!(*who, checking_who);
}

fn create_string(size: usize) -> String {
    let mut s = String::with_capacity(size);
    for _ in 0..size {
        s.push('a');
    }
    s
}

#[test]
pub fn fees_withdrawed_from_sponsor_not_from_sender() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE])
        .wl_admins(vec![ADMIN_1])
        .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS)])
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(ALICE, 10), (BOB, 5)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call = RefCall::transfer {
                collection_id: collection_id,
                token_id: token_id,
                to: CHARLIE,
                amount: 5,
            };
            call_wrapper(call, BOB);
            event_check_wrapper(ALICE);
        });
}

#[test]
pub fn fees_withdrawed_from_sender_if_sponsor_was_unconfirmed() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE])
        .wl_admins(vec![ADMIN_1])
        .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (BOB, 10_000 * DOLLARS)])
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, false)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(ALICE, 10), (BOB, 5)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call = RefCall::transfer {
                collection_id: collection_id,
                token_id: token_id,
                to: CHARLIE,
                amount: 5,
            };
            call_wrapper(call, BOB);
            event_check_wrapper(BOB);
        });
}

#[test]
pub fn fees_withdrawed_from_sender_if_sponsor_was_disabled() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE])
        .wl_admins(vec![ADMIN_1])
        .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (BOB, 10_000 * DOLLARS)])
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_default(ADMIN_1)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(ALICE, 10), (BOB, 5)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call = RefCall::transfer {
                collection_id: collection_id,
                token_id: token_id,
                to: CHARLIE,
                amount: 5,
            };
            call_wrapper(call, BOB);
            event_check_wrapper(BOB);
        });
}

#[test]
pub fn transfer_fails_if_sponsor_have_not_enought_money() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE])
        .wl_admins(vec![ADMIN_1])
        .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(ALICE, 10), (BOB, 5)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call: <MockRuntime as frame_system::Config>::RuntimeCall = RuntimeCall::Refungible(
                RefCall::transfer {
                    collection_id: collection_id,
                    token_id: token_id,
                    to: CHARLIE,
                    amount: 5,
                }
            );
            let info = call.get_dispatch_info();
            let len = call.encode().len();

            assert_err!(
                <ChargeTransactionPayment<MockRuntime>>
                    ::new(0)
                    .pre_dispatch(&BOB, &call.clone(), &info, len),
                TransactionValidityError::Invalid(InvalidTransaction::Payment)
            );
        });
}

#[test]
pub fn transfer_check_limits() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE])
        .wl_admins(vec![ADMIN_1])
        .balances(
            vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS), (BOB, 10_000 * DOLLARS)]
        )
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(ALICE, 10), (BOB, 10)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call_1 = RefCall::transfer {
                collection_id: collection_id,
                token_id: token_id,
                to: CHARLIE,
                amount: 5,
            };
            call_wrapper(call_1, BOB);
            event_check_wrapper(ALICE);

            let call_2 = RefCall::transfer {
                collection_id: collection_id,
                token_id: token_id,
                to: CHARLIE,
                amount: 5,
            };
            call_wrapper(call_2, BOB);
            event_check_wrapper(BOB);
        });
}

#[test]
pub fn sponsoring_transfer_works_after_timeout() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE])
        .wl_admins(vec![ADMIN_1])
        .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS)])
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(ALICE, 10), (BOB, 10)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call_1 = RefCall::transfer {
                collection_id: collection_id,
                token_id: token_id,
                to: CHARLIE,
                amount: 5,
            };
            call_wrapper(call_1, BOB);
            event_check_wrapper(ALICE);

            System::set_block_number(6);

            let call_2 = RefCall::transfer {
                collection_id: collection_id,
                token_id: token_id,
                to: CHARLIE,
                amount: 5,
            };
            call_wrapper(call_2, BOB);
            event_check_wrapper(ALICE);
        });
}

#[test]
pub fn transfer_limit_independent_for_different_users_on_same_token() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE])
        .wl_admins(vec![ADMIN_1])
        .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS)])
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(BOB, 10), (CHARLIE, 10)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call_from_bob_to_charlie = RefCall::transfer {
                collection_id: collection_id,
                token_id: token_id,
                to: CHARLIE,
                amount: 5,
            };
            call_wrapper(call_from_bob_to_charlie, BOB);
            event_check_wrapper(ALICE);

            let call_from_charlie_to_bob = RefCall::transfer {
                collection_id: collection_id,
                token_id: token_id,
                to: BOB,
                amount: 5,
            };
            call_wrapper(call_from_charlie_to_bob, CHARLIE);
            event_check_wrapper(ALICE);
        });
}

#[test]
pub fn set_allowance_check_limits() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE, DAVE])
        .wl_admins(vec![ADMIN_1])
        .balances(
            vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS), (BOB, 10_000 * DOLLARS)]
        )
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(BOB, 20), (CHARLIE, 10), (DAVE, 10)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call_1 = RefCall::set_allowance {
                collection_id: collection_id,
                token_id: token_id,
                spender: CHARLIE,
                amount: 5,
            };
            call_wrapper(call_1, BOB);
            event_check_wrapper(ALICE);

            let call_2 = RefCall::set_allowance {
                collection_id: collection_id,
                token_id: token_id,
                spender: DAVE,
                amount: 5,
            };
            call_wrapper(call_2, BOB);
            event_check_wrapper(BOB);
        });
}

#[test]
pub fn sponsoring_set_allowance_works_after_timeout() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE, DAVE])
        .wl_admins(vec![ADMIN_1])
        .balances(
            vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS)]
        )
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(BOB, 20), (CHARLIE, 10), (DAVE, 10)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call_1 = RefCall::set_allowance {
                collection_id: collection_id,
                token_id: token_id,
                spender: CHARLIE,
                amount: 5,
            };
            call_wrapper(call_1, BOB);
            event_check_wrapper(ALICE);

            System::set_block_number(6);

            let call_2 = RefCall::set_allowance {
                collection_id: collection_id,
                token_id: token_id,
                spender: DAVE,
                amount: 5,
            };
            call_wrapper(call_2, BOB);
            event_check_wrapper(ALICE);
        });
}

#[test]
pub fn set_allowance_limit_is_applied_for_all_tokens() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE, DAVE])
        .wl_admins(vec![ADMIN_1])
        .balances(
            vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS), (BOB, 10_000 * DOLLARS)]
        )
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id_1 = TokenInitializer::new()
                .balances(vec![(BOB, 10)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call_1 = RefCall::set_allowance {
                collection_id: collection_id,
                token_id: token_id_1,
                spender: CHARLIE,
                amount: 5,
            };
            call_wrapper(call_1, BOB);
            event_check_wrapper(ALICE);

            let token_id_2 = TokenInitializer::new()
                .balances(vec![(BOB, 15)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let call_2 = RefCall::set_allowance {
                collection_id: collection_id,
                token_id: token_id_2,
                spender: DAVE,
                amount: 10,
            };
            call_wrapper(call_2, BOB);
            event_check_wrapper(BOB);
        });
}

#[test]
pub fn transfer_from_check_limits() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE, DAVE])
        .wl_admins(vec![ADMIN_1])
        .balances(
            vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS), (BOB, 10_000 * DOLLARS)]
        )
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(BOB, 10), (CHARLIE, 20), (DAVE, 10)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let set_allowance_call = RefCall::set_allowance {
                collection_id: collection_id,
                token_id: token_id,
                spender: BOB,
                amount: 15,
            };
            call_wrapper(set_allowance_call, CHARLIE);

            let transfer_from_call_1 = RefCall::transfer_from {
                collection_id: collection_id,
                token_id: token_id,
                from: CHARLIE,
                to: DAVE,
                amount: 5,
            };
            call_wrapper(transfer_from_call_1, BOB);
            event_check_wrapper(ALICE);

            let transfer_from_call_2 = RefCall::transfer_from {
                collection_id: collection_id,
                token_id: token_id,
                from: CHARLIE,
                to: DAVE,
                amount: 5,
            };
            call_wrapper(transfer_from_call_2, BOB);
            event_check_wrapper(BOB);
        });
}

#[test]
pub fn sponsoring_transfer_from_works_after_timeout() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE, DAVE])
        .wl_admins(vec![ADMIN_1])
        .balances(
            vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS)]
        )
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(BOB, 10), (CHARLIE, 20), (DAVE, 10)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let set_allowance_call = RefCall::set_allowance {
                collection_id: collection_id,
                token_id: token_id,
                spender: BOB,
                amount: 15,
            };
            call_wrapper(set_allowance_call, CHARLIE);

            let transfer_from_call_1 = RefCall::transfer_from {
                collection_id: collection_id,
                token_id: token_id,
                from: CHARLIE,
                to: DAVE,
                amount: 5,
            };
            call_wrapper(transfer_from_call_1, BOB);
            event_check_wrapper(ALICE);

            System::set_block_number(6);

            let transfer_from_call_2 = RefCall::transfer_from {
                collection_id: collection_id,
                token_id: token_id,
                from: CHARLIE,
                to: DAVE,
                amount: 5,
            };
            call_wrapper(transfer_from_call_2, BOB);
            event_check_wrapper(ALICE);
        });
}

#[test]
pub fn transfer_from_limit_independent_for_different_users_on_same_token() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE, DAVE])
        .wl_admins(vec![ADMIN_1])
        .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS)])
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let token_id = TokenInitializer::new()
                .balances(vec![(BOB, 10), (CHARLIE, 10), (DAVE, 10)])
                .init(collection_id, ADMIN_1)
                .unwrap();

            let set_allowance_call_1 = RefCall::set_allowance {
                collection_id: collection_id,
                token_id: token_id,
                spender: BOB,
                amount: 5,
            };
            call_wrapper(set_allowance_call_1, CHARLIE);

            let transfer_from_call_1 = RefCall::transfer_from {
                collection_id: collection_id,
                token_id: token_id,
                from: CHARLIE,
                to: DAVE,
                amount: 5,
            };
            call_wrapper(transfer_from_call_1, BOB);
            event_check_wrapper(ALICE);

            let set_allowance_call_2 = RefCall::set_allowance {
                collection_id: collection_id,
                token_id: token_id,
                spender: CHARLIE,
                amount: 5,
            };
            call_wrapper(set_allowance_call_2, DAVE);

            let transfer_from_call_2 = RefCall::transfer_from {
                collection_id: collection_id,
                token_id: token_id,
                from: DAVE,
                to: BOB,
                amount: 5,
            };
            call_wrapper(transfer_from_call_2, CHARLIE);
            event_check_wrapper(ALICE);
        });
}

#[test]
pub fn create_item_check_limits() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE, DAVE])
        .wl_admins(vec![ADMIN_1])
        .balances(
            vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS), (BOB, 10_000 * DOLLARS)]
        )
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let user_balances_1 = vec![(ALICE, 10), (BOB, 15)];
            let token_properties_1 = create_properties(
                vec!["PropertyKey1", "PropertyKey2"],
                vec!["PropertyValue1", "PropertyValue2"]
            );
            let data_1 = CreateItemData::<AccountId> {
                balances: user_balances_1,
                properties: token_properties_1,
            };
            let create_item_call_1 = RefCall::create_item {
                collection_id: collection_id,
                data: data_1,
            };
            call_wrapper(create_item_call_1, ADMIN_1);
            event_check_wrapper(ALICE);

            let user_balances_2 = vec![(CHARLIE, 10), (DAVE, 15)];
            let token_properties_2 = create_properties(
                vec!["PropertyKey1", "PropertyKey2"],
                vec!["PropertyValue1", "PropertyValue2"]
            );
            let data_2 = CreateItemData::<AccountId> {
                balances: user_balances_2,
                properties: token_properties_2,
            };
            let create_item_call_2 = RefCall::create_item {
                collection_id: collection_id,
                data: data_2,
            };
            call_wrapper(create_item_call_2, ADMIN_1);
            event_check_wrapper(ADMIN_1);
        });
}

#[test]
pub fn sponsoring_create_item_works_after_timeout() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE, DAVE])
        .wl_admins(vec![ADMIN_1])
        .balances(
            vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS), (BOB, 10_000 * DOLLARS)]
        )
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let user_balances_1 = vec![(ALICE, 10), (BOB, 15)];
            let token_properties_1 = create_properties(
                vec!["PropertyKey1", "PropertyKey2"],
                vec!["PropertyValue1", "PropertyValue2"]
            );
            let data_1 = CreateItemData::<AccountId> {
                balances: user_balances_1,
                properties: token_properties_1,
            };
            let create_item_call_1 = RefCall::create_item {
                collection_id: collection_id,
                data: data_1,
            };
            call_wrapper(create_item_call_1, ADMIN_1);
            event_check_wrapper(ALICE);

            System::set_block_number(6);

            let user_balances_2 = vec![(CHARLIE, 10), (DAVE, 15)];
            let token_properties_2 = create_properties(
                vec!["PropertyKey1", "PropertyKey2"],
                vec!["PropertyValue1", "PropertyValue2"]
            );
            let data_2 = CreateItemData::<AccountId> {
                balances: user_balances_2,
                properties: token_properties_2,
            };
            let create_item_call_2 = RefCall::create_item {
                collection_id: collection_id,
                data: data_2,
            };
            call_wrapper(create_item_call_2, ADMIN_1);
            event_check_wrapper(ALICE);
        });
}

#[test]
pub fn create_item_limit_independent_for_different_collections() {
    ExtBuilder::new()
        .investors(vec![ALICE, BOB, CHARLIE, DAVE])
        .wl_admins(vec![ADMIN_1])
        .balances(
            vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS), (BOB, 10_000 * DOLLARS)]
        )
        .build()
        .execute_with(|| {
            let collection_id_1 = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");

            let user_balances_1 = vec![(ALICE, 10), (BOB, 15)];
            let token_properties_1 = create_properties(
                vec!["PropertyKey1", "PropertyKey2"],
                vec!["PropertyValue1", "PropertyValue2"]
            );
            let data_1 = CreateItemData::<AccountId> {
                balances: user_balances_1,
                properties: token_properties_1,
            };
            let create_item_call_1 = RefCall::create_item {
                collection_id: collection_id_1,
                data: data_1,
            };
            call_wrapper(create_item_call_1, ADMIN_1);
            event_check_wrapper(ALICE);

            let collection_id_2 = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");
            let user_balances_2 = vec![(CHARLIE, 10), (DAVE, 15)];
            let token_properties_2 = create_properties(
                vec!["PropertyKey1", "PropertyKey2"],
                vec!["PropertyValue1", "PropertyValue2"]
            );
            let data_2 = CreateItemData::<AccountId> {
                balances: user_balances_2,
                properties: token_properties_2,
            };
            let create_item_call_2 = RefCall::create_item {
                collection_id: collection_id_2,
                data: data_2,
            };
            call_wrapper(create_item_call_2, ADMIN_1);
            event_check_wrapper(ALICE);
        });
}

#[test]
pub fn set_token_property_time_limits_check() {
    ExtBuilder::new()
        .investors(vec![ALICE])
        .wl_admins(vec![ADMIN_1])
        .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS)])
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");
            let token_id = TokenInitializer::new()
                .properties(
                    vec![
                        (create_property("Key1", "OldValue1"), true),
                        (create_property("Key2", "OldValue2"), true)
                    ]
                )
                .init(collection_id, ADMIN_1)
                .unwrap();
            let properties = vec![
                create_property("Key1", "NewValue1"),
                create_property("Key2", "NewValue2")
            ];

            let set_property_call_1 = RefCall::set_token_properties {
                collection_id: collection_id,
                token_id: token_id,
                properties: properties,
            };
            call_wrapper(set_property_call_1, ADMIN_1);
            event_check_wrapper(ALICE);

            let new_properties = vec![
                create_property("Key1", "NewValue3"),
                create_property("Key2", "NewValue4")
            ];
            let set_property_call_2 = RefCall::set_token_properties {
                collection_id: collection_id,
                token_id: token_id,
                properties: new_properties,
            };
            call_wrapper(set_property_call_2, ADMIN_1);
            event_check_wrapper(ADMIN_1);
        });
}

#[test]
pub fn sponsoring_set_token_property_works_after_timeout() {
    ExtBuilder::new()
        .investors(vec![ALICE])
        .wl_admins(vec![ADMIN_1])
        .balances(vec![(ADMIN_1, 10_000 * DOLLARS), (ALICE, 10_000 * DOLLARS)])
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");
            let token_id = TokenInitializer::new()
                .properties(
                    vec![
                        (create_property("Key1", "OldValue1"), true),
                        (create_property("Key2", "OldValue2"), true)
                    ]
                )
                .init(collection_id, ADMIN_1)
                .unwrap();
            let properties = vec![
                create_property("Key1", "NewValue1"),
                create_property("Key2", "NewValue2")
            ];

            let set_property_call_1 = RefCall::set_token_properties {
                collection_id: collection_id,
                token_id: token_id,
                properties: properties,
            };
            call_wrapper(set_property_call_1, ADMIN_1);
            event_check_wrapper(ALICE);

            System::set_block_number(6);

            let new_properties = vec![
                create_property("Key1", "NewValue3"),
                create_property("Key2", "NewValue4")
            ];
            let set_property_call_2 = RefCall::set_token_properties {
                collection_id: collection_id,
                token_id: token_id,
                properties: new_properties,
            };
            call_wrapper(set_property_call_2, ADMIN_1);
            event_check_wrapper(ALICE);
        });
}

#[test]
pub fn set_token_property_size_limits_check() {
    ExtBuilder::new()
        .investors(vec![ALICE])
        .wl_admins(vec![ADMIN_1])
        .balances(vec![(ADMIN_1, 10_000 * DOLLARS)])
        .build()
        .execute_with(|| {
            let collection_id = CollectionInitializer::<MockRuntime>
                ::new()
                .init_with_sponsor(ALICE, ADMIN_1, true)
                .expect("Collection init failed");
            let token_id = TokenInitializer::new()
                .properties(vec![(create_property("Key1", "OldValue1"), true)])
                .init(collection_id, ADMIN_1)
                .unwrap();
            let str = &create_string(2050);
            let properties = vec![create_property("Key1", str)];

            let set_property_call = RefCall::set_token_properties {
                collection_id: collection_id,
                token_id: token_id,
                properties: properties,
            };
            call_wrapper(set_property_call, ADMIN_1);
            event_check_wrapper(ADMIN_1);
        });
}
