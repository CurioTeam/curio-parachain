#![cfg(test)]

use frame_support::{
    assert_ok,
    assert_noop,
    dispatch::{
        DispatchError
    }
};
use orml_traits::MultiCurrency;
use crate::{
    CurrencyMap,
    Event,
    Error,
    EthBlacklisted,
    InRequestStatus,
    OutRequestId,
    NativeBridgedSupply,
    Managers,
    MAX_BATCH_SIZE,
    MintData,
    Paused,
    SubBlacklisted,
    mock::{
        ALICE,
        Bridge,
        BOB,
        CHARLIE,
        Currencies,
        CGT,
        CGT_ERC20,
        CGT_DECIMALS,
        DAI,
        DAI_ERC20,
        DAI_DECIMALS,
        ETH_ADDRESS_1,
        ExtBuilder,
        RuntimeEvent,
        RuntimeOrigin,
        MockRuntime,
        System,
    }
};

#[test]
fn mint_works() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_ok!(Bridge::mint(RuntimeOrigin::signed(ALICE), 0, BOB, DAI_ERC20(), 100 * DAI_DECIMALS));
        assert_eq!(InRequestStatus::<MockRuntime>::get(&0), true);
        assert_eq!(Currencies::total_balance(DAI, &BOB), 100 * DAI_DECIMALS);
        System::assert_last_event(RuntimeEvent::Bridge(Event::Mint {
            request_id: 0,
            to: BOB,
            token: DAI_ERC20(),
            currency_id: DAI,
            amount: 100 * DAI_DECIMALS
        }));
    });
}

#[test]
fn mint_works_native_currency() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_ok!(Bridge::mint(RuntimeOrigin::signed(ALICE), 0, BOB, CGT_ERC20(), 100 * CGT_DECIMALS));
        assert_eq!(InRequestStatus::<MockRuntime>::get(&0), true);
        assert_eq!(Currencies::total_balance(CGT, &BOB), 100 * CGT_DECIMALS);
        assert_eq!(NativeBridgedSupply::<MockRuntime>::get(), 100 * CGT_DECIMALS);
        System::assert_last_event(RuntimeEvent::Bridge(Event::Mint {
            request_id: 0,
            to: BOB,
            token: CGT_ERC20(),
            currency_id: CGT,
            amount: 100 * CGT_DECIMALS
        }));
    });
}

#[test]
fn mint_fails_sender_is_not_a_manager() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::mint(RuntimeOrigin::signed(BOB), 0, ALICE, CGT_ERC20(), 100 * CGT_DECIMALS),
            Error::<MockRuntime>::SenderNotBridgeManager
        );
    });
}

#[test]
fn mint_fails_if_paused() {
    ExtBuilder::new()
    .full_pause()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::mint(RuntimeOrigin::signed(ALICE), 0, BOB, CGT_ERC20(), 100 * CGT_DECIMALS),
            Error::<MockRuntime>::BridgePaused
        );
    });
}

#[test]
fn mint_fails_if_currency_not_supported() {
    ExtBuilder::new()
    .currencies(vec![])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::mint(RuntimeOrigin::signed(ALICE), 0, BOB, CGT_ERC20(), 100 * CGT_DECIMALS),
            Error::<MockRuntime>::CurrencyNotSupported
        );
    });
}

#[test]
fn mint_fails_if_currency_paused() {
    ExtBuilder::new()
    .currencies(vec![(CGT, CGT_ERC20(), true)])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::mint(RuntimeOrigin::signed(ALICE), 0, BOB, CGT_ERC20(), 100 * CGT_DECIMALS),
            Error::<MockRuntime>::CurrencyNotActive
        );
    });
}

#[test]
fn mint_fails_if_receiver_blacklisted() {
    ExtBuilder::new()
    .blacklist_sub(vec![BOB])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::mint(RuntimeOrigin::signed(ALICE), 0, BOB, CGT_ERC20(), 100 * CGT_DECIMALS),
            Error::<MockRuntime>::SubAccountBlacklisted
        );
    });
}

