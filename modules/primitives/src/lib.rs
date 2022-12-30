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

// Source https://github.com/AcalaNetwork/Acala
// Subject to the GPL-3.0 license.

// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	weights::{
		constants::WEIGHT_PER_SECOND, Weight
	}
};
use sp_runtime::{
    traits::{Verify, IdentifyAccount, BlakeTwo256},
    generic, MultiSignature, MultiAddress, Perbill
};
use sp_core::{Encode, Decode, RuntimeDebug, MaxEncodedLen};
use scale_info::{TypeInfo};
pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub mod currency;
pub mod time;
pub use currency::*;

pub use polkadot_runtime_common::{BlockHashCount};

/// Opaque block header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Opaque block type.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// Opaque block identifier type.
pub type BlockId = generic::BlockId<Block>;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u64;

pub type Amount = i128;

// MILLICENTS = the base number of indivisible units for balances
pub const MILLICENTS: Balance = 10_000_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
}

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = CENTS;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 0.5 of a second of compute with a 12 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND
	.saturating_div(2)
	.set_proof_size(cumulus_primitives_core::relay_chain::v2::MAX_POV_SIZE as u64);


#[derive(Encode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TradingPair(CurrencyId, CurrencyId);

impl TradingPair {
	pub fn from_currency_ids(currency_id_a: CurrencyId, currency_id_b: CurrencyId) -> Option<Self> {
		if currency_id_a.is_trading_pair_currency_id()
			&& currency_id_b.is_trading_pair_currency_id()
			&& currency_id_a != currency_id_b
		{
			if currency_id_a > currency_id_b {
				Some(TradingPair(currency_id_b, currency_id_a))
			} else {
				Some(TradingPair(currency_id_a, currency_id_b))
			}
		} else {
			None
		}
	}

	pub fn first(&self) -> CurrencyId {
		self.0
	}

	pub fn second(&self) -> CurrencyId {
		self.1
	}

	pub fn dex_share_currency_id(&self) -> CurrencyId {
		CurrencyId::join_dex_share_currency_id(self.first(), self.second())
			.expect("shouldn't be invalid! guaranteed by construction")
	}
}

impl Decode for TradingPair {
	fn decode<I: codec::Input>(input: &mut I) -> sp_std::result::Result<Self, codec::Error> {
		let (first, second): (CurrencyId, CurrencyId) = Decode::decode(input)?;
		TradingPair::from_currency_ids(first, second).ok_or_else(|| codec::Error::from("invalid currency id"))
	}
}