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

// Source https://github.com/UniqueNetwork/unique-chain
// Subject to the GPL-3.0 license.

// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Integration between bondrewd and parity scale codec
//! Maybe we can move it to scale-codec itself in future?

#[macro_export]
macro_rules! bondrewd_codec {
	($T:ty) => {
		impl Encode for $T {
			fn encode_to<O: codec::Output + ?Sized>(&self, dest: &mut O) {
				dest.write(&self.into_bytes())
			}
		}
		impl codec::Decode for $T {
			fn decode<I: codec::Input + ?Sized>(from: &mut I) -> Result<Self, codec::Error> {
				let mut bytes = [0; Self::BYTE_SIZE];
				from.read(&mut bytes)?;
				Ok(Self::from_bytes(bytes))
			}
		}
		impl MaxEncodedLen for $T {
			fn max_encoded_len() -> usize {
				Self::BYTE_SIZE
			}
		}
		impl TypeInfo for $T {
			type Identity = [u8; Self::BYTE_SIZE];
			fn type_info() -> scale_info::Type {
				<[u8; Self::BYTE_SIZE] as TypeInfo>::type_info()
			}
		}
	};
}
