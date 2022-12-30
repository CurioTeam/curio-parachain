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

// Source https://github.com/KILTprotocol/kilt-node
// Subject to the GPL-3.0 license.

use crate::{
	types::BalanceOf, BlocksAuthored, BlocksRewarded, CandidatePool, Config, DelegatorState, InflationConfig, Pallet,
	Rewards, TotalCollatorStake,
};
use frame_support::traits::Currency;
use sp_runtime::{
	traits::{Saturating, Zero},
	Perquintill,
};

impl<T: Config> Pallet<T> {
	/// Calculates the staking rewards for a given account address.
	///
	/// Subtracts the number of rewarded blocks from the number of authored
	/// blocks by the collator and multiplies that with the current stake
	/// as well as reward rate.
	///
	/// At least used in Runtime API.
	pub fn get_unclaimed_staking_rewards(acc: &T::AccountId) -> BalanceOf<T> {
		let count_rewarded = BlocksRewarded::<T>::get(acc);
		let rewards = Rewards::<T>::get(acc);

		// delegators and collators need to be handled differently
		if let Some(delegator_state) = DelegatorState::<T>::get(acc) {
			// #blocks for unclaimed staking rewards equals
			// #blocks_authored_by_collator - #blocks_claimed_by_delegator
			let count_unclaimed = BlocksAuthored::<T>::get(&delegator_state.owner).saturating_sub(count_rewarded);
			let stake = delegator_state.amount;
			// rewards += stake * reward_count * delegator_reward_rate
			rewards.saturating_add(Self::calc_block_rewards_delegator(stake, count_unclaimed.into()))
		} else if Self::is_active_candidate(acc).is_some() {
			// #blocks for unclaimed staking rewards equals
			// #blocks_authored_by_collator - #blocks_claimed_by_collator
			let count_unclaimed = BlocksAuthored::<T>::get(acc).saturating_sub(count_rewarded);
			let stake = CandidatePool::<T>::get(acc)
				.map(|state| state.stake)
				.unwrap_or_else(BalanceOf::<T>::zero);
			// rewards += stake * self_count * collator_reward_rate
			rewards.saturating_add(Self::calc_block_rewards_collator(stake, count_unclaimed.into()))
		} else {
			rewards
		}
	}

	/// Calculates the current staking and reward rates for collators and
	/// delegators.
	///
	/// At least used in Runtime API.
	pub fn get_staking_rates() -> runtime_api_staking::StakingRates {
		let total_issuance = T::Currency::total_issuance();
		let total_stake = TotalCollatorStake::<T>::get();
		let inflation_config = InflationConfig::<T>::get();
		let collator_staking_rate = Perquintill::from_rational(total_stake.collators, total_issuance);
		let delegator_staking_rate = Perquintill::from_rational(total_stake.delegators, total_issuance);
		let collator_reward_rate = Perquintill::from_rational(
			inflation_config.collator.max_rate.deconstruct(),
			collator_staking_rate.deconstruct(),
		) * inflation_config.collator.reward_rate.annual;
		let delegator_reward_rate = Perquintill::from_rational(
			inflation_config.delegator.max_rate.deconstruct(),
			delegator_staking_rate.deconstruct(),
		) * inflation_config.delegator.reward_rate.annual;

		runtime_api_staking::StakingRates {
			collator_staking_rate,
			collator_reward_rate,
			delegator_staking_rate,
			delegator_reward_rate,
		}
	}
}