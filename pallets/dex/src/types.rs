use crate::*;
use codec::{Encode, Decode, MaxEncodedLen};
use sp_runtime::{RuntimeDebug, FixedU128};
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub type ExchangeRate = FixedU128;
pub type Ratio = FixedU128;

/// Parameters of TradingPair in Provisioning status
#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct ProvisioningParameters<Balance, BlockNumber> {
	/// limit contribution per time.
	pub min_contribution: (Balance, Balance),
	/// target provision that trading pair could to be Enabled.
	pub target_provision: (Balance, Balance),
	/// accumulated provision amount for this Provisioning trading pair.
	pub accumulated_provision: (Balance, Balance),
	/// The number of block that status can be converted to Enabled.
	pub not_before: BlockNumber,
}

/// Status for TradingPair
#[derive(Clone, Copy, Encode, Decode, RuntimeDebug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum TradingPairStatus<Balance, BlockNumber> {
	/// Default status,
	/// can withdraw liquidity, re-enable and list this trading pair.
	Disabled,
	/// TradingPair is Provisioning,
	/// can add provision and disable this trading pair.
	Provisioning(ProvisioningParameters<Balance, BlockNumber>),
	/// TradingPair is Enabled,
	/// can add/remove liquidity, trading and disable this trading pair.
	Enabled,
}

impl<Balance, BlockNumber> Default for TradingPairStatus<Balance, BlockNumber> {
	fn default() -> Self {
		Self::Disabled
	}
}

pub trait DexCurrencyId: Sized {
	fn is_token_currency_id(&self) -> bool;
	fn is_dex_share_currency_id(&self) -> bool;
	fn is_trading_pair_currency_id(&self) -> bool;
	fn split_dex_share_currency_id(&self) -> Option<(Self, Self)>;
	fn join_dex_share_currency_id(currency_id_0: Self, currency_id_1: Self) -> Option<Self>;
}

#[derive(Encode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TradingPair<CurrencyId: DexCurrencyId>(CurrencyId, CurrencyId);

impl<CurrencyId: DexCurrencyId + PartialEq + PartialOrd + Copy> TradingPair<CurrencyId> {
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

impl<CurrencyId: DexCurrencyId + PartialEq + PartialOrd + Copy + Decode> Decode for TradingPair<CurrencyId> {
	fn decode<I: codec::Input>(input: &mut I) -> sp_std::result::Result<Self, codec::Error> {
		let (first, second): (CurrencyId, CurrencyId) = Decode::decode(input)?;
		Self::from_currency_ids(first, second).ok_or_else(|| codec::Error::from("invalid currency id"))
	}
}

#[derive(RuntimeDebug, Encode, Decode, Clone, Copy, PartialEq, Eq, TypeInfo)]
pub enum SwapLimit<Balance> {
	/// use exact amount supply amount to swap. (exact_supply_amount, minimum_target_amount)
	ExactSupply(Balance, Balance),
	/// swap to get exact amount target. (maximum_supply_amount, exact_target_amount)
	ExactTarget(Balance, Balance),
}

#[derive(Eq, PartialEq, RuntimeDebug)]
pub enum SwapError {
	CannotSwap,
}

impl Into<DispatchError> for SwapError {
	fn into(self) -> DispatchError {
		DispatchError::Other("Cannot swap")
	}
}