#[test]
fn mint_fails_if_given_request_id_was_processed() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_ok!(Bridge::mint(RuntimeOrigin::signed(ALICE), 0, BOB, CGT_ERC20(), 100 * CGT_DECIMALS));
        assert_noop!(
            Bridge::mint(RuntimeOrigin::signed(ALICE), 0, BOB, CGT_ERC20(), 100 * CGT_DECIMALS),
            Error::<MockRuntime>::RequestAlreadyProcessed
        );
    });
}

#[test]
fn batch_mint_works() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        let data = vec![
            MintData {
                request_id: 0,
                to: BOB,
                token: CGT_ERC20(),
                amount: 100 * CGT_DECIMALS
            },
            MintData {
                request_id: 1,
                to: CHARLIE,
                token: DAI_ERC20(),
                amount: 100 * DAI_DECIMALS
            },
        ];
        let currencies = vec![CGT, DAI];
        assert_ok!(Bridge::batch_mint(RuntimeOrigin::signed(ALICE), data.clone()));
        for (index, mint_data) in data.iter().enumerate() {
            assert_eq!(InRequestStatus::<MockRuntime>::get(&mint_data.request_id), true);
            assert_eq!(Currencies::total_balance(currencies[index], &mint_data.to), mint_data.amount);

            System::assert_has_event(RuntimeEvent::Bridge(Event::Mint {
                request_id: mint_data.request_id,
                to: mint_data.to,
                token: mint_data.token,
                currency_id: currencies[index],
                amount: mint_data.amount,
            }));
        }
    });
}

#[test]
fn batch_mint_noop_if_one_of_mints_fails() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        let data = vec![
            MintData {
                request_id: 0,
                to: BOB,
                token: CGT_ERC20(),
                amount: 100 * CGT_DECIMALS
            },
            MintData {
                request_id: 0,
                to: CHARLIE,
                token: DAI_ERC20(),
                amount: 100 * DAI_DECIMALS
            },
        ];
        assert_noop!(
            Bridge::batch_mint(RuntimeOrigin::signed(ALICE), data.clone()),
            Error::<MockRuntime>::RequestAlreadyProcessed
        );
    });
}

#[test]
fn batch_mint_fails_if_batch_size_limit_exceeded() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        let data = (0..=MAX_BATCH_SIZE).map(|index| {
            MintData {
                request_id: index as u128,
                to: BOB,
                token: CGT_ERC20(),
                amount: 100 * DAI_DECIMALS
            }
        }).collect();

        assert_noop!(
            Bridge::batch_mint(RuntimeOrigin::signed(ALICE), data),
            Error::<MockRuntime>::MaxBatchSizeExceeded
        );
    });
}

#[test]
fn batch_mint_fails_sender_is_not_a_manager() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        let data = vec![
            MintData {
                request_id: 0,
                to: BOB,
                token: CGT_ERC20(),
                amount: 100 * CGT_DECIMALS
            },
            MintData {
                request_id: 1,
                to: CHARLIE,
                token: DAI_ERC20(),
                amount: 100 * DAI_DECIMALS
            },
        ];
        assert_noop!(
            Bridge::batch_mint(RuntimeOrigin::signed(BOB), data.clone()),
            Error::<MockRuntime>::SenderNotBridgeManager
        );
    });
}

#[test]
fn batch_mint_fails_if_paused() {
    ExtBuilder::new()
    .full_pause()
    .build()
    .execute_with(|| {
        let data = vec![
            MintData {
                request_id: 0,
                to: BOB,
                token: CGT_ERC20(),
                amount: 100 * CGT_DECIMALS
            },
            MintData {
                request_id: 1,
                to: CHARLIE,
                token: DAI_ERC20(),
                amount: 100 * DAI_DECIMALS
            },
        ];
        assert_noop!(
            Bridge::batch_mint(RuntimeOrigin::signed(ALICE), data.clone()),
            Error::<MockRuntime>::BridgePaused
        );
    });
}

