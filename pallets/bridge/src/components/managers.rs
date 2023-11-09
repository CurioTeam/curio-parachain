use super::*;

impl<T: Config> Pallet<T> {
    pub(crate) fn add_manager(manager: &T::AccountId) -> DispatchResult {
        Managers::<T>::try_mutate(manager, |is_manager| {
            if *is_manager {
                Err(Error::<T>::AlreadyManager.into())
            } else {
                *is_manager = true;
                Self::deposit_event(Event::ManagerAdded{ manager: manager.clone() });
                Ok(())
            }
        })
    }
    
    pub(crate) fn remove_manager(manager: &T::AccountId) -> DispatchResult {
        Managers::<T>::try_mutate(manager, move |is_manager| {
            if !*is_manager {
                Err(Error::<T>::AlreadyNotManager.into())
            } else {
                *is_manager = false;
                Self::deposit_event(Event::ManagerRemoved{ manager: manager.clone() });
                Ok(())
            }
        })
    }

    pub(crate) fn ensure_manager(manager: &T::AccountId) -> DispatchResult {
        ensure!(
            Self::is_manager(manager),
            Error::<T>::SenderNotBridgeManager
        );

        Ok(())
    }

    pub(crate) fn ensure_manager_origin(origin: T::RuntimeOrigin) -> Result<T::AccountId, DispatchError> {
        let who = ensure_signed(origin)?;
        Self::ensure_manager(&who)?;
        Ok(who)
    }
}