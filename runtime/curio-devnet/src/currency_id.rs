use codec::{
	Encode, 
	Decode, 
	MaxEncodedLen,
};
use scale_info::TypeInfo;
use sp_std::vec;
use sp_runtime::RuntimeDebug;
use pallet_dex::types::DexCurrencyId;
use module_currency_id::create_currency_id;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

create_currency_id! {
	#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[repr(u8)]
	pub enum TokenSymbol {
		CGT("Curio Governance Token", 18) = 0,

		// Polkadot
		DOT("Polkadot", 10) = 1,
		KSM("Kusama", 12) = 2,

		// Bridged
		ETH("Ethereum", 18) = 30,
		DAI("Dai Stablecoin", 18) = 31,
		USDT("Tether USD", 6) = 32,
		USDC("USD Coin", 6) = 33,
		BTC("Bitcoin", 18) = 34,
		AVAX("Avalanche", 9) = 35,
		SOL("Solana", 9) = 36,
		AURORA("Aurora", 18) = 37,
		NEAR("NEAR Protocol", 18) = 38,
		WCT1("Wrapped Car Token 1", 2) = 39,
		TON("TON coin", 9) = 40,
	}
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum DexShare {
	Token(TokenSymbol),
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum CurrencyId {
	Token(TokenSymbol),
	DexShare(DexShare, DexShare),
}

impl DexCurrencyId for CurrencyId {
	fn is_token_currency_id(&self) -> bool {
		matches!(self, CurrencyId::Token(_))
	}

	fn is_dex_share_currency_id(&self) -> bool {
		matches!(self, CurrencyId::DexShare(_, _))
	}

	fn is_trading_pair_currency_id(&self) -> bool {
		matches!(
			self,
			CurrencyId::Token(_)
		)
	}

	fn split_dex_share_currency_id(&self) -> Option<(Self, Self)> {
		match self {
			CurrencyId::DexShare(dex_share_0, dex_share_1) => {
				let currency_id_0: CurrencyId = (*dex_share_0).into();
				let currency_id_1: CurrencyId = (*dex_share_1).into();
				Some((currency_id_0, currency_id_1))
			}
			_ => None,
		}
	}

	fn join_dex_share_currency_id(currency_id_0: Self, currency_id_1: Self) -> Option<Self> {
		let dex_share_0 = match currency_id_0 {
			CurrencyId::Token(symbol) => DexShare::Token(symbol),
			// Unsupported
			CurrencyId::DexShare(..) => return None,
		};
		let dex_share_1 = match currency_id_1 {
			CurrencyId::Token(symbol) => DexShare::Token(symbol),
			// Unsupported
			CurrencyId::DexShare(..) => return None,
		};
		Some(CurrencyId::DexShare(dex_share_0, dex_share_1))
	}
}

impl From<DexShare> for u32 {
	fn from(val: DexShare) -> u32 {
		let mut bytes = [0u8; 4];
		match val {
			DexShare::Token(token) => {
				bytes[3] = token.into();
			}
		}
		u32::from_be_bytes(bytes)
	}
}

impl Into<CurrencyId> for DexShare {
	fn into(self) -> CurrencyId {
		match self {
			DexShare::Token(token) => CurrencyId::Token(token),
		}
	}
}