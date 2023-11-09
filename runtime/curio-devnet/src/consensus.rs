use crate::{
    Aura,
    AuraId,
    Balance,
    Balances,
    BlockNumber,
    BLOCKS_PER_YEAR,
    DAYS,
    DOLLARS,
    ParachainStaking,
    Runtime,
    RuntimeEvent,
    Treasury,
    weights,
};

use frame_support::{
    parameter_types
};
use sp_std::{
    vec::Vec
};
use sp_runtime::{
    Perquintill,
    impl_opaque_keys,
    traits::{
        ConvertInto
    }
};

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type EventHandler = ParachainStaking;
}

parameter_types! {
	pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const MinDelegatorStk: u128 = 1_000 * DOLLARS;
	/// Minimum round length is 1 hour
	pub const MinBlocksPerRound: BlockNumber = 5;
	/// Default length of a round/session is 2 hours
	pub const DefaultBlocksPerRound: BlockNumber = 10;
	/// Unstaked balance can be unlocked after 7 days
	pub const StakeDuration: BlockNumber = 7 * DAYS;
	/// Collator exit requests are delayed by 4 hours (2 rounds/sessions)
	pub const ExitQueueDelay: u32 = 2;
	/// Minimum 16 collators selected per round, default at genesis and minimum forever after
	pub const MinCollators: u32 = 2;
	/// At least 4 candidates which cannot leave the network if there are no other candidates.
	pub const MinRequiredCollators: u32 = 2;
	/// We only allow one delegation per round.
	pub const MaxDelegationsPerRound: u32 = 1;
	/// Maximum 35 delegators per collator at launch, might be increased later
	#[derive(Debug, Eq, PartialEq)]
	pub const MaxDelegatorsPerCollator: u32 = 70;
	/// Minimum stake required to become a collator
	pub const MinCollatorStake: u128 = 10_000 * DOLLARS;
	/// Maximum number of collator candidates
	#[derive(Debug, Eq, PartialEq)]
	pub const MaxCollatorCandidates: u32 = 150;
	/// Maximum number of concurrent requests to unlock unstaked balance
	pub const MaxUnstakeRequests: u32 = 10;
	/// The starting block number for the network rewards
	pub const NetworkRewardStart: BlockNumber = 0;
	/// The rate in percent for the network rewards
	pub const NetworkRewardRate: Perquintill = Perquintill::from_percent(10);
	
}

impl parachain_staking::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	
	type MinBlocksPerRound = MinBlocksPerRound;
	type DefaultBlocksPerRound = DefaultBlocksPerRound;
	type StakeDuration = StakeDuration;
	type ExitQueueDelay = ExitQueueDelay;
	type MinCollators = MinCollators;
	type MinRequiredCollators = MinRequiredCollators;
	type MaxDelegationsPerRound = MaxDelegationsPerRound;
	type MaxDelegatorsPerCollator = MaxDelegatorsPerCollator;
	type MinCollatorStake = MinCollatorStake;
	type MinCollatorCandidateStake = MinCollatorStake;
	type MaxTopCandidates = MaxCollatorCandidates;
	type MinDelegatorStake = MinDelegatorStk;
	type MaxUnstakeRequests = MaxUnstakeRequests;
	type NetworkRewardRate = NetworkRewardRate;
	type NetworkRewardStart = NetworkRewardStart;
	type NetworkRewardBeneficiary = Treasury;
	type WeightInfo = weights::parachain_staking::WeightInfo<Runtime>;
	
	const BLOCKS_PER_YEAR: Self::BlockNumber = BLOCKS_PER_YEAR;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = ConvertInto;
	type ShouldEndSession = ParachainStaking;
	type NextSessionRotation = ParachainStaking;
	type SessionManager = ParachainStaking;
	// Essentially just Aura, but lets be pedantic.
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = weights::pallet_session::WeightInfo<Runtime>;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}