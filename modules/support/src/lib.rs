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

use sp_runtime::FixedU128;

pub mod dex;
pub use dex::*;

pub mod incentives;
pub use incentives::*;

pub type ExchangeRate = FixedU128;
use primitives::{CurrencyId, Balance, TokenInfo};

pub type Ratio = FixedU128;

pub fn token_unit(currency_id: CurrencyId) -> Balance {
    if let Some(decimals) = currency_id.decimals() {
        (10 as Balance).saturating_pow(decimals.into())
    } else {
        panic!("{:?} not a token", currency_id);
    }
}