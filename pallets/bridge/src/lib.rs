#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::*,
    Blake2_128Concat,
};

use frame_system::{
    pallet_prelude::*,
};

use codec::MaxEncodedLen;
use scale_info::TypeInfo;

use orml_traits::MultiCurrency;

use sp_core::H160 as EvmAddress;
use sp_std::prelude::*;
use sp_runtime::{
    traits::{
        CheckedAdd, CheckedSub
    },
    ArithmeticError
};
pub use crate::weights::WeightInfo;

mod components;

mod mock;
mod mock_shared;
mod tests;
mod benchmarking;
mod weights;

pub use pallet::*;

type BalanceOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;
type CurrencyIdOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::CurrencyId;
type RequestId = u128;

pub const MAX_BATCH_SIZE: usize = 100;

#[derive(Clone, Encode, Decode, Debug, TypeInfo, MaxEncodedLen, PartialEq)]
pub struct MintData<AccountId, Balance> {
    request_id: RequestId,
    to: AccountId,
    token: EvmAddress,
    amount: Balance
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
	pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type BridgeRoot: EnsureOrigin<Self::RuntimeOrigin>;

        #[pallet::constant]
		type GetNativeCurrencyId: Get<CurrencyIdOf<Self>>;

        type MultiCurrency: MultiCurrency<Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Paused,
        Unpaused,

        Mint {
            request_id: RequestId,
            to: T::AccountId,
            token: EvmAddress,
            currency_id: CurrencyIdOf<T>,
            amount: BalanceOf<T>
        },
        Burn {
            request_id: RequestId,
            from: T::AccountId,
            to: EvmAddress,
            token: EvmAddress,
            currency_id: CurrencyIdOf<T>,
            amount: BalanceOf<T>
        },
        
        ManagerAdded {
            manager: T::AccountId
        },
        ManagerRemoved {
            manager: T::AccountId
        },

        CurrencyPaused {
            id: CurrencyIdOf<T>
        },
        CurrencyUnpaused {
            id: CurrencyIdOf<T>
        },

        BlacklistedSub {
            account: T::AccountId
        },
        RemovedFromBlacklistSub {
            account: T::AccountId
        },
        BlacklistedEth {
            account: EvmAddress
        },
        RemovedFromBlacklistEth {
            account: EvmAddress
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        BridgePaused,

        SenderNotBridgeManager,
        
        AlreadyPaused,
        AlreadyNotPaused,

        AlreadyManager,
        AlreadyNotManager,

        CurrencyAlreadyPaused,
        CurrencyAlreadyNotPaused,
        CurrencyNotSupported,
        CurrencyNotActive,
        CurrencyNotPaused,

        SubAccountBlacklisted,
        EthAccountBlacklisted,
        AlreadyBlacklistedSub,
        AlreadyNotBlacklistedSub,
        AlreadyBlacklistedEth,
        AlreadyNotBlacklistedEth,

        RequestAlreadyProcessed,

        InsufficientNativeBridged,

        MaxBatchSizeExceeded
    }

