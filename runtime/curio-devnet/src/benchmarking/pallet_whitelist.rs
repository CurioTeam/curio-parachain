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

use sp_std::prelude::*;

use orml_benchmarking::{runtime_benchmarks, whitelisted_caller};

use crate::{Runtime, System, RolesRoot, RuntimeOrigin, AccountId, Whitelist};

use pallet_whitelist::Event as PalletEvent;
use pallet_whitelist::{MAX_NEW_INVESTORS, Investor, InvestorKey};

use super::utils::assert_last_event;

fn investors_from_size(size: usize) -> Vec<(InvestorKey, Investor<AccountId>)> {
    let mut investors = Vec::<(InvestorKey, Investor<AccountId>)>::with_capacity(size.into());

	for i in 0..size {
		investors.push(
			(
				[i as u8; 32],
				Investor {
					account: AccountId::new([i as u8; 32]),
					is_active: false
				}
			)
		)
	}

	investors
}

runtime_benchmarks! {
    {Runtime, pallet_whitelist}
    
    add_admin {
        let admin: AccountId = whitelisted_caller();
    }: _(RuntimeOrigin::signed(RolesRoot::get()), admin.clone())
    verify {
        assert_last_event(PalletEvent::AddAdmin { 
            new_admin: admin
        }.into());
    }

    remove_admin {
        let admin: AccountId = whitelisted_caller();
        Whitelist::add_admin(RuntimeOrigin::signed(RolesRoot::get()), admin.clone()).unwrap();
    }: _(RuntimeOrigin::signed(RolesRoot::get()), admin.clone())
    verify {
        assert_last_event(PalletEvent::RemoveAdmin { 
            admin: admin
        }.into());
    }

    add_manager {
        let admin: AccountId = whitelisted_caller();
        Whitelist::add_admin(RuntimeOrigin::signed(RolesRoot::get()), admin.clone()).unwrap();

        let manager: AccountId = whitelisted_caller();
    }: _(RuntimeOrigin::signed(admin.clone()), manager.clone())
    verify {
        assert_last_event(PalletEvent::AddManager { 
            who: admin,
            new_manager: manager
        }.into());
    }

    remove_manager {
        let admin: AccountId = whitelisted_caller();
        Whitelist::add_admin(RuntimeOrigin::signed(RolesRoot::get()), admin.clone()).unwrap();

        let manager: AccountId = whitelisted_caller();
        Whitelist::add_manager(RuntimeOrigin::signed(admin.clone()), manager.clone()).unwrap();
    }: _(RuntimeOrigin::signed(admin.clone()), manager.clone())
    verify {
        assert_last_event(PalletEvent::RemoveManager { 
            who: admin,
            manager: manager
        }.into());
    }

    add_investors {
        let i in 1..MAX_NEW_INVESTORS.into();

        let admin: AccountId = whitelisted_caller();
        Whitelist::add_admin(RuntimeOrigin::signed(RolesRoot::get()), admin.clone()).unwrap();

        let investors = investors_from_size(i as usize);
    }: _(RuntimeOrigin::signed(admin.clone()), investors.clone())
    verify {
        for (investor_key, investor) in investors {
            System::assert_has_event(PalletEvent::AddInvestor {
                who: admin.clone(),
                investor_key: investor_key,
                investor: investor
            }.into());
        }
    }

    set_investor_status {
        let admin: AccountId = whitelisted_caller();
        Whitelist::add_admin(RuntimeOrigin::signed(RolesRoot::get()), admin.clone()).unwrap();

        let investors = investors_from_size(1 as usize);
        let (_, investor) = investors[0].clone();
        Whitelist::add_investors(RuntimeOrigin::signed(admin.clone()), investors).unwrap();
    }: _(RuntimeOrigin::signed(admin.clone()), investor.account.clone(), !investor.is_active)
    verify {
        assert_last_event(PalletEvent::InvestorStatusSet {
			who: admin, 
			investor: investor.account,
			is_active: !investor.is_active
		}.into());
    }

    change_investor_address {
        let admin: AccountId = whitelisted_caller();
        Whitelist::add_admin(RuntimeOrigin::signed(RolesRoot::get()), admin.clone()).unwrap();

        let investor = Investor {
            account: AccountId::new([0u8; 32]),
            is_active: false
        };

        let investors = vec![
            (
                [0u8; 32],
                investor.clone()
            )
        ];
        Whitelist::add_investors(RuntimeOrigin::signed(admin.clone()), investors).unwrap();

        let new_account = AccountId::new([1u8; 32]); 
    }: _(RuntimeOrigin::signed(admin.clone()), investor.account.clone(), new_account.clone())
    verify {
        assert_last_event(PalletEvent::InvestorAccountChanged {
			who: admin, 
			old_account: investor.account,
			new_account: new_account
		}.into());
    }

    change_my_address {
        let admin: AccountId = whitelisted_caller();
        Whitelist::add_admin(RuntimeOrigin::signed(RolesRoot::get()), admin.clone()).unwrap();

        let investor_account = AccountId::new([0u8; 32]);

        let investors = vec![
            (
                [0u8; 32],
                Investor {
                    account: investor_account.clone(),
                    is_active: false
                }
            )
        ];
        Whitelist::add_investors(RuntimeOrigin::signed(admin.clone()), investors).unwrap();

        let new_account = AccountId::new([1u8; 32]); 
    }: _(RuntimeOrigin::signed(investor_account.clone()), new_account.clone())
    verify {
        assert_last_event(PalletEvent::InvestorAccountChanged {
			who: investor_account.clone(), 
			old_account: investor_account,
			new_account: new_account
		}.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orml_benchmarking::impl_benchmark_test_suite;
    use crate::benchmarking::utils::tests::new_test_ext;

    impl_benchmark_test_suite!(new_test_ext(),);
}