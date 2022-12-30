use sp_std::vec::Vec;

use primitives::{CurrencyId, Balance, TokenInfo};

pub fn is_vec_unique<T>(vec: &Vec<T>) -> bool 
    where T: Eq
{
    vec.iter().enumerate().all(|(i, item)| !vec[i+1..].contains(item))
}

pub fn token_unit(currency_id: CurrencyId) -> Balance {
    if let Some(decimals) = currency_id.decimals() {
        (10 as Balance).saturating_pow(decimals.into())
    } else {
        panic!("{:?} not a token", currency_id);
    }
}