#[test]
fn batch_mint_fails_if_currency_not_supported() {
    ExtBuilder::new()
    .currencies(vec![])
    .build()
    .execute_with(|| {
        let data = vec![
            MintData {
                request_id: 0,
                to: BOB,
                token: CGT_ERC20(),
                amount: 100 * CGT_DECIMALS
            },
            MintData {
                request_id: 1,
                to: CHARLIE,
                token: DAI_ERC20(),
                amount: 100 * DAI_DECIMALS
            },
        ];
        assert_noop!(
            Bridge::batch_mint(RuntimeOrigin::signed(ALICE), data.clone()),
            Error::<MockRuntime>::CurrencyNotSupported
        );
    });
}

#[test]
fn batch_mint_fails_if_currency_paused() {
    ExtBuilder::new()
    .currencies(vec![
        (CGT, CGT_ERC20(), false),
        (DAI, DAI_ERC20(), true)
    ])
    .build()
    .execute_with(|| {
        let data = vec![
            MintData {
                request_id: 0,
                to: BOB,
                token: CGT_ERC20(),
                amount: 100 * CGT_DECIMALS
            },
            MintData {
                request_id: 1,
                to: CHARLIE,
                token: DAI_ERC20(),
                amount: 100 * DAI_DECIMALS
            },
        ];
        assert_noop!(
            Bridge::batch_mint(RuntimeOrigin::signed(ALICE), data.clone()),
            Error::<MockRuntime>::CurrencyNotActive
        );
    });
}

#[test]
fn batch_mint_fails_if_receiver_blacklisted() {
    ExtBuilder::new()
    .blacklist_sub(vec![BOB])
    .build()
    .execute_with(|| {
        let data = vec![
            MintData {
                request_id: 0,
                to: BOB,
                token: CGT_ERC20(),
                amount: 100 * CGT_DECIMALS
            },
            MintData {
                request_id: 1,
                to: CHARLIE,
                token: DAI_ERC20(),
                amount: 100 * DAI_DECIMALS
            },
        ];
        assert_noop!(
            Bridge::batch_mint(RuntimeOrigin::signed(ALICE), data.clone()),
            Error::<MockRuntime>::SubAccountBlacklisted
        );
    });
}

#[test]
fn batch_mint_fails_if_given_request_id_was_processed() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        let data = vec![
            MintData {
                request_id: 0,
                to: BOB,
                token: CGT_ERC20(),
                amount: 100 * CGT_DECIMALS
            },
            MintData {
                request_id: 0,
                to: CHARLIE,
                token: DAI_ERC20(),
                amount: 100 * DAI_DECIMALS
            },
        ];
        assert_noop!(
            Bridge::batch_mint(RuntimeOrigin::signed(ALICE), data.clone()),
            Error::<MockRuntime>::RequestAlreadyProcessed
        );
    });
}

#[test]
fn burn_works() {
    ExtBuilder::new()
    .balances(vec![
        (BOB, DAI, 100 * DAI_DECIMALS)
    ])
    .build()
    .execute_with(|| {
        assert_eq!(OutRequestId::<MockRuntime>::get(), 0);
        assert_eq!(Currencies::total_balance(DAI, &BOB), 100 * DAI_DECIMALS);
        assert_ok!(Bridge::burn(RuntimeOrigin::signed(BOB), DAI, ETH_ADDRESS_1(), 100 * DAI_DECIMALS));
        assert_eq!(OutRequestId::<MockRuntime>::get(), 1);
        assert_eq!(Currencies::total_balance(DAI, &BOB), 0);
        System::assert_last_event(RuntimeEvent::Bridge(Event::Burn {
            request_id: 0,
            from: BOB,
            to: ETH_ADDRESS_1(),
            token: DAI_ERC20(),
            currency_id: DAI,
            amount: 100 * DAI_DECIMALS
        }));
    });
}

