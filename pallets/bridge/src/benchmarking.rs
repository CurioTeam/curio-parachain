#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Bridge;
use frame_benchmarking::v1::{account, benchmarks};
use frame_system::RawOrigin;

use crate::mock_shared::{
    CGT_ERC20,
    ETH_ADDRESS_1,
};

fn existential_deposit<T: Config>() -> BalanceOf<T> {
    <T::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::minimum_balance(T::GetNativeCurrencyId::get())
}

fn add_supported_token<T: Config>(currency_id: CurrencyIdOf<T>, eth_address: EvmAddress) -> EvmAddress {
    if !Bridge::<T>::is_currency_supported(&currency_id) {
        TokenMap::<T>::insert(eth_address.clone(), currency_id);
        CurrencyMap::<T>::insert(currency_id, (eth_address.clone(), false));
        eth_address
    } else {
        Bridge::<T>::try_get_token_by_currency(&currency_id).unwrap()
    }
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn assert_has_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

benchmarks! {
	mint {
        let token = add_supported_token::<T>(T::GetNativeCurrencyId::get(), CGT_ERC20());
		let bridge_root = T::BridgeRoot::try_successful_origin().unwrap();
        let manager: T::AccountId = account("Manager", 0, 0);
        let bob: T::AccountId = account("Bob", 0, 0);
        Bridge::<T>::set_manager(bridge_root, manager.clone(), true).unwrap();
        
	}: _(RawOrigin::Signed(manager), 0, bob.clone(), token.clone(), existential_deposit::<T>())
	verify {
		assert_last_event::<T>(Event::Mint{
            request_id: 0,
            to: bob,
            token: token.clone(),
            currency_id: T::GetNativeCurrencyId::get(),
            amount: existential_deposit::<T>()
        }.into());
	}

    batch_mint {
        let token = add_supported_token::<T>(T::GetNativeCurrencyId::get(), CGT_ERC20());
        let l in 1 .. MAX_BATCH_SIZE.try_into().unwrap();
		let bridge_root = T::BridgeRoot::try_successful_origin().unwrap();
        let manager: T::AccountId = account("Manager", 0, 0);
        let bob: T::AccountId = account("Bob", 0, 0);
        Bridge::<T>::set_manager(bridge_root, manager.clone(), true).unwrap();
        
        let data: Vec<MintData<T::AccountId, BalanceOf<T>>> = (0..l).map(|i| {
            MintData {
                request_id: i as RequestId,
                to: bob.clone(),
                token: token.clone(),
                amount: existential_deposit::<T>()
            }
        }).collect();
	}: _(RawOrigin::Signed(manager), data.clone())
	verify {
        for i in 0..l {
            assert_has_event::<T>(Event::Mint {
                request_id: i as RequestId,
                to: bob.clone(),
                token: token.clone(),
                currency_id: T::GetNativeCurrencyId::get(),
                amount: existential_deposit::<T>()
            }.into());
        }
	}

    burn {
        let token = add_supported_token::<T>(T::GetNativeCurrencyId::get(), CGT_ERC20());
		let bridge_root = T::BridgeRoot::try_successful_origin().unwrap();
        let manager: T::AccountId = account("Manager", 0, 0);
        let bob: T::AccountId = account("Bob", 0, 0);
        Bridge::<T>::set_manager(bridge_root, manager.clone(), true).unwrap();
        Bridge::<T>::mint(RawOrigin::Signed(manager.clone()).into(), 0, bob.clone(), token.clone(), existential_deposit::<T>()).unwrap();
        
	}: _(RawOrigin::Signed(bob.clone()), T::GetNativeCurrencyId::get(), ETH_ADDRESS_1(), existential_deposit::<T>())
	verify {
		assert_last_event::<T>(Event::Burn{
            request_id: 0,
            from: bob,
            to: ETH_ADDRESS_1(),
            token: token.clone(),
            currency_id: T::GetNativeCurrencyId::get(),
            amount: existential_deposit::<T>()
        }.into());
	}

    set_full_pause {
        let token = add_supported_token::<T>(T::GetNativeCurrencyId::get(), CGT_ERC20());
		let bridge_root = T::BridgeRoot::try_successful_origin().unwrap();
        let manager: T::AccountId = account("Manager", 0, 0);
        Bridge::<T>::set_manager(bridge_root, manager.clone(), true).unwrap();
        
	}: _(RawOrigin::Signed(manager), true)
	verify {
		assert_last_event::<T>(Event::Paused.into());
	}

    set_currency_pause {
        let token = add_supported_token::<T>(T::GetNativeCurrencyId::get(), CGT_ERC20());
		let bridge_root = T::BridgeRoot::try_successful_origin().unwrap();
        let manager: T::AccountId = account("Manager", 0, 0);
        Bridge::<T>::set_manager(bridge_root, manager.clone(), true).unwrap();
        
	}: _(RawOrigin::Signed(manager), T::GetNativeCurrencyId::get(), true)
	verify {
		assert_last_event::<T>(Event::CurrencyPaused {
            id: T::GetNativeCurrencyId::get()
        }.into());
	}

    set_eth_blacklist {
        let token = add_supported_token::<T>(T::GetNativeCurrencyId::get(), CGT_ERC20());
		let bridge_root = T::BridgeRoot::try_successful_origin().unwrap();
        let manager: T::AccountId = account("Manager", 0, 0);
        Bridge::<T>::set_manager(bridge_root, manager.clone(), true).unwrap();
        
	}: _(RawOrigin::Signed(manager), ETH_ADDRESS_1(), true)
	verify {
		assert_last_event::<T>(Event::BlacklistedEth {
            account: ETH_ADDRESS_1()
        }.into());
	}

    set_sub_blacklist {
        let token = add_supported_token::<T>(T::GetNativeCurrencyId::get(), CGT_ERC20());
		let bridge_root = T::BridgeRoot::try_successful_origin().unwrap();
        let manager: T::AccountId = account("Manager", 0, 0);
        let bob: T::AccountId = account("Bob", 0, 0);
        Bridge::<T>::set_manager(bridge_root, manager.clone(), true).unwrap();
        
	}: _(RawOrigin::Signed(manager), bob.clone(), true)
	verify {
		assert_last_event::<T>(Event::BlacklistedSub {
            account: bob
        }.into());
	}

    set_manager {
        let token = add_supported_token::<T>(T::GetNativeCurrencyId::get(), CGT_ERC20());
		let bridge_root = T::BridgeRoot::try_successful_origin().unwrap();
        let manager: T::AccountId = account("Manager", 0, 0);
        
	}: _<T::RuntimeOrigin>(bridge_root, manager.clone(), true)
	verify {
		assert_last_event::<T>(Event::ManagerAdded {
            manager: manager
        }.into());
	}

	impl_benchmark_test_suite!(Bridge, crate::mock::ExtBuilder::default().build(), crate::mock::MockRuntime);
}