    #[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub supported_currencies: Vec<(CurrencyIdOf<T>, EvmAddress, bool)>,
        pub initial_managers: Vec<T::AccountId>,
        pub blacklisted_sub: Vec<T::AccountId>,
        pub blacklisted_eth: Vec<EvmAddress>,
        pub full_pause: bool
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { 
				supported_currencies: vec![],
                initial_managers: vec![],
                blacklisted_sub: vec![],
                blacklisted_eth: vec![],
                full_pause: false,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for (currency_id, token, paused) in self.supported_currencies.iter() {
                TokenMap::<T>::insert(token.clone(), currency_id);
                CurrencyMap::<T>::insert(currency_id, (token.clone(), paused));
            }

            for account in self.blacklisted_eth.iter() {
                EthBlacklisted::<T>::insert(account, true);
            }

            for account in self.blacklisted_sub.iter() {
                SubBlacklisted::<T>::insert(account, true);
            }

            for manager in self.initial_managers.iter() {
                Managers::<T>::insert(manager, true);
            }

            Paused::<T>::set(self.full_pause);
		}
	}

    #[pallet::storage]
    pub type NativeBridgedSupply<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;
    
    #[pallet::storage]
    pub type TokenMap<T: Config> = StorageMap<_, Blake2_128Concat, EvmAddress, CurrencyIdOf<T>, OptionQuery>;

    #[pallet::storage]
    pub type CurrencyMap<T: Config> = StorageMap<_, Blake2_128Concat, CurrencyIdOf<T>, (EvmAddress, bool), OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn is_paused)]
    pub type Paused<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    pub type OutRequestId<T: Config> = StorageValue<_, RequestId, ValueQuery>;

    #[pallet::storage]
    pub type InRequestStatus<T: Config> = StorageMap<_, Blake2_128Concat, RequestId, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn is_eth_blacklisted)]
    pub type EthBlacklisted<T: Config> = StorageMap<_, Blake2_128Concat, EvmAddress, bool, ValueQuery>;
    
    #[pallet::storage]
    #[pallet::getter(fn is_sub_blacklisted)]
    pub type SubBlacklisted<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;
    
    #[pallet::storage]
    #[pallet::getter(fn is_manager)]
    pub type Managers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::mint())]
        pub fn mint(origin: OriginFor<T>, request_id: RequestId, to: T::AccountId, token: EvmAddress, amount: BalanceOf<T>) -> DispatchResult {
            Self::ensure_manager_origin(origin)?;
            Self::ensure_not_paused()?;
            
            Self::do_mint(request_id, &to, token, amount)
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::batch_mint(data.len() as u32))]
        pub fn batch_mint(origin: OriginFor<T>, data: Vec<MintData<T::AccountId, BalanceOf<T>>>) -> DispatchResult {
            Self::ensure_manager_origin(origin)?;
            Self::ensure_not_paused()?;
            
            ensure!(
                data.len() <= MAX_BATCH_SIZE,
                Error::<T>::MaxBatchSizeExceeded
            );
        
            for mint in data.iter() {
                Self::do_mint(mint.request_id, &mint.to, mint.token, mint.amount)?;
            }

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(<T as Config>::WeightInfo::burn())]
        pub fn burn(origin: OriginFor<T>, currency_id: CurrencyIdOf<T>, to: EvmAddress, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::ensure_not_paused()?;
            Self::ensure_currency_active(&currency_id)?;
            Self::ensure_not_blacklisted_sub(&who)?;
            Self::ensure_not_blacklisted_eth(&to)?;
            let token = Self::try_get_token_by_currency(&currency_id)?;

            Self::handle_native_burn_case(currency_id, amount)?;
            T::MultiCurrency::withdraw(currency_id, &who, amount)?;
            let current_request_id = OutRequestId::<T>::get();
            OutRequestId::<T>::set(current_request_id + 1);

            Self::deposit_event(Event::Burn {
                request_id: current_request_id,
                from: who,
                to,
                token,
                currency_id,
                amount
            });
        
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(<T as Config>::WeightInfo::set_full_pause())]
        pub fn set_full_pause(origin: OriginFor<T>, pause: bool) -> DispatchResult {
            Self::ensure_manager_origin(origin)?;

            if pause {
                Self::pause()
            } else {
                Self::unpause()
            }
        }

        #[pallet::call_index(4)]
        #[pallet::weight(<T as Config>::WeightInfo::set_currency_pause())]
        pub fn set_currency_pause(origin: OriginFor<T>, currency_id: CurrencyIdOf<T>, pause: bool) -> DispatchResult {
            Self::ensure_manager_origin(origin)?;

            if pause {
                Self::pause_currency(&currency_id)
            } else {
                Self::unpause_currency(&currency_id)
            }
        }

        #[pallet::call_index(5)]
        #[pallet::weight(<T as Config>::WeightInfo::set_eth_blacklist())]
        pub fn set_eth_blacklist(origin: OriginFor<T>, account: EvmAddress, blacklisted: bool) -> DispatchResult {
            Self::ensure_manager_origin(origin)?;

            if blacklisted {
                Self::blacklist_eth(&account)
            } else {
                Self::unblacklist_eth(&account)
            }
        }

        #[pallet::call_index(6)]
        #[pallet::weight(<T as Config>::WeightInfo::set_sub_blacklist())]
        pub fn set_sub_blacklist(origin: OriginFor<T>, account: T::AccountId, blacklisted: bool) -> DispatchResult {
            Self::ensure_manager_origin(origin)?;

            if blacklisted {
                Self::blacklist_sub(&account)
            } else {
                Self::unblacklist_sub(&account)
            }
        }

        #[pallet::call_index(7)]
        #[pallet::weight(<T as Config>::WeightInfo::set_manager())]
        pub fn set_manager(origin: OriginFor<T>, manager: T::AccountId, is_manager: bool) -> DispatchResult {
            T::BridgeRoot::ensure_origin(origin)?;

            if is_manager {
                Self::add_manager(&manager)
            } else {
                Self::remove_manager(&manager)
            }
        }
    }
}

impl<T: Config> Pallet<T> {
    fn ensure_request_not_processed(id: RequestId) -> DispatchResult {
        ensure!(
            !InRequestStatus::<T>::get(&id),
            Error::<T>::RequestAlreadyProcessed
        );

        Ok(())
    }

    fn do_mint(request_id: RequestId, to: &T::AccountId, token: EvmAddress, amount: BalanceOf<T>) -> DispatchResult {
        Self::ensure_request_not_processed(request_id)?;
        Self::ensure_not_blacklisted_sub(to)?;
        
        let currency_id = Self::try_get_currency_by_token(&token)?;
        Self::ensure_currency_active(&currency_id)?;

        Self::handle_native_mint_case(currency_id, amount)?;
        T::MultiCurrency::deposit(currency_id, to, amount)?;
        InRequestStatus::<T>::insert(&request_id, true);

        Self::deposit_event(Event::Mint {
            request_id, to: to.clone(), token, currency_id, amount
        });

        Ok(())
    }

    fn handle_native_mint_case(currency_id: CurrencyIdOf<T>, amount: BalanceOf<T>) -> DispatchResult {
        if currency_id == T::GetNativeCurrencyId::get() {
            NativeBridgedSupply::<T>::try_mutate(|supply| {
                if let Some(sum) = supply.checked_add(&amount) {
                    *supply = sum;
                    Ok(())
                } else {
                    Err(ArithmeticError::Overflow.into())
                }
            })
        } else {
            Ok(())
        }
    }

    fn handle_native_burn_case(currency_id: CurrencyIdOf<T>, amount: BalanceOf<T>) -> DispatchResult {
        if currency_id == T::GetNativeCurrencyId::get() {
            NativeBridgedSupply::<T>::try_mutate(|supply| {
                if let Some(sub) = supply.checked_sub(&amount) {
                    *supply = sub;
                    Ok(())
                } else {
                    Err(Error::<T>::InsufficientNativeBridged.into())
                }
            })
        } else {
            Ok(())
        }
    }
}
