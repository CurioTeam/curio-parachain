use frame_system::pallet_prelude::*;
use frame_support::pallet_prelude::*;
use crate::{Error};

/// Interface for using the pallet in security assets
pub trait WhitelistInterface<T: frame_system::Config + crate::Config> {
    fn ensure_origin_is_admin(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError>;
    fn ensure_origin_is_manager(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError>;
    fn ensure_origin_is_admin_or_manager(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError> {
        if let Ok(account_id) = Self::ensure_origin_is_admin(origin.clone()) {
			return Ok(account_id);
		}

		Self::ensure_origin_is_manager(origin)
    }

    fn ensure_admin(who: &T::AccountId) -> DispatchResult {
        ensure!(
			Self::is_admin(who),
			Error::<T>::NotWhitelistAdmin
		);

		Ok(())
    }
    fn ensure_manager(who: &T::AccountId) -> DispatchResult {
        ensure!(
			Self::is_manager(who),
			Error::<T>::NotWhitelistManager
		);

		Ok(())
    }


    fn is_admin(who: &T::AccountId) -> bool;
    fn is_manager(who: &T::AccountId) -> bool;
    fn is_admin_or_manager(who: &T::AccountId) -> bool {
        Self::is_admin(who) || Self::is_manager(who)
    }
    
    fn is_active_investor(who: &T::AccountId) -> bool;
    fn is_active_investors(first: &T::AccountId, second: &T::AccountId) -> bool {
        Self::is_active_investor(first) && Self::is_active_investor(second)
    }
}