#[test]
fn burn_works_native_currency() {
    ExtBuilder::new()
    .balances(vec![
        (BOB, CGT, 100 * CGT_DECIMALS)
    ])
    .native_bridged_supply(100 * CGT_DECIMALS)
    .build()
    .execute_with(|| {
        assert_eq!(OutRequestId::<MockRuntime>::get(), 0);
        assert_eq!(NativeBridgedSupply::<MockRuntime>::get(), 100 * CGT_DECIMALS);
        assert_eq!(Currencies::total_balance(CGT, &BOB), 100 * CGT_DECIMALS);
        assert_ok!(Bridge::burn(RuntimeOrigin::signed(BOB), CGT, ETH_ADDRESS_1(), 100 * CGT_DECIMALS));
        assert_eq!(OutRequestId::<MockRuntime>::get(), 1);
        assert_eq!(NativeBridgedSupply::<MockRuntime>::get(), 0);
        assert_eq!(Currencies::total_balance(CGT, &BOB), 0);
        System::assert_last_event(RuntimeEvent::Bridge(Event::Burn {
            request_id: 0,
            from: BOB,
            to: ETH_ADDRESS_1(),
            token: CGT_ERC20(),
            currency_id: CGT,
            amount: 100 * CGT_DECIMALS
        }));
    });
}

#[test]
fn burn_fails_if_insufficient_native_supply() {
    ExtBuilder::new()
    .balances(vec![
        (BOB, CGT, 100 * CGT_DECIMALS)
    ])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::burn(RuntimeOrigin::signed(BOB), CGT, ETH_ADDRESS_1(), 100 * CGT_DECIMALS),
            Error::<MockRuntime>::InsufficientNativeBridged
        );
    });
}

#[test]
fn burn_fails_if_paused() {
    ExtBuilder::new()
    .full_pause()
    .balances(vec![
        (BOB, DAI, 100 * DAI_DECIMALS)
    ])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::burn(RuntimeOrigin::signed(BOB), DAI, ETH_ADDRESS_1(), 100 * DAI_DECIMALS),
            Error::<MockRuntime>::BridgePaused
        );
    });
}

#[test]
fn burn_fails_if_currency_not_supported() {
    ExtBuilder::new()
    .currencies(vec![
        (CGT, CGT_ERC20(), false)
    ])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::burn(RuntimeOrigin::signed(BOB), DAI, ETH_ADDRESS_1(), 100 * DAI_DECIMALS),
            Error::<MockRuntime>::CurrencyNotSupported
        );
    });
}

#[test]
fn burn_fails_if_currency_paused() {
    ExtBuilder::new()
    .currencies_pause(vec![DAI])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::burn(RuntimeOrigin::signed(BOB), DAI, ETH_ADDRESS_1(), 100 * DAI_DECIMALS),
            Error::<MockRuntime>::CurrencyNotActive
        );
    });
}

#[test]
fn burn_fails_if_sender_blacklisted() {
    ExtBuilder::new()
    .blacklist_sub(vec![BOB])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::burn(RuntimeOrigin::signed(BOB), DAI, ETH_ADDRESS_1(), 100 * DAI_DECIMALS),
            Error::<MockRuntime>::SubAccountBlacklisted
        );
    });
}

#[test]
fn burn_fails_if_receiver_blacklisted() {
    ExtBuilder::new()
    .blacklist_eth(vec![ETH_ADDRESS_1()])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::burn(RuntimeOrigin::signed(BOB), DAI, ETH_ADDRESS_1(), 100 * DAI_DECIMALS),
            Error::<MockRuntime>::EthAccountBlacklisted
        );
    });
}

