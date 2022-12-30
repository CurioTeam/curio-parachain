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

#![allow(clippy::from_over_into)]

use bstringify::bstringify;
use codec::{Decode, Encode, MaxEncodedLen};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{
	prelude::*,
	vec
};

use crate::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

macro_rules! create_currency_id {
    ($(#[$meta:meta])*
	$vis:vis enum TokenSymbol {
        $($(#[$vmeta:meta])* $symbol:ident($name:expr, $deci:literal) = $val:literal,)*
    }) => {
		$(#[$meta])*
		$vis enum TokenSymbol {
			$($(#[$vmeta])* $symbol = $val,)*
		}

		impl TryFrom<u8> for TokenSymbol {
			type Error = ();

			fn try_from(v: u8) -> Result<Self, Self::Error> {
				match v {
					$($val => Ok(TokenSymbol::$symbol),)*
					_ => Err(()),
				}
			}
		}

		impl Into<u8> for TokenSymbol {
			fn into(self) -> u8 {
				match self {
					$(TokenSymbol::$symbol => ($val),)*
				}
			}
		}

		impl TryFrom<Vec<u8>> for CurrencyId {
			type Error = ();
			fn try_from(v: Vec<u8>) -> Result<CurrencyId, ()> {
				match v.as_slice() {
					$(bstringify!($symbol) => Ok(CurrencyId::Token(TokenSymbol::$symbol)),)*
					_ => Err(()),
				}
			}
		}

		impl TokenInfo for CurrencyId {
			fn currency_id(&self) -> Option<u8> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($val),)*
					_ => None,
				}
			}
			fn name(&self) -> Option<&str> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($name),)*
					_ => None,
				}
			}
			fn symbol(&self) -> Option<&str> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some(stringify!($symbol)),)*
					_ => None,
				}
			}
			fn decimals(&self) -> Option<u8> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($deci),)*
					_ => None,
				}
			}
		}

		$(pub const $symbol: CurrencyId = CurrencyId::Token(TokenSymbol::$symbol);)*

		impl TokenSymbol {
			pub fn get_info() -> Vec<(&'static str, u32)> {
				vec![
					$((stringify!($symbol), $deci),)*
				]
			}
		}
	}
}

create_currency_id! {
	// Represent a Token symbol with 8 bit
	//
	// 0 - 127: Polkadot Ecosystem tokens
	// 0 - 19: Acala & Polkadot native tokens
	// 20 - 39: External tokens (e.g. bridged)
	// 40 - 127: Polkadot parachain tokens
	//
	// 128 - 255: Kusama Ecosystem tokens
	// 128 - 147: Karura & Kusama native tokens
	// 148 - 167: External tokens (e.g. bridged)
	// 168 - 255: Kusama parachain tokens
	#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[repr(u8)]
	pub enum TokenSymbol {
		// Native token
		CGT("Curio Governance Token", 18) = 0,

		// Polkadot ecosystem
		DOT("Polkadot", 10) = 2,
		QTZ("Quartz", 18) = 3,

		// Bridged
		ETH("Ethereum", 18) = 20,
	}
}

pub trait TokenInfo {
	fn currency_id(&self) -> Option<u8>;
	fn name(&self) -> Option<&str>;
	fn symbol(&self) -> Option<&str>;
	fn decimals(&self) -> Option<u8>;
}

pub type ForeignAssetId = u16;
pub type Lease = BlockNumber;

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

impl CurrencyId {
	pub fn is_token_currency_id(&self) -> bool {
		matches!(self, CurrencyId::Token(_))
	}

	pub fn is_dex_share_currency_id(&self) -> bool {
		matches!(self, CurrencyId::DexShare(_, _))
	}

	pub fn is_trading_pair_currency_id(&self) -> bool {
		matches!(
			self,
			CurrencyId::Token(_)
		)
	}

	pub fn split_dex_share_currency_id(&self) -> Option<(Self, Self)> {
		match self {
			CurrencyId::DexShare(dex_share_0, dex_share_1) => {
				let currency_id_0: CurrencyId = (*dex_share_0).into();
				let currency_id_1: CurrencyId = (*dex_share_1).into();
				Some((currency_id_0, currency_id_1))
			}
			_ => None,
		}
	}

	pub fn join_dex_share_currency_id(currency_id_0: Self, currency_id_1: Self) -> Option<Self> {
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

/// H160 CurrencyId Type enum
#[derive(
	Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive, TypeInfo,
)]
#[repr(u8)]
pub enum CurrencyIdType {
	Token = 1, // 0 is prefix of precompile and predeploy
	DexShare,
	StableAsset,
}

#[derive(
	Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive, TypeInfo,
)]
#[repr(u8)]
pub enum DexShareType {
	Token,
}

impl Into<DexShareType> for DexShare {
	fn into(self) -> DexShareType {
		match self {
			DexShare::Token(_) => DexShareType::Token,
		}
	}
}
