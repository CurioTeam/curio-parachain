use frame_support::{
    dispatch::{
        DispatchError,
        DispatchResult
    }
};

use sp_std::vec::Vec;

use crate::{
    types::{
        SwapLimit
    }
};

pub trait DEXManager<AccountId, Balance, CurrencyId> {
	fn get_liquidity_pool(currency_id_a: CurrencyId, currency_id_b: CurrencyId) -> (Balance, Balance);

	fn get_swap_amount(path: &[CurrencyId], limit: SwapLimit<Balance>) -> Option<(Balance, Balance)>;

	fn get_best_price_swap_path(
		supply_currency_id: CurrencyId,
		target_currency_id: CurrencyId,
		limit: SwapLimit<Balance>,
		alternative_path_joint_list: Vec<Vec<CurrencyId>>,
	) -> Option<(Vec<CurrencyId>, Balance, Balance)>;

	fn swap_with_specific_path(
		who: &AccountId,
		path: &[CurrencyId],
		limit: SwapLimit<Balance>,
	) -> Result<(Balance, Balance), DispatchError>;

	fn add_liquidity(
		who: &AccountId,
		currency_id_a: CurrencyId,
		currency_id_b: CurrencyId,
		max_amount_a: Balance,
		max_amount_b: Balance,
		min_share_increment: Balance,
		stake_increment_share: bool,
	) -> Result<(Balance, Balance, Balance), DispatchError>;

	fn remove_liquidity(
		who: &AccountId,
		currency_id_a: CurrencyId,
		currency_id_b: CurrencyId,
		remove_share: Balance,
		min_withdrawn_a: Balance,
		min_withdrawn_b: Balance,
		by_unstake: bool,
	) -> Result<(Balance, Balance), DispatchError>;
}

pub trait DEXIncentives<AccountId, CurrencyId, Balance> {
	fn do_deposit_dex_share(who: &AccountId, lp_currency_id: CurrencyId, amount: Balance) -> DispatchResult;
	fn do_withdraw_dex_share(who: &AccountId, lp_currency_id: CurrencyId, amount: Balance) -> DispatchResult;
}

impl<AccountId, CurrencyId, Balance> DEXIncentives<AccountId, CurrencyId, Balance> for () {
	fn do_deposit_dex_share(_: &AccountId, _: CurrencyId, _: Balance) -> DispatchResult {
		Ok(())
	}

	fn do_withdraw_dex_share(_: &AccountId, _: CurrencyId, _: Balance) -> DispatchResult {
		Ok(())
	}
}