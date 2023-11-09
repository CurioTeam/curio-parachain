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

// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use core::marker::PhantomData;
use up_sponsorship::SponsorshipHandler;
use frame_support::{
	traits::{IsSubType},
};
use collection_primitives::{
	CollectionId, REFUNGIBLE_SPONSOR_TRANSFER_TIMEOUT, TokenId,
};
use sp_runtime::traits::Saturating;
use pallet_common::{
    CollectionHandle, CreateItemBasket, TokenPropertyBasket,
};
use pallet_refungible::{
    Call as CurioCall, Config as CurioConfig, RefungibleTransferBasket, RefungibleApproveBasket
};

pub trait Config: CurioConfig {}
impl<T> Config for T where T: CurioConfig {}

// TODO: permission check?
pub fn withdraw_set_token_property<T: Config>(
	collection: &CollectionHandle<T>,
	who: &T::AccountId,
	item_id: &TokenId,
	data_size: usize,
) -> Option<()> {
	// preliminary sponsoring correctness check
	if !<pallet_refungible::Owned<T>>::get((collection.id, who, item_id)) {
		return None;
	}

	if data_size > collection.limits.sponsored_data_size() as usize {
		return None;
	}

	let block_number = <frame_system::Pallet<T>>::block_number() as T::BlockNumber;
	let limit = collection.limits.sponsored_data_rate_limit()?;

	if let Some(last_tx_block) = TokenPropertyBasket::<T>::get(collection.id, item_id) {
		let timeout = last_tx_block + limit.into();
		if block_number < timeout {
			return None;
		}
	}

	<TokenPropertyBasket<T>>::insert(collection.id, item_id, block_number);

	Some(())
}

pub fn withdraw_transfer<T: Config>(
	collection: &CollectionHandle<T>,
	who: &T::AccountId,
	item_id: &TokenId,
) -> Option<()> {
	// preliminary sponsoring correctness check
	if !<pallet_refungible::Owned<T>>::get((collection.id, who, item_id)) {
		return None;
	}

	// sponsor timeout
	let block_number = <frame_system::Pallet<T>>::block_number() as T::BlockNumber;
	let limit = collection
		.limits
		.sponsor_transfer_timeout(REFUNGIBLE_SPONSOR_TRANSFER_TIMEOUT);

	let last_tx_block = <RefungibleTransferBasket<T>>::get((collection.id, item_id, who));

	if let Some(last_tx_block) = last_tx_block {
		let timeout = last_tx_block + limit.into();
		if block_number < timeout {
			return None;
		}
	}

	<RefungibleTransferBasket<T>>::insert(
		(collection.id, item_id, who),
		block_number,
	);

	Some(())
}

pub fn withdraw_create_item<T: Config>(
	collection: &CollectionHandle<T>,
	who: &T::AccountId,
) -> Option<()> {
	// sponsor timeout
	let block_number = <frame_system::Pallet<T>>::block_number() as T::BlockNumber;
	let limit = collection
		.limits
		.sponsor_transfer_timeout(REFUNGIBLE_SPONSOR_TRANSFER_TIMEOUT);

	if let Some(last_tx_block) = <CreateItemBasket<T>>::get((collection.id, who)) {
		let timeout = last_tx_block + limit.into();
		if block_number < timeout {
			return None;
		}
	}

	CreateItemBasket::<T>::insert((collection.id, who), block_number);

	Some(())
}

pub fn withdraw_approve<T: Config>(
	collection: &CollectionHandle<T>,
	who: &T::AccountId,
	item_id: &TokenId,
) -> Option<()> {
	// sponsor timeout
	let block_number = <frame_system::Pallet<T>>::block_number() as T::BlockNumber;
	let limit = collection.limits.sponsor_approve_timeout();

	let last_tx_block = <RefungibleApproveBasket<T>>::get((collection.id, item_id, who));

	if let Some(last_tx_block) = last_tx_block {
		let timeout = last_tx_block + limit.into();
		if block_number < timeout {
			return None;
		}
	}

	<RefungibleApproveBasket<T>>::insert((collection.id, item_id, who), block_number);

	Some(())
}