#[test]
fn set_full_pause_works() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_eq!(Paused::<MockRuntime>::get(), false);
        assert_ok!(Bridge::set_full_pause(RuntimeOrigin::signed(ALICE), true));
        System::assert_last_event(RuntimeEvent::Bridge(Event::Paused));
        assert_eq!(Paused::<MockRuntime>::get(), true);
        assert_ok!(Bridge::set_full_pause(RuntimeOrigin::signed(ALICE), false));
        System::assert_last_event(RuntimeEvent::Bridge(Event::Unpaused));
        assert_eq!(Paused::<MockRuntime>::get(), false);
    });
}

#[test]
fn set_full_pause_fails_if_sender_is_not_a_manager() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_full_pause(RuntimeOrigin::signed(BOB), true),
            Error::<MockRuntime>::SenderNotBridgeManager
        );
    });
}

#[test]
fn set_full_pause_fails_if_already_paused() {
    ExtBuilder::new()
    .full_pause()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_full_pause(RuntimeOrigin::signed(ALICE), true),
            Error::<MockRuntime>::AlreadyPaused
        );
    });
}

#[test]
fn set_full_pause_fails_if_already_not_paused() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_full_pause(RuntimeOrigin::signed(ALICE), false),
            Error::<MockRuntime>::AlreadyNotPaused
        );
    });
}

#[test]
fn set_currency_pause_works() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_eq!(CurrencyMap::<MockRuntime>::get(DAI).unwrap().1, false);
        assert_ok!(Bridge::set_currency_pause(RuntimeOrigin::signed(ALICE), DAI, true));
        System::assert_last_event(RuntimeEvent::Bridge(Event::CurrencyPaused {
            id: DAI
        }));
        assert_eq!(CurrencyMap::<MockRuntime>::get(DAI).unwrap().1, true);
        assert_ok!(Bridge::set_currency_pause(RuntimeOrigin::signed(ALICE), DAI, false));
        System::assert_last_event(RuntimeEvent::Bridge(Event::CurrencyUnpaused {
            id: DAI
        }));
        assert_eq!(CurrencyMap::<MockRuntime>::get(DAI).unwrap().1, false);
    });
}

#[test]
fn set_currency_pause_fails_if_sender_is_not_a_manager() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_currency_pause(RuntimeOrigin::signed(BOB), DAI, true),
            Error::<MockRuntime>::SenderNotBridgeManager
        );
    });
}

#[test]
fn set_currency_pause_fails_if_already_paused() {
    ExtBuilder::new()
    .currencies_pause(vec![DAI])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_currency_pause(RuntimeOrigin::signed(ALICE), DAI, true),
            Error::<MockRuntime>::CurrencyAlreadyPaused
        );
    });
}

#[test]
fn set_currency_pause_fails_if_already_not_paused() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_currency_pause(RuntimeOrigin::signed(ALICE), DAI, false),
            Error::<MockRuntime>::CurrencyAlreadyNotPaused
        );
    });
}

#[test]
fn set_eth_blacklist_works() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_eq!(EthBlacklisted::<MockRuntime>::get(&ETH_ADDRESS_1()), false);
        assert_ok!(Bridge::set_eth_blacklist(RuntimeOrigin::signed(ALICE), ETH_ADDRESS_1(), true));
        System::assert_last_event(RuntimeEvent::Bridge(Event::BlacklistedEth {
            account: ETH_ADDRESS_1()
        }));
        assert_eq!(EthBlacklisted::<MockRuntime>::get(&ETH_ADDRESS_1()), true);
        assert_ok!(Bridge::set_eth_blacklist(RuntimeOrigin::signed(ALICE), ETH_ADDRESS_1(), false));
        System::assert_last_event(RuntimeEvent::Bridge(Event::RemovedFromBlacklistEth {
            account: ETH_ADDRESS_1()
        }));
        assert_eq!(EthBlacklisted::<MockRuntime>::get(&ETH_ADDRESS_1()), false);
    });
}

#[test]
fn set_eth_blacklist_fails_if_sender_is_not_a_manager() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_eth_blacklist(RuntimeOrigin::signed(BOB), ETH_ADDRESS_1(), true),
            Error::<MockRuntime>::SenderNotBridgeManager
        );
    });
}

