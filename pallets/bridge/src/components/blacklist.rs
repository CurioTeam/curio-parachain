use super::*;

impl<T: Config> Pallet<T> {
    pub(crate) fn blacklist_sub(account: &T::AccountId) -> DispatchResult {
        SubBlacklisted::<T>::try_mutate(account, |blacklisted| {
            if *blacklisted {
                Err(Error::<T>::AlreadyBlacklistedSub.into())
            } else {
                *blacklisted = true;
                Self::deposit_event(Event::BlacklistedSub{ account: account.clone() });
                Ok(())
            }
        })
    }

    pub(crate) fn unblacklist_sub(account: &T::AccountId) -> DispatchResult {
        SubBlacklisted::<T>::try_mutate(account, |blacklisted| {
            if !*blacklisted {
                Err(Error::<T>::AlreadyNotBlacklistedSub.into())
            } else {
                *blacklisted = false;
                Self::deposit_event(Event::RemovedFromBlacklistSub{ account: account.clone() });
                Ok(())
            }
        })
    }

    pub(crate) fn blacklist_eth(account: &EvmAddress) -> DispatchResult {
        EthBlacklisted::<T>::try_mutate(account, |blacklisted| {
            if *blacklisted {
                Err(Error::<T>::AlreadyBlacklistedEth.into())
            } else {
                *blacklisted = true;
                Self::deposit_event(Event::BlacklistedEth{ account: account.clone() });
                Ok(())
            }
        })
    }

    pub(crate) fn unblacklist_eth(account: &EvmAddress) -> DispatchResult {
        EthBlacklisted::<T>::try_mutate(account, |blacklisted| {
            if !*blacklisted {
                Err(Error::<T>::AlreadyNotBlacklistedEth.into())
            } else {
                *blacklisted = false;
                Self::deposit_event(Event::RemovedFromBlacklistEth{ account: account.clone() });
                Ok(())
            }
        })
    }

    pub(crate) fn ensure_not_blacklisted_sub(account: &T::AccountId) -> DispatchResult {
        ensure!(
            !Self::is_sub_blacklisted(account),
            Error::<T>::SubAccountBlacklisted
        );

        Ok(())
    }
    pub(crate) fn ensure_not_blacklisted_eth(account: &EvmAddress) -> DispatchResult {
        ensure!(
            !Self::is_eth_blacklisted(account),
            Error::<T>::EthAccountBlacklisted
        );

        Ok(())
    }
}