fn load<T: CurioConfig>(id: CollectionId) -> Option<(T::AccountId, CollectionHandle<T>)> {
	let collection = CollectionHandle::new(id)?;
	let sponsor = collection.sponsorship.sponsor().cloned()?;
	Some((sponsor, collection))
}

pub struct CurioSponsorshipHandler<T>(PhantomData<T>);
impl<T, C> SponsorshipHandler<T::AccountId, C> for CurioSponsorshipHandler<T>
where
	T: Config,
	C: IsSubType<CurioCall<T>>,
{
	fn get_sponsor(who: &T::AccountId, call: &C) -> Option<T::AccountId> {
		match IsSubType::<CurioCall<T>>::is_sub_type(call)? {
			CurioCall::set_token_properties {
				collection_id,
				token_id,
				properties,
				..
			} => {
				let (sponsor, collection) = load::<T>(*collection_id)?;
				withdraw_set_token_property(
					&collection,
					who,
					&token_id,
					// No overflow may happen, as data larger than usize can't reach here
					properties.iter().map(|p| p.key.len() + p.value.len()).sum(),
				)
				.map(|()| sponsor)
			}
			CurioCall::create_item {
				collection_id,
				..
			} => {
				let (sponsor, collection) = load(*collection_id)?;
				withdraw_create_item::<T>(
					&collection,
					who,
				)
				.map(|()| sponsor)
			}
			CurioCall::transfer {
				collection_id,
				token_id,
				..
			} => {
				let (sponsor, collection) = load(*collection_id)?;
				withdraw_transfer::<T>(
					&collection,
					who,
					token_id,
				)
				.map(|()| sponsor)
			}
			CurioCall::transfer_from {
				collection_id,
				token_id,
				from,
				..
			} => {
				let (sponsor, collection) = load(*collection_id)?;
				withdraw_transfer::<T>(&collection, from, token_id).map(|()| sponsor)
			}
			CurioCall::set_allowance {
				collection_id,
				token_id,
				..
			} => {
				let (sponsor, collection) = load(*collection_id)?;
				withdraw_approve::<T>(&collection, who, token_id).map(|()| sponsor)
			}
			_ => None,
		}
	}
}

pub trait SponsorshipPredict<T: Config> {
	fn predict(collection: CollectionId, account: T::AccountId, token: TokenId) -> Option<u64>
	where
		u64: From<<T as frame_system::Config>::BlockNumber>;
}

pub struct CurioSponsorshipPredict<T>(PhantomData<T>);

impl<T: Config> SponsorshipPredict<T> for CurioSponsorshipPredict<T> {
	fn predict(collection_id: CollectionId, who: T::AccountId, token: TokenId) -> Option<u64>
	where
		u64: From<<T as frame_system::Config>::BlockNumber>,
	{
		let collection = <CollectionHandle<T>>::try_get(collection_id).ok()?;
		let _ = collection.sponsorship.sponsor()?;

		// sponsor timeout
		let block_number = <frame_system::Pallet<T>>::block_number() as T::BlockNumber;
		let limit = collection
			.limits
			.sponsor_transfer_timeout(REFUNGIBLE_SPONSOR_TRANSFER_TIMEOUT);

		let last_tx_block = <RefungibleTransferBasket<T>>::get((collection.id, token, who));

		if let Some(last_tx_block) = last_tx_block {
			return Some(
				last_tx_block
					.saturating_add(limit.into())
					.saturating_sub(block_number)
					.into(),
			);
		}

		let token_exists = <pallet_refungible::TotalSupply<T>>::contains_key((collection.id, token));

		if token_exists {
			Some(0)
		} else {
			None
		}
	}
}