use super::*;

impl<T: Config> Pallet<T> {
    pub(crate) fn pause_currency(id: &CurrencyIdOf<T>) -> DispatchResult {
        CurrencyMap::<T>::try_mutate(id, |status| {
            if let Some((_, paused)) = status.as_mut() {
                if *paused {
                    Err(Error::<T>::CurrencyAlreadyPaused.into())
                } else {
                    *paused = true;
                    Self::deposit_event(Event::CurrencyPaused{ id: id.clone()});
                    Ok(())
                }
            } else {
                Err(Error::<T>::CurrencyNotSupported.into())
            }
        })
    }

    pub(crate) fn unpause_currency(id: &CurrencyIdOf<T>) -> DispatchResult {
        CurrencyMap::<T>::try_mutate(id, |status| {
            if let Some((_, paused)) = status.as_mut() {
                if !*paused {
                    Err(Error::<T>::CurrencyAlreadyNotPaused.into())
                } else {
                    *paused = false;
                    Self::deposit_event(Event::CurrencyUnpaused{ id: id.clone()});
                    Ok(())
                }
            } else {
                Err(Error::<T>::CurrencyNotSupported.into())
            }
        })
    }

    pub(crate) fn is_currency_supported(id: &CurrencyIdOf<T>) -> bool {
        CurrencyMap::<T>::get(id).is_some()
    }

    pub(crate) fn is_currency_active(id: &CurrencyIdOf<T>) -> bool {
        !CurrencyMap::<T>::get(id).map_or(true, |c| c.1)
    }

    pub(crate) fn ensure_currency_active(id: &CurrencyIdOf<T>) -> DispatchResult {
        ensure!(
            Self::is_currency_supported(id),
            Error::<T>::CurrencyNotSupported
        );
        
        ensure!(
            Self::is_currency_active(id),
            Error::<T>::CurrencyNotActive
        );

        Ok(())
    }

    pub(crate) fn try_get_currency_by_token(token: &EvmAddress) -> Result<CurrencyIdOf<T>, DispatchError> {
        TokenMap::<T>::get(token).ok_or(Error::<T>::CurrencyNotSupported.into())
    }

    pub(crate) fn try_get_token_by_currency(currency_id: &CurrencyIdOf<T>) -> Result<EvmAddress, DispatchError> {
        let (token, _) =  CurrencyMap::<T>::get(currency_id).ok_or(Error::<T>::CurrencyNotSupported)?;
        Ok(token)
    }
}