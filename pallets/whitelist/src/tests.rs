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

use crate::{mock::*, Error, Investor, traits::WhitelistInterface};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};

#[test]
fn ensure_roles_root_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::ensure_origin_is_roles_root(RuntimeOrigin::signed(ROLES_ROOT)));
	});
}

#[test]
fn ensure_roles_root_fails_for_others() {
	new_test_ext().execute_with(|| {
		assert_noop!(
            Whitelist::ensure_origin_is_roles_root(RuntimeOrigin::signed(ALICE)),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Whitelist::ensure_origin_is_roles_root(RuntimeOrigin::none()),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Whitelist::ensure_origin_is_roles_root(RuntimeOrigin::root()),
            DispatchError::BadOrigin
        );
	});
}

#[test]
fn ensure_admin_works() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
		assert_ok!(Whitelist::ensure_origin_is_admin(RuntimeOrigin::signed(ALICE)));
	});
}

#[test]
fn ensure_admin_fails_for_others() {
	new_test_ext().execute_with(|| {
        assert_noop!(
            Whitelist::ensure_origin_is_admin(RuntimeOrigin::signed(ROLES_ROOT)),
            Error::<Test>::PermissionDenied
        );
		assert_noop!(
            Whitelist::ensure_origin_is_admin(RuntimeOrigin::signed(ALICE)),
            Error::<Test>::PermissionDenied
        );
        assert_noop!(
            Whitelist::ensure_origin_is_admin(RuntimeOrigin::none()),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Whitelist::ensure_origin_is_admin(RuntimeOrigin::root()),
            DispatchError::BadOrigin
        );
	});
}

#[test]
fn ensure_manager_works() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB));
		assert_ok!(Whitelist::ensure_origin_is_manager(RuntimeOrigin::signed(BOB)));
	});
}

#[test]
fn ensure_manager_fails_for_others() {
	new_test_ext().execute_with(|| {
        assert_noop!(
            Whitelist::ensure_origin_is_manager(RuntimeOrigin::signed(ROLES_ROOT)),
            Error::<Test>::PermissionDenied
        );
		assert_noop!(
            Whitelist::ensure_origin_is_manager(RuntimeOrigin::signed(ALICE)),
            Error::<Test>::PermissionDenied
        );
        assert_noop!(
            Whitelist::ensure_origin_is_manager(RuntimeOrigin::none()),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Whitelist::ensure_origin_is_manager(RuntimeOrigin::root()),
            DispatchError::BadOrigin
        );
	});
}

#[test]
fn ensure_admin_or_manager_works_for_admin() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
		assert_ok!(Whitelist::ensure_origin_is_admin_or_manager(RuntimeOrigin::signed(ALICE)));
	});
}

#[test]
fn ensure_admin_or_manager_works_for_manager() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB));
		assert_ok!(Whitelist::ensure_origin_is_admin_or_manager(RuntimeOrigin::signed(BOB)));
	});
}

#[test]
fn ensure_admin_or_manager_fails_for_others() {
	new_test_ext().execute_with(|| {
        assert_noop!(
            Whitelist::ensure_origin_is_admin_or_manager(RuntimeOrigin::signed(ROLES_ROOT)),
            Error::<Test>::PermissionDenied
        );
		assert_noop!(
            Whitelist::ensure_origin_is_admin_or_manager(RuntimeOrigin::signed(ALICE)),
            Error::<Test>::PermissionDenied
        );
        assert_noop!(
            Whitelist::ensure_origin_is_admin_or_manager(RuntimeOrigin::none()),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Whitelist::ensure_origin_is_admin_or_manager(RuntimeOrigin::root()),
            DispatchError::BadOrigin
        );
	});
}

#[test]
fn is_active_investor_returns_true_if_active() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestors4::get();

        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors.clone()));

        assert_eq!(investors[0].1.is_active, true);
        assert_eq!(Whitelist::is_active_investor(&investors[0].1.account), true);
	});
}

#[test]
fn is_active_investor_returns_false_if_not_active() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestors4::get();

        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors.clone()));

        assert_eq!(investors[1].1.is_active, false);
        assert_eq!(Whitelist::is_active_investor(&investors[1].1.account), false);
	});
}

