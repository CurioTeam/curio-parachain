use frame_support::pallet_prelude::*;
use frame_support::weights::constants::RocksDbWeight;

/// Weight functions needed for pallet_whitelist.
pub trait WeightInfo {
	fn add_admin() -> Weight;
    fn remove_admin() -> Weight;
    fn add_manager() -> Weight;
    fn remove_manager() -> Weight;
    fn add_investors(i: u32, ) -> Weight;
    fn set_investor_status() -> Weight;
    fn change_investor_address() -> Weight;
    fn change_my_address() -> Weight;
}

impl WeightInfo for () {
	// Storage: Whitelist Admins (r:1 w:1)
	fn add_admin() -> Weight {
		// Minimum execution time: 51_837 nanoseconds.
		Weight::from_ref_time(52_708_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Whitelist Admins (r:1 w:1)
	fn remove_admin() -> Weight {
		// Minimum execution time: 52_949 nanoseconds.
		Weight::from_ref_time(55_784_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Whitelist Admins (r:1 w:0)
	// Storage: Whitelist Managers (r:1 w:1)
	fn add_manager() -> Weight {
		// Minimum execution time: 59_161 nanoseconds.
		Weight::from_ref_time(60_283_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Whitelist Admins (r:1 w:0)
	// Storage: Whitelist Managers (r:1 w:1)
	fn remove_manager() -> Weight {
		// Minimum execution time: 72_556 nanoseconds.
		Weight::from_ref_time(77_645_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Whitelist Admins (r:1 w:0)
	// Storage: Whitelist KeysOfInvestors (r:1 w:1)
	// Storage: Whitelist Investors (r:1 w:1)
	/// The range of component `i` is `[1, 100]`.
	fn add_investors(i: u32, ) -> Weight {
		// Minimum execution time: 74_009 nanoseconds.
		Weight::from_ref_time(70_891_339 as u64)
			// Standard Error: 97_869
			.saturating_add(Weight::from_ref_time(18_466_297 as u64).saturating_mul(i as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(i as u64)))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(i as u64)))
	}
	// Storage: Whitelist Admins (r:1 w:0)
	// Storage: Whitelist KeysOfInvestors (r:1 w:0)
	// Storage: Whitelist Investors (r:1 w:1)
	fn set_investor_status() -> Weight {
		// Minimum execution time: 55_704 nanoseconds.
		Weight::from_ref_time(56_556_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Whitelist Admins (r:1 w:0)
	// Storage: Whitelist KeysOfInvestors (r:1 w:2)
	// Storage: Whitelist Investors (r:1 w:1)
	fn change_investor_address() -> Weight {
		// Minimum execution time: 78_687 nanoseconds.
		Weight::from_ref_time(92_323_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Whitelist KeysOfInvestors (r:1 w:2)
	// Storage: Whitelist Investors (r:1 w:1)
	fn change_my_address() -> Weight {
		// Minimum execution time: 71_744 nanoseconds.
		Weight::from_ref_time(75_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
}