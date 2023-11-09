
//! Autogenerated weights for `pallet_refungible`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `alex-ubuntu`, CPU: `12th Gen Intel(R) Core(TM) i7-12700K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/curio-parachain-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_refungible
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/curio-devnet/src/weights/pallet_refungible.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_refungible`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_refungible::WeightInfo for WeightInfo<T> {
	/// Storage: Whitelist Admins (r:1 w:0)
	/// Proof: Whitelist Admins (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Common CreatedCollectionCount (r:1 w:1)
	/// Proof: Common CreatedCollectionCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Common DestroyedCollectionCount (r:1 w:0)
	/// Proof: Common DestroyedCollectionCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Common CollectionProperties (r:0 w:1)
	/// Proof: Common CollectionProperties (max_values: None, max_size: Some(2113950), added: 2116425, mode: MaxEncodedLen)
	/// Storage: Common CollectionById (r:0 w:1)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Common PropertyPermissions (r:0 w:1)
	/// Proof: Common PropertyPermissions (max_values: None, max_size: Some(16598), added: 19073, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 64]`.
	/// The range of component `j` is `[1, 64]`.
	fn init_collection(i: u32, j: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1179`
		//  Estimated: `3522`
		// Minimum execution time: 60_964 nanoseconds.
		Weight::from_parts(33_109_756, 3522)
			// Standard Error: 75_771
			.saturating_add(Weight::from_parts(771_327, 0).saturating_mul(i.into()))
			// Standard Error: 75_771
			.saturating_add(Weight::from_parts(906_875, 0).saturating_mul(j.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Common CollectionById (r:1 w:1)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Refungible TotalSupply (r:1 w:0)
	/// Proof: Refungible TotalSupply (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Common DestroyedCollectionCount (r:1 w:1)
	/// Proof: Common DestroyedCollectionCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Common AdminAmount (r:0 w:1)
	/// Proof: Common AdminAmount (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Common CollectionProperties (r:0 w:1)
	/// Proof: Common CollectionProperties (max_values: None, max_size: Some(2113950), added: 2116425, mode: MaxEncodedLen)
	/// Storage: Refungible TokensMinted (r:0 w:1)
	/// Proof: Refungible TokensMinted (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Refungible TokensBurnt (r:0 w:1)
	/// Proof: Refungible TokensBurnt (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	fn destroy_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2317`
		//  Estimated: `6274`
		// Minimum execution time: 47_458 nanoseconds.
		Weight::from_parts(51_064_000, 6274)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Common PropertyPermissions (r:1 w:0)
	/// Proof: Common PropertyPermissions (max_values: None, max_size: Some(16598), added: 19073, mode: MaxEncodedLen)
	/// Storage: Common CollectionProperties (r:1 w:1)
	/// Proof: Common CollectionProperties (max_values: None, max_size: Some(2113950), added: 2116425, mode: MaxEncodedLen)
	fn set_collection_property() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `51734`
		//  Estimated: `2138758`
		// Minimum execution time: 90_834 nanoseconds.
		Weight::from_parts(93_264_000, 2138758)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Common CollectionProperties (r:1 w:1)
	/// Proof: Common CollectionProperties (max_values: None, max_size: Some(2113950), added: 2116425, mode: MaxEncodedLen)
	fn delete_collection_property() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `35111`
		//  Estimated: `2119685`
		// Minimum execution time: 65_117 nanoseconds.
		Weight::from_parts(67_096_000, 2119685)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Common PropertyPermissions (r:1 w:0)
	/// Proof: Common PropertyPermissions (max_values: None, max_size: Some(16598), added: 19073, mode: MaxEncodedLen)
	/// Storage: Common CollectionProperties (r:1 w:1)
	/// Proof: Common CollectionProperties (max_values: None, max_size: Some(2113950), added: 2116425, mode: MaxEncodedLen)
	/// The range of component `k` is `[1, 64]`.
	fn set_collection_properties(k: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `51734`
		//  Estimated: `2138758`
		// Minimum execution time: 93_176 nanoseconds.
		Weight::from_parts(62_649_476, 2138758)
			// Standard Error: 115_708
			.saturating_add(Weight::from_parts(64_222_140, 0).saturating_mul(k.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Common CollectionProperties (r:1 w:1)
	/// Proof: Common CollectionProperties (max_values: None, max_size: Some(2113950), added: 2116425, mode: MaxEncodedLen)
	/// The range of component `k` is `[1, 64]`.
	fn delete_collection_properties(k: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `35111`
		//  Estimated: `2119685`
		// Minimum execution time: 66_220 nanoseconds.
		Weight::from_parts(238_445_189, 2119685)
			// Standard Error: 218_314
			.saturating_add(Weight::from_parts(23_418_750, 0).saturating_mul(k.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Common PropertyPermissions (r:1 w:1)
	/// Proof: Common PropertyPermissions (max_values: None, max_size: Some(16598), added: 19073, mode: MaxEncodedLen)
	fn set_property_permission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `18655`
		//  Estimated: `22333`
		// Minimum execution time: 60_118 nanoseconds.
		Weight::from_parts(61_911_000, 22333)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Whitelist KeysOfInvestors (r:200 w:0)
	/// Proof: Whitelist KeysOfInvestors (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Whitelist Investors (r:200 w:0)
	/// Proof: Whitelist Investors (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Refungible TokensMinted (r:1 w:1)
	/// Proof: Refungible TokensMinted (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Refungible AccountBalance (r:200 w:200)
	/// Proof: Refungible AccountBalance (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Common PropertyPermissions (r:1 w:0)
	/// Proof: Common PropertyPermissions (max_values: None, max_size: Some(16598), added: 19073, mode: MaxEncodedLen)
	/// Storage: Refungible TokenProperties (r:1 w:1)
	/// Proof: Refungible TokenProperties (max_values: None, max_size: Some(2113954), added: 2116429, mode: MaxEncodedLen)
	/// Storage: Refungible Balance (r:0 w:200)
	/// Proof: Refungible Balance (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Refungible TotalSupply (r:0 w:1)
	/// Proof: Refungible TotalSupply (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Refungible Owned (r:0 w:200)
	/// Proof: Refungible Owned (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 200]`.
	/// The range of component `p` is `[1, 64]`.
	fn create_item(l: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `19011 + l * (308 ±0)`
		//  Estimated: `2141253 + l * (7650 ±0)`
		// Minimum execution time: 2_584_108 nanoseconds.
		Weight::from_parts(2_638_498_000, 2141253)
			// Standard Error: 153_978
			.saturating_add(Weight::from_parts(9_672_965, 0).saturating_mul(l.into()))
			// Standard Error: 481_271
			.saturating_add(Weight::from_parts(14_755_060, 7650).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(l.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(l.into())))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Whitelist KeysOfInvestors (r:200 w:0)
	/// Proof: Whitelist KeysOfInvestors (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Whitelist Investors (r:200 w:0)
	/// Proof: Whitelist Investors (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Refungible TokensMinted (r:1 w:1)
	/// Proof: Refungible TokensMinted (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Refungible AccountBalance (r:200 w:200)
	/// Proof: Refungible AccountBalance (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Common PropertyPermissions (r:1 w:0)
	/// Proof: Common PropertyPermissions (max_values: None, max_size: Some(16598), added: 19073, mode: MaxEncodedLen)
	/// Storage: Refungible TokenProperties (r:1 w:1)
	/// Proof: Refungible TokenProperties (max_values: None, max_size: Some(2113954), added: 2116429, mode: MaxEncodedLen)
	/// Storage: Refungible Balance (r:0 w:200)
	/// Proof: Refungible Balance (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Refungible TotalSupply (r:0 w:1)
	/// Proof: Refungible TotalSupply (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Refungible Owned (r:0 w:200)
	/// Proof: Refungible Owned (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn create_max_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80692`
		//  Estimated: `3671253`
		// Minimum execution time: 6_277_944 nanoseconds.
		Weight::from_parts(6_341_546_000, 3671253)
			.saturating_add(T::DbWeight::get().reads(604))
			.saturating_add(T::DbWeight::get().writes(603))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Common PropertyPermissions (r:1 w:0)
	/// Proof: Common PropertyPermissions (max_values: None, max_size: Some(16598), added: 19073, mode: MaxEncodedLen)
	/// Storage: Refungible TokenProperties (r:1 w:1)
	/// Proof: Refungible TokenProperties (max_values: None, max_size: Some(2113954), added: 2116429, mode: MaxEncodedLen)
	fn set_token_property() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101515`
		//  Estimated: `2138762`
		// Minimum execution time: 117_346 nanoseconds.
		Weight::from_parts(120_912_000, 2138762)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Common PropertyPermissions (r:1 w:0)
	/// Proof: Common PropertyPermissions (max_values: None, max_size: Some(16598), added: 19073, mode: MaxEncodedLen)
	/// Storage: Refungible TokenProperties (r:1 w:1)
	/// Proof: Refungible TokenProperties (max_values: None, max_size: Some(2113954), added: 2116429, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 64]`.
	fn set_token_properties(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101515`
		//  Estimated: `2138762`
		// Minimum execution time: 124_473 nanoseconds.
		Weight::from_parts(88_297_514, 2138762)
			// Standard Error: 63_768
			.saturating_add(Weight::from_parts(65_004_177, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Common PropertyPermissions (r:1 w:0)
	/// Proof: Common PropertyPermissions (max_values: None, max_size: Some(16598), added: 19073, mode: MaxEncodedLen)
	/// Storage: Refungible TokenProperties (r:1 w:1)
	/// Proof: Refungible TokenProperties (max_values: None, max_size: Some(2113954), added: 2116429, mode: MaxEncodedLen)
	fn delete_token_property() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101515`
		//  Estimated: `2138762`
		// Minimum execution time: 109_190 nanoseconds.
		Weight::from_parts(115_937_000, 2138762)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Common PropertyPermissions (r:1 w:0)
	/// Proof: Common PropertyPermissions (max_values: None, max_size: Some(16598), added: 19073, mode: MaxEncodedLen)
	/// Storage: Refungible TokenProperties (r:1 w:1)
	/// Proof: Refungible TokenProperties (max_values: None, max_size: Some(2113954), added: 2116429, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 64]`.
	fn delete_token_properties(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101515`
		//  Estimated: `2138762`
		// Minimum execution time: 109_502 nanoseconds.
		Weight::from_parts(272_697_215, 2138762)
			// Standard Error: 239_257
			.saturating_add(Weight::from_parts(39_290_144, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Whitelist KeysOfInvestors (r:2 w:0)
	/// Proof: Whitelist KeysOfInvestors (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Whitelist Investors (r:2 w:0)
	/// Proof: Whitelist Investors (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Refungible Balance (r:2 w:2)
	/// Proof: Refungible Balance (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Refungible AccountBalance (r:1 w:1)
	/// Proof: Refungible AccountBalance (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Refungible Owned (r:0 w:1)
	/// Proof: Refungible Owned (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `56604`
		//  Estimated: `21147`
		// Minimum execution time: 90_784 nanoseconds.
		Weight::from_parts(92_432_000, 21147)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Whitelist KeysOfInvestors (r:2 w:0)
	/// Proof: Whitelist KeysOfInvestors (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Whitelist Investors (r:2 w:0)
	/// Proof: Whitelist Investors (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Refungible Balance (r:1 w:0)
	/// Proof: Refungible Balance (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Refungible Allowance (r:0 w:1)
	/// Proof: Refungible Allowance (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	fn set_allowance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `55438`
		//  Estimated: `16045`
		// Minimum execution time: 73_581 nanoseconds.
		Weight::from_parts(80_298_000, 16045)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Whitelist KeysOfInvestors (r:3 w:0)
	/// Proof: Whitelist KeysOfInvestors (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Whitelist Investors (r:3 w:0)
	/// Proof: Whitelist Investors (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Refungible Allowance (r:1 w:1)
	/// Proof: Refungible Allowance (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Refungible Balance (r:2 w:2)
	/// Proof: Refungible Balance (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Refungible AccountBalance (r:1 w:1)
	/// Proof: Refungible AccountBalance (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Refungible Owned (r:0 w:1)
	/// Proof: Refungible Owned (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn transfer_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `57851`
		//  Estimated: `28837`
		// Minimum execution time: 116_717 nanoseconds.
		Weight::from_parts(120_344_000, 28837)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Refungible TotalSupply (r:1 w:1)
	/// Proof: Refungible TotalSupply (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Refungible Balance (r:1 w:1)
	/// Proof: Refungible Balance (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Refungible AccountBalance (r:1 w:1)
	/// Proof: Refungible AccountBalance (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Refungible Owned (r:0 w:1)
	/// Proof: Refungible Owned (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `53645`
		//  Estimated: `10877`
		// Minimum execution time: 70_751 nanoseconds.
		Weight::from_parts(73_218_000, 10877)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Whitelist KeysOfInvestors (r:2 w:0)
	/// Proof: Whitelist KeysOfInvestors (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Whitelist Investors (r:2 w:0)
	/// Proof: Whitelist Investors (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Refungible Allowance (r:1 w:1)
	/// Proof: Refungible Allowance (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Refungible TotalSupply (r:1 w:1)
	/// Proof: Refungible TotalSupply (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Refungible Balance (r:1 w:1)
	/// Proof: Refungible Balance (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Refungible AccountBalance (r:1 w:1)
	/// Proof: Refungible AccountBalance (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Refungible Owned (r:0 w:1)
	/// Proof: Refungible Owned (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn burn_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `56508`
		//  Estimated: `23678`
		// Minimum execution time: 103_782 nanoseconds.
		Weight::from_parts(106_585_000, 23678)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Refungible TotalSupply (r:1 w:1)
	/// Proof: Refungible TotalSupply (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Refungible Balance (r:1 w:1)
	/// Proof: Refungible Balance (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn repartition() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `19937`
		//  Estimated: `8338`
		// Minimum execution time: 40_185 nanoseconds.
		Weight::from_parts(41_695_000, 8338)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	/// Storage: Whitelist Admins (r:1 w:0)
	/// Proof: Whitelist Admins (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Common IsAdmin (r:1 w:1)
	/// Proof: Common IsAdmin (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Common AdminAmount (r:1 w:1)
	/// Proof: Common AdminAmount (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn toggle_admin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `51913`
		//  Estimated: `10827`
		// Minimum execution time: 50_701 nanoseconds.
		Weight::from_parts(52_476_000, 10827)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Common CollectionById (r:1 w:1)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	fn set_sponsor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2032`
		//  Estimated: `3260`
		// Minimum execution time: 20_021 nanoseconds.
		Weight::from_parts(20_786_000, 3260)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:1)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	fn confirm_sponsorship() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2104`
		//  Estimated: `3260`
		// Minimum execution time: 21_028 nanoseconds.
		Weight::from_parts(22_499_000, 3260)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Common CollectionById (r:1 w:1)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(785), added: 3260, mode: MaxEncodedLen)
	fn remove_sponsor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2144`
		//  Estimated: `3260`
		// Minimum execution time: 20_890 nanoseconds.
		Weight::from_parts(25_961_000, 3260)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