#[test]
fn is_active_investors_returns_true_if_both_active() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestors4::get();

        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors.clone()));

        assert_eq!(investors[0].1.is_active, true);
        assert_eq!(investors[2].1.is_active, true);
        assert_eq!(Whitelist::is_active_investors(&investors[0].1.account, &investors[2].1.account), true);
	});
}

#[test]
fn is_active_investors_returns_false_if_one_active() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestors4::get();

        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors.clone()));

        assert_eq!(investors[0].1.is_active, true);
        assert_eq!(investors[1].1.is_active, false);
        assert_eq!(Whitelist::is_active_investors(&investors[0].1.account, &investors[1].1.account), false);
        assert_eq!(Whitelist::is_active_investors(&investors[1].1.account, &investors[0].1.account), false);
	});
}

#[test]
fn is_active_investors_returns_false_if_both_not_active() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestors4::get();

        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors.clone()));

        assert_eq!(investors[1].1.is_active, false);
        assert_eq!(investors[3].1.is_active, false);
        assert_eq!(Whitelist::is_active_investors(&investors[1].1.account, &investors[3].1.account), false);
	});
}

#[test]
fn add_admin_works() {
	new_test_ext().execute_with(|| {
        System::set_block_number(1);

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_eq!(Whitelist::is_admin(ALICE), true);
        
        System::assert_last_event(RuntimeEvent::Whitelist(crate::Event::AddAdmin { 
            new_admin: ALICE
        }));
	});
}

#[test]
fn add_admin_fails_if_admin_exists() {
	new_test_ext().execute_with(|| {
        System::set_block_number(1);

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_eq!(Whitelist::is_admin(ALICE), true);
        
        System::assert_last_event(RuntimeEvent::Whitelist(crate::Event::AddAdmin {
            new_admin: ALICE
        }));

        assert_noop!(
            Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE),
            Error::<Test>::WhitelistAdminExists
        );
	});
}

#[test]
fn add_admin_works_only_for_root() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB));

        assert_noop!(
            Whitelist::add_admin(RuntimeOrigin::signed(ALICE), EVE),
            DispatchError::BadOrigin
        );
		assert_noop!(
            Whitelist::add_admin(RuntimeOrigin::signed(BOB), EVE),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Whitelist::add_admin(RuntimeOrigin::signed(CHARLIE), EVE),
            DispatchError::BadOrigin
        );
	});
}

#[test]
fn remove_admin_works() {
	new_test_ext().execute_with(|| {
        System::set_block_number(1);

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_eq!(Whitelist::is_admin(ALICE), true);
        
        System::assert_last_event(RuntimeEvent::Whitelist(crate::Event::AddAdmin { 
            new_admin: ALICE
        }));

        assert_ok!(Whitelist::remove_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_eq!(Whitelist::is_admin(ALICE), false);
        
        System::assert_last_event(RuntimeEvent::Whitelist(crate::Event::RemoveAdmin { 
            admin: ALICE
        }));
	});
}

#[test]
fn remove_admin_fails_if_admin_not_exists() {
	new_test_ext().execute_with(|| {
        assert_eq!(Whitelist::is_admin(ALICE), false);
        
        assert_noop!(
            Whitelist::remove_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE),
            Error::<Test>::NotWhitelistAdmin
        );
	});
}

#[test]
fn remove_admin_works_only_for_root() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), EVE));

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB));

        assert_noop!(
            Whitelist::remove_admin(RuntimeOrigin::signed(ALICE), EVE),
            DispatchError::BadOrigin
        );
		assert_noop!(
            Whitelist::remove_admin(RuntimeOrigin::signed(BOB), EVE),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Whitelist::remove_admin(RuntimeOrigin::signed(CHARLIE), EVE),
            DispatchError::BadOrigin
        );
	});
}

#[test]
fn add_manager_works() {
	new_test_ext().execute_with(|| {
        System::set_block_number(1);

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
		assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB));
        assert_eq!(Whitelist::is_manager(BOB), true);
        
        System::assert_last_event(RuntimeEvent::Whitelist(crate::Event::AddManager { 
            who: ALICE,
            new_manager: BOB
        }));
	});
}

