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

use frame_support::traits::{Currency, Get};
use sp_runtime::{
	traits::{Saturating, Zero},
	Perquintill,
};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

use crate::{
	types::{BalanceOf, Stake},
	BlocksAuthored, BlocksRewarded, CandidatePool, Config, DelegatorState, InflationConfig, Pallet,
	Rewards, TotalCollatorStake,
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
	pub fn get_staking_rates() -> parachain_staking_runtime_api::StakingRates {
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

		parachain_staking_runtime_api::StakingRates {
			collator_staking_rate,
			collator_reward_rate,
			delegator_staking_rate,
			delegator_reward_rate,
		}
	}

		/// Provides a sorted list of collators most suited for given
	/// delegator's stake amount determined with some heuristic algorithm.
	///
	/// The algorithm takes into account the following factors:
	/// - Total collator stake: the bigger, the more probably that this collator will
	/// continue to produce blocks.
	/// - Number of blocks authored by collator: the more, the more reliable collator is
	/// - If it is possible to become a delegator of some.
	/// collator with the given stake amount: if there are free slots or possibility to
	/// overstake some other delegator.
	///
	/// List sorted from most valued collator to the least.
	pub fn get_sorted_proposed_candidates(balance: T::Balance) -> Vec<T::AccountId> {
		let top_candidates = Self::selected_candidates();
		let mut suitable_top_candidates = Vec::new();
		let mut collators_stake_coefficients = BTreeMap::new();
		let mut blocks_authored = Vec::new();
		let mut blocks_authored_coefficients = BTreeMap::new();
		let mut final_coefficients = Vec::new();

		let stake_coefficent: u32 = 7;
		let blocks_coefficient: u32 = 3;

		// Got suitable top candidates, which fits for new delegator
		for i in 0..top_candidates.len() {
			let candidate;
			if let Some(account) = Self::candidate_pool(&top_candidates[i]) {
				candidate = account;
			} else {
				continue;
			}
			if T::MaxDelegatorsPerCollator::get() as usize > candidate.delegators.len() {
				suitable_top_candidates.push(Stake {
					owner: candidate.id,
					amount: candidate.total,
				});
			} else {
				if candidate.delegators[candidate.delegators.len() - 1].amount < balance.into() {
					suitable_top_candidates.push(Stake {
						owner: candidate.id,
						amount: candidate.total,
					});
				}
			}
		}
		// Sorted coefficients of stake
		let mut current_stake_coeff: u32 = 1;
		let mut current_stake_coeff_amount = suitable_top_candidates[0].amount;
		for i in 0..suitable_top_candidates.len() {
			if suitable_top_candidates[i].amount < current_stake_coeff_amount {
				current_stake_coeff += 1;
				current_stake_coeff_amount = suitable_top_candidates[i].amount;
			}

			collators_stake_coefficients.insert(
				suitable_top_candidates[i].owner.clone(),
				current_stake_coeff,
			);
		}
		// Got authored blocks with their accounts into vector
		for i in 0..suitable_top_candidates.len() {
			let collator = suitable_top_candidates[i].owner.clone();
			blocks_authored.push((collator.clone(), Self::blocks_authored(collator)))
		}

		// Sort vector from bigger to smaller
		blocks_authored.sort_by(|a, b| b.1.cmp(&a.1));
		// Convert authored blocks to coefficient

		let mut current_blocks_coeff: u32 = 1;
		let mut current_blocks_coeff_amount = blocks_authored[0].1;

		for i in 0..blocks_authored.len() {
			if blocks_authored[i].1 < current_blocks_coeff_amount {
				current_blocks_coeff += 1;
				current_blocks_coeff_amount = blocks_authored[i].1;
			}

			blocks_authored_coefficients.insert(blocks_authored[i].0.clone(), current_blocks_coeff);
		}
		// Fill vector with final coefficients
		for i in suitable_top_candidates {
			let collator_stake_coefficient;
			let blocks_authored_coefficient;

			if let Some(stk_coeff) = collators_stake_coefficients.get(&i.owner) {
				collator_stake_coefficient = stk_coeff;
			} else {
				continue;
			}

			if let Some(blck_coeff) = blocks_authored_coefficients.get(&i.owner) {
				blocks_authored_coefficient = blck_coeff;
			} else {
				continue;
			}

			let final_stake_coefficient =
				stake_coefficent.saturating_mul(collator_stake_coefficient.clone());
			let final_blocks_coefficient =
				blocks_coefficient.saturating_mul(blocks_authored_coefficient.clone());
			final_coefficients.push((
				i.owner.clone(),
				final_stake_coefficient + final_blocks_coefficient,
			));
		}
		final_coefficients.sort_by(|a, b| a.1.cmp(&b.1));
		final_coefficients.into_iter().map(|item| item.0).collect()
	}
}