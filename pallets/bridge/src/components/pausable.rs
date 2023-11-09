use super::*;

impl<T: Config> Pallet<T> {
    pub(crate) fn pause() -> DispatchResult {
        Paused::<T>::try_mutate(|paused| {
            if *paused {
                Err(Error::<T>::AlreadyPaused.into())
            } else {
                *paused = true;
                Self::deposit_event(Event::Paused);
                Ok(())
            }
        })
    }
    
    pub(crate) fn unpause() -> DispatchResult {
        Paused::<T>::try_mutate(|paused| {
            if !*paused {
                Err(Error::<T>::AlreadyNotPaused.into())
            } else {
                *paused = false; 
                Self::deposit_event(Event::Unpaused);
                Ok(())
            }
        })
    }

    pub(crate) fn ensure_not_paused() -> DispatchResult {
        ensure!(
            !Self::is_paused(),
            Error::<T>::BridgePaused
        );

        Ok(())
    }
}