#[test]
fn add_manager_fails_if_manager_exists() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
		assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB));
        
        assert_noop!(
            Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB),
            Error::<Test>::WhitelistManagerExists
        );
	});
}

#[test]
fn add_manager_works_only_for_admin() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB));

        assert_noop!(
            Whitelist::add_manager(RuntimeOrigin::signed(ROLES_ROOT), EVE),
            Error::<Test>::PermissionDenied
        );
        assert_noop!(
            Whitelist::add_manager(RuntimeOrigin::signed(BOB), EVE),
            Error::<Test>::PermissionDenied
        );
        assert_noop!(
            Whitelist::add_manager(RuntimeOrigin::signed(CHARLIE), EVE),
            Error::<Test>::PermissionDenied
        );
	});
}

#[test]
fn remove_manager_works() {
	new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
		assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB));

        assert_ok!(Whitelist::remove_manager(RuntimeOrigin::signed(ALICE), BOB));
        assert_eq!(Whitelist::is_manager(BOB), false);
        
        System::assert_last_event(RuntimeEvent::Whitelist(crate::Event::RemoveManager { 
            who: ALICE,
            manager: BOB
        }));
	});
}

#[test]
fn remove_manager_fails_if_manager_not_exists() {
	new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_eq!(Whitelist::is_manager(BOB), false);
        
        assert_noop!(
            Whitelist::remove_manager(RuntimeOrigin::signed(ALICE), BOB),
            Error::<Test>::NotWhitelistManager
        );
	});
}

#[test]
fn remove_manager_works_only_for_admin() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB));
        assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), EVE));

        assert_noop!(
            Whitelist::remove_manager(RuntimeOrigin::signed(ROLES_ROOT), BOB),
            Error::<Test>::PermissionDenied
        );
        assert_noop!(
            Whitelist::remove_manager(RuntimeOrigin::signed(EVE), BOB),
            Error::<Test>::PermissionDenied
        );
        assert_noop!(
            Whitelist::remove_manager(RuntimeOrigin::signed(CHARLIE), BOB),
            Error::<Test>::PermissionDenied
        );
	});
}

#[test]
fn add_investors_works_for_admin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestors2::get();

        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors.clone()));

        for (key, investor) in investors {
            assert_eq!(
                Whitelist::investor_key(investor.account).unwrap(),
                key
            );
            assert_eq!(
                Whitelist::investor(key).unwrap(),
                Investor {
                    account: investor.account,
                    is_active: investor.is_active
                }
            );
            System::assert_has_event(RuntimeEvent::Whitelist(crate::Event::AddInvestor {
                who: ALICE,
                investor_key: key,
                investor: Investor {
                    account: investor.account,
                    is_active: investor.is_active
                }
            }));
        }
	});
}

#[test]
fn add_investors_works_for_manager() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), CHARLIE));
        
        let investors = SampleInvestors2::get();

        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(CHARLIE), investors.clone()));

        for (key, investor) in investors {
            assert_eq!(
                Whitelist::investor_key(investor.account).unwrap(),
                key
            );
            assert_eq!(
                Whitelist::investor(key).unwrap(),
                Investor {
                    account: investor.account,
                    is_active: investor.is_active
                }
            );
            System::assert_has_event(RuntimeEvent::Whitelist(crate::Event::AddInvestor {
                who: CHARLIE,
                investor_key: key,
                investor: Investor {
                    account: investor.account,
                    is_active: investor.is_active
                }
            }));
        }
	});
}

#[test]
fn add_investors_fails_if_too_many_investors() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));

        assert_noop!(
            Whitelist::add_investors(RuntimeOrigin::signed(ALICE), SampleInvestorsTooMany::get()),
            Error::<Test>::InvalidInput
        );
	});
}

#[test]
fn add_investors_fails_if_investors_empty() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = vec![];

        assert_noop!(
            Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors),
            Error::<Test>::InvalidInput
        );
	});
}

#[test]
fn add_investors_fails_if_account_already_investor() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestorsDoubleAccount::get();
        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors[..investors.len() - 1].to_vec()));

        assert_noop!(
            Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors[investors.len() - 1..].to_vec()),
            Error::<Test>::AccountAlreadyInvestor
        );
	});
}

