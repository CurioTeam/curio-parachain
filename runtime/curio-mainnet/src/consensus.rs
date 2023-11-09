use crate::*;

parameter_types! {
    pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = MaxAuthorities;
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

parameter_types! {
    pub const MinBlocksPerRound: BlockNumber = HOURS;
    pub const DefaultBlocksPerRound: BlockNumber = 2 * HOURS;

    pub const StakeDuration: BlockNumber = 7 * DAYS;
    pub const ExitQueueDelay: u32 = 2;

    pub const MinCollators: u32 = 8;
    pub const MinRequiredCollators: u32 = 4;
    #[derive(Debug, Eq, PartialEq)]
    pub const MaxCollatorCandidates: u32 = 150;

    pub const MaxDelegationsPerRound: u32 = 1;
    #[derive(Debug, Eq, PartialEq)]
    pub const MaxDelegatorsPerCollator: u32 = 70;

    pub const MinCollatorStake: u128 = 10_000 * DOLLARS;
    pub const MinDelegatorStk: u128 = 100 * DOLLARS;

    pub const MaxUnstakeRequests: u32 = 10;

    pub const NetworkRewardStart: BlockNumber = 0;
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

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
    type EventHandler = ParachainStaking;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}