#[test]
fn set_eth_blacklist_fails_if_already_blacklisted() {
    ExtBuilder::new()
    .blacklist_eth(vec![ETH_ADDRESS_1()])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_eth_blacklist(RuntimeOrigin::signed(ALICE), ETH_ADDRESS_1(), true),
            Error::<MockRuntime>::AlreadyBlacklistedEth
        );
    });
}

#[test]
fn set_eth_blacklist_fails_if_already_not_blacklisted() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_eth_blacklist(RuntimeOrigin::signed(ALICE), ETH_ADDRESS_1(), false),
            Error::<MockRuntime>::AlreadyNotBlacklistedEth
        );
    });
}

#[test]
fn set_sub_blacklist_works() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_eq!(SubBlacklisted::<MockRuntime>::get(&BOB), false);
        assert_ok!(Bridge::set_sub_blacklist(RuntimeOrigin::signed(ALICE), BOB, true));
        System::assert_last_event(RuntimeEvent::Bridge(Event::BlacklistedSub {
            account: BOB
        }));
        assert_eq!(SubBlacklisted::<MockRuntime>::get(&BOB), true);
        assert_ok!(Bridge::set_sub_blacklist(RuntimeOrigin::signed(ALICE), BOB, false));
        System::assert_last_event(RuntimeEvent::Bridge(Event::RemovedFromBlacklistSub {
            account: BOB
        }));
        assert_eq!(SubBlacklisted::<MockRuntime>::get(&BOB), false);
    });
}

#[test]
fn set_sub_blacklist_fails_if_sender_is_not_a_manager() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_sub_blacklist(RuntimeOrigin::signed(BOB), BOB, true),
            Error::<MockRuntime>::SenderNotBridgeManager
        );
    });
}

#[test]
fn set_sub_blacklist_fails_if_already_blacklisted() {
    ExtBuilder::new()
    .blacklist_sub(vec![BOB])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_sub_blacklist(RuntimeOrigin::signed(ALICE), BOB, true),
            Error::<MockRuntime>::AlreadyBlacklistedSub
        );
    });
}

#[test]
fn set_sub_blacklist_fails_if_already_not_blacklisted() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_sub_blacklist(RuntimeOrigin::signed(ALICE), BOB, false),
            Error::<MockRuntime>::AlreadyNotBlacklistedSub
        );
    });
}

#[test]
fn set_manager_works() {
    ExtBuilder::new()
    .managers(vec![])
    .build()
    .execute_with(|| {
        assert_eq!(Managers::<MockRuntime>::get(&ALICE), false);
        assert_ok!(Bridge::set_manager(RuntimeOrigin::root(), ALICE, true));
        System::assert_last_event(RuntimeEvent::Bridge(Event::ManagerAdded {
            manager: ALICE
        }));
        assert_eq!(Managers::<MockRuntime>::get(&ALICE), true);
        assert_ok!(Bridge::set_manager(RuntimeOrigin::root(), ALICE, false));
        System::assert_last_event(RuntimeEvent::Bridge(Event::ManagerRemoved {
            manager: ALICE
        }));
        assert_eq!(Managers::<MockRuntime>::get(&ALICE), false);
    });
}

#[test]
fn set_manager_fails_if_sender_is_not_a_bridge_root() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_manager(RuntimeOrigin::signed(ALICE), BOB, true),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn set_manager_fails_if_already_manager() {
    ExtBuilder::new()
    .managers(vec![ALICE])
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_manager(RuntimeOrigin::root(), ALICE, true),
            Error::<MockRuntime>::AlreadyManager
        );
    });
}

#[test]
fn set_manager_fails_if_already_not_manager() {
    ExtBuilder::new()
    .build()
    .execute_with(|| {
        assert_noop!(
            Bridge::set_manager(RuntimeOrigin::root(), BOB, false),
            Error::<MockRuntime>::AlreadyNotManager
        );
    });
}