#[test]
fn add_investors_fails_if_account_duplicates_given() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestorsDoubleAccount::get();

        assert_noop!(
            Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors),
            Error::<Test>::AccountDuplicate
        );
	});
}

#[test]
fn add_investors_fails_if_investor_key_exists() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestorsDoubleKey::get();
        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors[..investors.len() - 1].to_vec()));

        assert_noop!(
            Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors[investors.len() - 1..].to_vec()),
            Error::<Test>::InvestorKeyExists
        );
	});
}

#[test]
fn add_investors_fails_if_investor_key_duplicates_given() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestorsDoubleAccount::get();

        assert_noop!(
            Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors),
            Error::<Test>::AccountDuplicate
        );
	});
}

#[test]
fn add_investors_fails_for_root() {
	new_test_ext().execute_with(|| {
        let investors = SampleInvestors2::get();

        assert_noop!(
            Whitelist::add_investors(RuntimeOrigin::signed(ROLES_ROOT), investors),
            Error::<Test>::PermissionDenied
        );
	});
}

#[test]
fn set_investor_status_works_for_admin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestors2::get();
        
        let (target_key, Investor {account: target_account, is_active: status}) = investors[0].clone();
        let new_status = !status;

        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors));


        assert_ok!(Whitelist::set_investor_status(RuntimeOrigin::signed(ALICE), target_account, new_status));

        assert_eq!(
            Whitelist::investor(target_key).unwrap().is_active,
            new_status
        );

        System::assert_last_event(RuntimeEvent::Whitelist(crate::Event::InvestorStatusSet { 
            who: ALICE,
            investor: target_account,
            is_active: new_status
        }));
	});
}

#[test]
fn set_investor_status_works_for_manager() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), BOB));
        
        let investors = SampleInvestors2::get();
        
        let (target_key, Investor {account: target_account, is_active: status}) = investors[0].clone();
        let new_status = !status;

        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors));


        assert_ok!(Whitelist::set_investor_status(RuntimeOrigin::signed(BOB), target_account, new_status));

        assert_eq!(
            Whitelist::investor(target_key).unwrap().is_active,
            new_status
        );

        System::assert_last_event(RuntimeEvent::Whitelist(crate::Event::InvestorStatusSet { 
            who: BOB,
            investor: target_account,
            is_active: new_status
        }));
	});
}

#[test]
fn set_investor_status_fails_if_investor_not_exists() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        let investors = SampleInvestors2::get();

        assert_ok!(Whitelist::add_investors(RuntimeOrigin::signed(ALICE), investors));

        assert_noop!(
            Whitelist::set_investor_status(RuntimeOrigin::signed(ALICE), CHARLIE, true),
            Error::<Test>::NotInvestor
        );
	});
}

#[test]
fn set_investor_status_fails_if_already_active() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        assert_ok!(Whitelist::add_investors(
            RuntimeOrigin::signed(ALICE), 
            vec![([0u8; 32], Investor {account: BOB, is_active: true})]
        ));

        assert_noop!(
            Whitelist::set_investor_status(RuntimeOrigin::signed(ALICE), BOB, true),
            Error::<Test>::AlreadyActive
        );
	});
}

#[test]
fn set_investor_status_fails_if_already_not_active() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        assert_ok!(Whitelist::add_investors(
            RuntimeOrigin::signed(ALICE), 
            vec![([0u8; 32], Investor {account: BOB, is_active: false})]
        ));

        assert_noop!(
            Whitelist::set_investor_status(RuntimeOrigin::signed(ALICE), BOB, false),
            Error::<Test>::AlreadyNotActive
        );
	});
}

#[test]
fn set_investor_status_fails_for_root() {
    new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        assert_ok!(Whitelist::add_investors(
            RuntimeOrigin::signed(ALICE), 
            vec![([0u8; 32], Investor {account: BOB, is_active: false})]
        ));

        assert_noop!(
            Whitelist::set_investor_status(RuntimeOrigin::signed(ROLES_ROOT), BOB, false),
            Error::<Test>::PermissionDenied
        );
	});
}

#[test]
fn change_investor_address_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        assert_ok!(Whitelist::add_investors(
            RuntimeOrigin::signed(ALICE), 
            vec![([0u8; 32], Investor {account: BOB, is_active: false})]
        ));

        assert_ok!(Whitelist::change_investor_address(RuntimeOrigin::signed(ALICE), BOB, CHARLIE));

        assert_eq!(
            Whitelist::investor_key(CHARLIE).unwrap(),
            [0u8; 32]
        );

        assert_eq!(
            Whitelist::investor_key(BOB),
            None
        );

        assert_eq!(
            Whitelist::investor([0u8; 32]).unwrap(),
            Investor {account: CHARLIE, is_active: false}
        );

        System::assert_last_event(RuntimeEvent::Whitelist(crate::Event::InvestorAccountChanged { 
            who: ALICE,
            old_account: BOB,
            new_account: CHARLIE
        }));
	});
}

#[test]
fn change_investor_address_fails_if_same_address() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        assert_ok!(Whitelist::add_investors(
            RuntimeOrigin::signed(ALICE), 
            vec![([0u8; 32], Investor {account: BOB, is_active: false})]
        ));

        assert_noop!(
            Whitelist::change_investor_address(RuntimeOrigin::signed(ALICE), BOB, BOB),
            Error::<Test>::SameAddress
        );
	});
}

#[test]
fn change_investor_address_fails_if_not_investor() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        assert_ok!(Whitelist::add_investors(
            RuntimeOrigin::signed(ALICE), 
            vec![([0u8; 32], Investor {account: BOB, is_active: false})]
        ));

        assert_noop!(
            Whitelist::change_investor_address(RuntimeOrigin::signed(ALICE), CHARLIE, BOB),
            Error::<Test>::NotInvestor
        );
	});
}

#[test]
fn change_investor_address_fails_for_others() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        assert_ok!(Whitelist::add_manager(RuntimeOrigin::signed(ALICE), CHARLIE));
        
        assert_ok!(Whitelist::add_investors(
            RuntimeOrigin::signed(ALICE), 
            vec![([0u8; 32], Investor {account: BOB, is_active: false})]
        ));

        assert_noop!(
            Whitelist::change_investor_address(RuntimeOrigin::signed(CHARLIE), BOB, EVE),
            Error::<Test>::PermissionDenied
        );

        assert_noop!(
            Whitelist::change_investor_address(RuntimeOrigin::signed(ROLES_ROOT), BOB, EVE),
            Error::<Test>::PermissionDenied
        );

        assert_noop!(
            Whitelist::change_investor_address(RuntimeOrigin::signed(EVE), BOB, EVE),
            Error::<Test>::PermissionDenied
        );
	});
}

#[test]
fn change_my_address_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

		assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        assert_ok!(Whitelist::add_investors(
            RuntimeOrigin::signed(ALICE), 
            vec![([0u8; 32], Investor {account: BOB, is_active: false})]
        ));

        assert_ok!(Whitelist::change_my_address(RuntimeOrigin::signed(BOB), CHARLIE));

        assert_eq!(
            Whitelist::investor_key(CHARLIE).unwrap(),
            [0u8; 32]
        );

        assert_eq!(
            Whitelist::investor_key(BOB),
            None
        );

        assert_eq!(
            Whitelist::investor([0u8; 32]).unwrap(),
            Investor {account: CHARLIE, is_active: false}
        );

        System::assert_last_event(RuntimeEvent::Whitelist(crate::Event::InvestorAccountChanged { 
            who: BOB,
            old_account: BOB,
            new_account: CHARLIE
        }));
	});
}

#[test]
fn change_my_address_fails_if_same_address() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));
        
        assert_ok!(Whitelist::add_investors(
            RuntimeOrigin::signed(ALICE), 
            vec![([0u8; 32], Investor {account: BOB, is_active: false})]
        ));

        assert_noop!(
            Whitelist::change_my_address(RuntimeOrigin::signed(BOB), BOB),
            Error::<Test>::SameAddress
        );
	});
}

#[test]
fn change_my_address_fails_if_not_investor() {
    new_test_ext().execute_with(|| {
        assert_ok!(Whitelist::add_admin(RuntimeOrigin::signed(ROLES_ROOT), ALICE));

        assert_noop!(
            Whitelist::change_my_address(RuntimeOrigin::signed(ALICE), BOB),
            Error::<Test>::NotInvestor
        );
	});
}