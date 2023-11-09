// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/errors';

import type { ApiTypes, AugmentedError } from '@polkadot/api-base/types';

export type __AugmentedError<ApiType extends ApiTypes> = AugmentedError<ApiType>;

declare module '@polkadot/api-base/types/errors' {
  interface AugmentedErrors<ApiType extends ApiTypes> {
    authorship: {
      /**
       * The uncle is genesis.
       **/
      GenesisUncle: AugmentedError<ApiType>;
      /**
       * The uncle parent not in the chain.
       **/
      InvalidUncleParent: AugmentedError<ApiType>;
      /**
       * The uncle isn't recent enough to be included.
       **/
      OldUncle: AugmentedError<ApiType>;
      /**
       * The uncle is too high in chain.
       **/
      TooHighUncle: AugmentedError<ApiType>;
      /**
       * Too many uncles.
       **/
      TooManyUncles: AugmentedError<ApiType>;
      /**
       * The uncle is already included.
       **/
      UncleAlreadyIncluded: AugmentedError<ApiType>;
      /**
       * Uncles already set in the block.
       **/
      UnclesAlreadySet: AugmentedError<ApiType>;
    };
    balances: {
      /**
       * Beneficiary account must pre-exist
       **/
      DeadAccount: AugmentedError<ApiType>;
      /**
       * Value too low to create account due to existential deposit
       **/
      ExistentialDeposit: AugmentedError<ApiType>;
      /**
       * A vesting schedule already exists for this account
       **/
      ExistingVestingSchedule: AugmentedError<ApiType>;
      /**
       * Balance too low to send value
       **/
      InsufficientBalance: AugmentedError<ApiType>;
      /**
       * Transfer/payment would kill account
       **/
      KeepAlive: AugmentedError<ApiType>;
      /**
       * Account liquidity restrictions prevent withdrawal
       **/
      LiquidityRestrictions: AugmentedError<ApiType>;
      /**
       * Number of named reserves exceed MaxReserves
       **/
      TooManyReserves: AugmentedError<ApiType>;
      /**
       * Vesting balance too high to send value
       **/
      VestingBalance: AugmentedError<ApiType>;
    };
    bounties: {
      /**
       * The bounty cannot be closed because it has active child bounties.
       **/
      HasActiveChildBounty: AugmentedError<ApiType>;
      /**
       * Proposer's balance is too low.
       **/
      InsufficientProposersBalance: AugmentedError<ApiType>;
      /**
       * Invalid bounty fee.
       **/
      InvalidFee: AugmentedError<ApiType>;
      /**
       * No proposal or bounty at that index.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Invalid bounty value.
       **/
      InvalidValue: AugmentedError<ApiType>;
      /**
       * A bounty payout is pending.
       * To cancel the bounty, you must unassign and slash the curator.
       **/
      PendingPayout: AugmentedError<ApiType>;
      /**
       * The bounties cannot be claimed/closed because it's still in the countdown period.
       **/
      Premature: AugmentedError<ApiType>;
      /**
       * The reason given is just too big.
       **/
      ReasonTooBig: AugmentedError<ApiType>;
      /**
       * Require bounty curator.
       **/
      RequireCurator: AugmentedError<ApiType>;
      /**
       * Too many approvals are already queued.
       **/
      TooManyQueued: AugmentedError<ApiType>;
      /**
       * The bounty status is unexpected.
       **/
      UnexpectedStatus: AugmentedError<ApiType>;
    };
    common: {
      /**
       * Account token limit exceeded per collection
       **/
      AccountTokenLimitExceeded: AugmentedError<ApiType>;
      /**
       * Can't transfer tokens to ethereum zero address
       **/
      AddressIsZero: AugmentedError<ApiType>;
      /**
       * Address is not in allow list.
       **/
      AddressNotInAllowlist: AugmentedError<ApiType>;
      /**
       * Requested value is more than the approved
       **/
      ApprovedValueTooLow: AugmentedError<ApiType>;
      /**
       * Tried to approve more than owned
       **/
      CantApproveMoreThanOwned: AugmentedError<ApiType>;
      /**
       * Destroying only empty collections is allowed
       **/
      CantDestroyNotEmptyCollection: AugmentedError<ApiType>;
      /**
       * Exceeded max admin count
       **/
      CollectionAdminCountExceeded: AugmentedError<ApiType>;
      /**
       * Collection description can not be longer than 255 char.
       **/
      CollectionDescriptionLimitExceeded: AugmentedError<ApiType>;
      /**
       * Tried to store more data than allowed in collection field
       **/
      CollectionFieldSizeExceeded: AugmentedError<ApiType>;
      /**
       * Tried to access an external collection with an internal API
       **/
      CollectionIsExternal: AugmentedError<ApiType>;
      /**
       * Tried to access an internal collection with an external API
       **/
      CollectionIsInternal: AugmentedError<ApiType>;
      /**
       * Collection limit bounds per collection exceeded
       **/
      CollectionLimitBoundsExceeded: AugmentedError<ApiType>;
      /**
       * Collection name can not be longer than 63 char.
       **/
      CollectionNameLimitExceeded: AugmentedError<ApiType>;
      /**
       * This collection does not exist.
       **/
      CollectionNotFound: AugmentedError<ApiType>;
      /**
       * Collection token limit exceeded
       **/
      CollectionTokenLimitExceeded: AugmentedError<ApiType>;
      /**
       * Token prefix can not be longer than 15 char.
       **/
      CollectionTokenPrefixLimitExceeded: AugmentedError<ApiType>;
      /**
       * Empty property keys are forbidden
       **/
      EmptyPropertyKey: AugmentedError<ApiType>;
      /**
       * Only ASCII letters, digits, and symbols `_`, `-`, and `.` are allowed
       **/
      InvalidCharacterInPropertyKey: AugmentedError<ApiType>;
      /**
       * Metadata flag frozen
       **/
      MetadataFlagFrozen: AugmentedError<ApiType>;
      /**
       * Sender parameter and item owner must be equal.
       **/
      MustBeTokenOwner: AugmentedError<ApiType>;
      /**
       * No permission to perform action
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * Tried to store more property data than allowed
       **/
      NoSpaceForProperty: AugmentedError<ApiType>;
      /**
       * Insufficient funds to perform an action
       **/
      NotSufficientFounds: AugmentedError<ApiType>;
      /**
       * Tried to enable permissions which are only permitted to be disabled
       **/
      OwnerPermissionsCantBeReverted: AugmentedError<ApiType>;
      /**
       * Property key is too long
       **/
      PropertyKeyIsTooLong: AugmentedError<ApiType>;
      /**
       * Tried to store more property keys than allowed
       **/
      PropertyLimitReached: AugmentedError<ApiType>;
      /**
       * Collection is not in mint mode.
       **/
      PublicMintingNotAllowed: AugmentedError<ApiType>;
      /**
       * Only tokens from specific collections may nest tokens under this one
       **/
      SourceCollectionIsNotAllowedToNest: AugmentedError<ApiType>;
      /**
       * Item does not exist
       **/
      TokenNotFound: AugmentedError<ApiType>;
      /**
       * Item is balance not enough
       **/
      TokenValueTooLow: AugmentedError<ApiType>;
      /**
       * Total collections bound exceeded.
       **/
      TotalCollectionsLimitExceeded: AugmentedError<ApiType>;
      /**
       * Collection settings not allowing items transferring
       **/
      TransfersDisabled: AugmentedError<ApiType>;
      /**
       * Property permission set that property is unmutable
       **/
      UnmutableProperty: AugmentedError<ApiType>;
      /**
       * The operation is not supported
       **/
      UnsupportedOperation: AugmentedError<ApiType>;
      /**
       * User does not satisfy the nesting rule
       **/
      UserIsNotAllowedToNest: AugmentedError<ApiType>;
    };
    council: {
      /**
       * Members are already initialized!
       **/
      AlreadyInitialized: AugmentedError<ApiType>;
      /**
       * Duplicate proposals not allowed
       **/
      DuplicateProposal: AugmentedError<ApiType>;
      /**
       * Duplicate vote ignored
       **/
      DuplicateVote: AugmentedError<ApiType>;
      /**
       * Account is not a member
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Proposal must exist
       **/
      ProposalMissing: AugmentedError<ApiType>;
      /**
       * The close call was made too early, before the end of the voting.
       **/
      TooEarly: AugmentedError<ApiType>;
      /**
       * There can only be a maximum of `MaxProposals` active proposals.
       **/
      TooManyProposals: AugmentedError<ApiType>;
      /**
       * Mismatched index
       **/
      WrongIndex: AugmentedError<ApiType>;
      /**
       * The given length bound for the proposal was too low.
       **/
      WrongProposalLength: AugmentedError<ApiType>;
      /**
       * The given weight bound for the proposal was too low.
       **/
      WrongProposalWeight: AugmentedError<ApiType>;
    };
    cumulusXcm: {
    };
    currencies: {
      /**
       * Unable to convert the Amount type into Balance.
       **/
      AmountIntoBalanceFailed: AugmentedError<ApiType>;
      /**
       * Balance is too low.
       **/
      BalanceTooLow: AugmentedError<ApiType>;
      /**
       * Deposit result is not expected
       **/
      DepositFailed: AugmentedError<ApiType>;
    };
    democracy: {
      /**
       * Cannot cancel the same proposal twice
       **/
      AlreadyCanceled: AugmentedError<ApiType>;
      /**
       * The account is already delegating.
       **/
      AlreadyDelegating: AugmentedError<ApiType>;
      /**
       * Identity may not veto a proposal twice
       **/
      AlreadyVetoed: AugmentedError<ApiType>;
      /**
       * Proposal already made
       **/
      DuplicateProposal: AugmentedError<ApiType>;
      /**
       * The instant referendum origin is currently disallowed.
       **/
      InstantNotAllowed: AugmentedError<ApiType>;
      /**
       * Too high a balance was provided that the account cannot afford.
       **/
      InsufficientFunds: AugmentedError<ApiType>;
      /**
       * Invalid hash
       **/
      InvalidHash: AugmentedError<ApiType>;
      /**
       * Maximum number of votes reached.
       **/
      MaxVotesReached: AugmentedError<ApiType>;
      /**
       * No proposals waiting
       **/
      NoneWaiting: AugmentedError<ApiType>;
      /**
       * Delegation to oneself makes no sense.
       **/
      Nonsense: AugmentedError<ApiType>;
      /**
       * The actor has no permission to conduct the action.
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * No external proposal
       **/
      NoProposal: AugmentedError<ApiType>;
      /**
       * The account is not currently delegating.
       **/
      NotDelegating: AugmentedError<ApiType>;
      /**
       * Next external proposal not simple majority
       **/
      NotSimpleMajority: AugmentedError<ApiType>;
      /**
       * The given account did not vote on the referendum.
       **/
      NotVoter: AugmentedError<ApiType>;
      /**
       * Proposal still blacklisted
       **/
      ProposalBlacklisted: AugmentedError<ApiType>;
      /**
       * Proposal does not exist
       **/
      ProposalMissing: AugmentedError<ApiType>;
      /**
       * Vote given for invalid referendum
       **/
      ReferendumInvalid: AugmentedError<ApiType>;
      /**
       * Maximum number of items reached.
       **/
      TooMany: AugmentedError<ApiType>;
      /**
       * Value too low
       **/
      ValueLow: AugmentedError<ApiType>;
      /**
       * The account currently has votes attached to it and the operation cannot succeed until
       * these are removed, either through `unvote` or `reap_vote`.
       **/
      VotesExist: AugmentedError<ApiType>;
      /**
       * Voting period too low
       **/
      VotingPeriodLow: AugmentedError<ApiType>;
      /**
       * Invalid upper bound.
       **/
      WrongUpperBound: AugmentedError<ApiType>;
    };
    dex: {
      /**
       * Trading pair is already Enabled
       **/
      AlreadyEnabled: AugmentedError<ApiType>;
      /**
       * Cannot swap
       **/
      CannotSwap: AugmentedError<ApiType>;
      /**
       * Supply amount is more than max_supply_amount
       **/
      ExcessiveSupplyAmount: AugmentedError<ApiType>;
      /**
       * Liquidity is not enough
       **/
      InsufficientLiquidity: AugmentedError<ApiType>;
      /**
       * Target amount is less to min_target_amount
       **/
      InsufficientTargetAmount: AugmentedError<ApiType>;
      /**
       * The increment of provision is invalid
       **/
      InvalidContributionIncrement: AugmentedError<ApiType>;
      /**
       * Invalid currency id
       **/
      InvalidCurrencyId: AugmentedError<ApiType>;
      /**
       * The increment of liquidity is invalid
       **/
      InvalidLiquidityIncrement: AugmentedError<ApiType>;
      /**
       * The trading path is invalid
       **/
      InvalidTradingPath: AugmentedError<ApiType>;
      /**
       * Invalid trading path length
       **/
      InvalidTradingPathLength: AugmentedError<ApiType>;
      /**
       * The swap dosen't meet the invariant check
       **/
      InvariantCheckFailed: AugmentedError<ApiType>;
      /**
       * Trading pair must be in Disabled status
       **/
      MustBeDisabled: AugmentedError<ApiType>;
      /**
       * Trading pair must be in Enabled status
       **/
      MustBeEnabled: AugmentedError<ApiType>;
      /**
       * Trading pair must be in Provisioning status
       **/
      MustBeProvisioning: AugmentedError<ApiType>;
      /**
       * This trading pair is not allowed to be listed
       **/
      NotAllowedList: AugmentedError<ApiType>;
      /**
       * Not allowed to refund provision
       **/
      NotAllowedRefund: AugmentedError<ApiType>;
      /**
       * Trading pair is still provisioning
       **/
      StillProvisioning: AugmentedError<ApiType>;
      /**
       * The liquidity withdrawn is unacceptable
       **/
      UnacceptableLiquidityWithdrawn: AugmentedError<ApiType>;
      /**
       * The share increment is unacceptable
       **/
      UnacceptableShareIncrement: AugmentedError<ApiType>;
      /**
       * The Provision is unqualified to be converted to `Enabled`
       **/
      UnqualifiedProvision: AugmentedError<ApiType>;
      /**
       * The supply amount is zero
       **/
      ZeroSupplyAmount: AugmentedError<ApiType>;
      /**
       * The target amount is zero
       **/
      ZeroTargetAmount: AugmentedError<ApiType>;
    };
    dmpQueue: {
      /**
       * The amount of weight given is possibly not enough for executing the message.
       **/
      OverLimit: AugmentedError<ApiType>;
      /**
       * The message index given is unknown.
       **/
      Unknown: AugmentedError<ApiType>;
    };
    elections: {
      /**
       * Duplicated candidate submission.
       **/
      DuplicatedCandidate: AugmentedError<ApiType>;
      /**
       * Candidate does not have enough funds.
       **/
      InsufficientCandidateFunds: AugmentedError<ApiType>;
      /**
       * The renouncing origin presented a wrong `Renouncing` parameter.
       **/
      InvalidRenouncing: AugmentedError<ApiType>;
      /**
       * Prediction regarding replacement after member removal is wrong.
       **/
      InvalidReplacement: AugmentedError<ApiType>;
      /**
       * The provided count of number of votes is incorrect.
       **/
      InvalidVoteCount: AugmentedError<ApiType>;
      /**
       * The provided count of number of candidates is incorrect.
       **/
      InvalidWitnessData: AugmentedError<ApiType>;
      /**
       * Cannot vote with stake less than minimum balance.
       **/
      LowBalance: AugmentedError<ApiType>;
      /**
       * Cannot vote more than maximum allowed.
       **/
      MaximumVotesExceeded: AugmentedError<ApiType>;
      /**
       * Member cannot re-submit candidacy.
       **/
      MemberSubmit: AugmentedError<ApiType>;
      /**
       * Must be a voter.
       **/
      MustBeVoter: AugmentedError<ApiType>;
      /**
       * Not a member.
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Must vote for at least one candidate.
       **/
      NoVotes: AugmentedError<ApiType>;
      /**
       * Runner cannot re-submit candidacy.
       **/
      RunnerUpSubmit: AugmentedError<ApiType>;
      /**
       * Too many candidates have been created.
       **/
      TooManyCandidates: AugmentedError<ApiType>;
      /**
       * Cannot vote more than candidates.
       **/
      TooManyVotes: AugmentedError<ApiType>;
      /**
       * Voter can not pay voting bond.
       **/
      UnableToPayBond: AugmentedError<ApiType>;
      /**
       * Cannot vote when no candidates or members exist.
       **/
      UnableToVote: AugmentedError<ApiType>;
    };
    identity: {
      /**
       * Account ID is already named.
       **/
      AlreadyClaimed: AugmentedError<ApiType>;
      /**
       * Empty index.
       **/
      EmptyIndex: AugmentedError<ApiType>;
      /**
       * Fee is changed.
       **/
      FeeChanged: AugmentedError<ApiType>;
      /**
       * The index is invalid.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Invalid judgement.
       **/
      InvalidJudgement: AugmentedError<ApiType>;
      /**
       * The target is invalid.
       **/
      InvalidTarget: AugmentedError<ApiType>;
      /**
       * The provided judgement was for a different identity.
       **/
      JudgementForDifferentIdentity: AugmentedError<ApiType>;
      /**
       * Judgement given.
       **/
      JudgementGiven: AugmentedError<ApiType>;
      /**
       * No identity found.
       **/
      NoIdentity: AugmentedError<ApiType>;
      /**
       * Account isn't found.
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * Account isn't named.
       **/
      NotNamed: AugmentedError<ApiType>;
      /**
       * Sub-account isn't owned by sender.
       **/
      NotOwned: AugmentedError<ApiType>;
      /**
       * Sender is not a sub-account.
       **/
      NotSub: AugmentedError<ApiType>;
      /**
       * Sticky judgement.
       **/
      StickyJudgement: AugmentedError<ApiType>;
      /**
       * Too many additional fields.
       **/
      TooManyFields: AugmentedError<ApiType>;
      /**
       * Maximum amount of registrars reached. Cannot add any more.
       **/
      TooManyRegistrars: AugmentedError<ApiType>;
      /**
       * Too many subs-accounts.
       **/
      TooManySubAccounts: AugmentedError<ApiType>;
    };
    indices: {
      /**
       * The index was not available.
       **/
      InUse: AugmentedError<ApiType>;
      /**
       * The index was not already assigned.
       **/
      NotAssigned: AugmentedError<ApiType>;
      /**
       * The index is assigned to another account.
       **/
      NotOwner: AugmentedError<ApiType>;
      /**
       * The source and destination accounts are identical.
       **/
      NotTransfer: AugmentedError<ApiType>;
      /**
       * The index is permanent and may not be freed/changed.
       **/
      Permanent: AugmentedError<ApiType>;
    };
    multisig: {
      /**
       * Call is already approved by this signatory.
       **/
      AlreadyApproved: AugmentedError<ApiType>;
      /**
       * The data to be stored is already stored.
       **/
      AlreadyStored: AugmentedError<ApiType>;
      /**
       * The maximum weight information provided was too low.
       **/
      MaxWeightTooLow: AugmentedError<ApiType>;
      /**
       * Threshold must be 2 or greater.
       **/
      MinimumThreshold: AugmentedError<ApiType>;
      /**
       * Call doesn't need any (more) approvals.
       **/
      NoApprovalsNeeded: AugmentedError<ApiType>;
      /**
       * Multisig operation not found when attempting to cancel.
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * No timepoint was given, yet the multisig operation is already underway.
       **/
      NoTimepoint: AugmentedError<ApiType>;
      /**
       * Only the account that originally created the multisig is able to cancel it.
       **/
      NotOwner: AugmentedError<ApiType>;
      /**
       * The sender was contained in the other signatories; it shouldn't be.
       **/
      SenderInSignatories: AugmentedError<ApiType>;
      /**
       * The signatories were provided out of order; they should be ordered.
       **/
      SignatoriesOutOfOrder: AugmentedError<ApiType>;
      /**
       * There are too few signatories in the list.
       **/
      TooFewSignatories: AugmentedError<ApiType>;
      /**
       * There are too many signatories in the list.
       **/
      TooManySignatories: AugmentedError<ApiType>;
      /**
       * A timepoint was given, yet no multisig operation is underway.
       **/
      UnexpectedTimepoint: AugmentedError<ApiType>;
      /**
       * A different timepoint was given to the multisig operation that is underway.
       **/
      WrongTimepoint: AugmentedError<ApiType>;
    };
    parachainStaking: {
      /**
       * The delegator has already previously delegated the collator
       * candidate.
       **/
      AlreadyDelegatedCollator: AugmentedError<ApiType>;
      /**
       * The account is already delegating the collator candidate.
       **/
      AlreadyDelegating: AugmentedError<ApiType>;
      /**
       * The collator candidate has already trigger the process to leave the
       * set of collator candidates.
       **/
      AlreadyLeaving: AugmentedError<ApiType>;
      /**
       * The account is already part of the collator candidates set.
       **/
      CandidateExists: AugmentedError<ApiType>;
      /**
       * The account is not part of the collator candidates set.
       **/
      CandidateNotFound: AugmentedError<ApiType>;
      /**
       * The collator candidate is in the process of leaving the set of
       * candidates and thus cannot be delegated to.
       **/
      CannotDelegateIfLeaving: AugmentedError<ApiType>;
      /**
       * The account has a full list of unstaking requests and needs to
       * unlock at least one of these before being able to join (again).
       * NOTE: Can only happen if the account was a candidate or
       * delegator before and either got kicked or exited voluntarily.
       **/
      CannotJoinBeforeUnlocking: AugmentedError<ApiType>;
      /**
       * The collator tried to leave before waiting at least for
       * `ExitQueueDelay` many rounds.
       **/
      CannotLeaveYet: AugmentedError<ApiType>;
      /**
       * The number of selected candidates per staking round is
       * above the maximum value allowed.
       **/
      CannotSetAboveMax: AugmentedError<ApiType>;
      /**
       * The number of selected candidates per staking round is
       * below the minimum value allowed.
       **/
      CannotSetBelowMin: AugmentedError<ApiType>;
      /**
       * The collator candidate is in the process of leaving the set of
       * candidates and cannot perform any other actions in the meantime.
       **/
      CannotStakeIfLeaving: AugmentedError<ApiType>;
      /**
       * The account has not staked enough funds to delegate a collator
       * candidate.
       **/
      DelegationBelowMin: AugmentedError<ApiType>;
      /**
       * The given delegation does not exist in the set of delegations.
       **/
      DelegationNotFound: AugmentedError<ApiType>;
      /**
       * The delegator has exceeded the number of delegations per round which
       * is equal to MaxDelegatorsPerCollator.
       * 
       * This protects against attacks in which a delegator can re-delegate
       * from a collator who has already authored a block, to another one
       * which has not in this round.
       **/
      DelegationsPerRoundExceeded: AugmentedError<ApiType>;
      /**
       * The account is already part of the delegators set.
       **/
      DelegatorExists: AugmentedError<ApiType>;
      /**
       * The account is not part of the delegators set.
       **/
      DelegatorNotFound: AugmentedError<ApiType>;
      /**
       * An invalid inflation configuration is trying to be set.
       **/
      InvalidSchedule: AugmentedError<ApiType>;
      /**
       * The delegator has already delegated the maximum number of candidates
       * allowed.
       **/
      MaxCollatorsPerDelegatorExceeded: AugmentedError<ApiType>;
      /**
       * The staking reward being unlocked does not exist.
       * Max unlocking requests reached.
       **/
      NoMoreUnstaking: AugmentedError<ApiType>;
      /**
       * The collator candidate wanted to execute the exit but has not
       * requested to leave before by calling `init_leave_candidates`.
       **/
      NotLeaving: AugmentedError<ApiType>;
      /**
       * The account has not delegated any collator candidate yet, hence it
       * is not in the set of delegators.
       **/
      NotYetDelegating: AugmentedError<ApiType>;
      /**
       * Cannot claim rewards if empty.
       **/
      RewardsNotFound: AugmentedError<ApiType>;
      /**
       * Provided staked value is zero. Should never be thrown.
       **/
      StakeNotFound: AugmentedError<ApiType>;
      /**
       * The reward rate cannot be adjusted yet as an entire year has not
       * passed.
       **/
      TooEarly: AugmentedError<ApiType>;
      /**
       * The set of collator candidates would fall below the required minimum
       * if the collator left.
       **/
      TooFewCollatorCandidates: AugmentedError<ApiType>;
      /**
       * The collator candidate has already reached the maximum number of
       * delegators.
       * 
       * This error is generated in case a new delegation request does not
       * stake enough funds to replace some other existing delegation.
       **/
      TooManyDelegators: AugmentedError<ApiType>;
      /**
       * The collator delegate or the delegator is trying to un-stake more
       * funds that are currently staked.
       **/
      Underflow: AugmentedError<ApiType>;
      /**
       * Cannot unlock when Unstaked is empty.
       **/
      UnstakingIsEmpty: AugmentedError<ApiType>;
      /**
       * The account has already staked the maximum amount of funds possible.
       **/
      ValStakeAboveMax: AugmentedError<ApiType>;
      /**
       * The account has not staked enough funds to be added to the collator
       * candidates set.
       **/
      ValStakeBelowMin: AugmentedError<ApiType>;
      /**
       * The account tried to stake more or less with amount zero.
       **/
      ValStakeZero: AugmentedError<ApiType>;
    };
    parachainSystem: {
      /**
       * The inherent which supplies the host configuration did not run this block
       **/
      HostConfigurationNotAvailable: AugmentedError<ApiType>;
      /**
       * No code upgrade has been authorized.
       **/
      NothingAuthorized: AugmentedError<ApiType>;
      /**
       * No validation function upgrade is currently scheduled.
       **/
      NotScheduled: AugmentedError<ApiType>;
      /**
       * Attempt to upgrade validation function while existing upgrade pending
       **/
      OverlappingUpgrades: AugmentedError<ApiType>;
      /**
       * Polkadot currently prohibits this parachain from upgrading its validation function
       **/
      ProhibitedByPolkadot: AugmentedError<ApiType>;
      /**
       * The supplied validation function has compiled into a blob larger than Polkadot is
       * willing to run
       **/
      TooBig: AugmentedError<ApiType>;
      /**
       * The given code upgrade has not been authorized.
       **/
      Unauthorized: AugmentedError<ApiType>;
      /**
       * The inherent which supplies the validation data did not run this block
       **/
      ValidationDataNotAvailable: AugmentedError<ApiType>;
    };
    polkadotXcm: {
      /**
       * The location is invalid since it already has a subscription from us.
       **/
      AlreadySubscribed: AugmentedError<ApiType>;
      /**
       * The given location could not be used (e.g. because it cannot be expressed in the
       * desired version of XCM).
       **/
      BadLocation: AugmentedError<ApiType>;
      /**
       * The version of the `Versioned` value used is not able to be interpreted.
       **/
      BadVersion: AugmentedError<ApiType>;
      /**
       * Could not re-anchor the assets to declare the fees for the destination chain.
       **/
      CannotReanchor: AugmentedError<ApiType>;
      /**
       * The destination `MultiLocation` provided cannot be inverted.
       **/
      DestinationNotInvertible: AugmentedError<ApiType>;
      /**
       * The assets to be sent are empty.
       **/
      Empty: AugmentedError<ApiType>;
      /**
       * The message execution fails the filter.
       **/
      Filtered: AugmentedError<ApiType>;
      /**
       * Origin is invalid for sending.
       **/
      InvalidOrigin: AugmentedError<ApiType>;
      /**
       * The referenced subscription could not be found.
       **/
      NoSubscription: AugmentedError<ApiType>;
      /**
       * There was some other issue (i.e. not to do with routing) in sending the message. Perhaps
       * a lack of space for buffering the message.
       **/
      SendFailure: AugmentedError<ApiType>;
      /**
       * Too many assets have been attempted for transfer.
       **/
      TooManyAssets: AugmentedError<ApiType>;
      /**
       * The desired destination was unreachable, generally because there is a no way of routing
       * to it.
       **/
      Unreachable: AugmentedError<ApiType>;
      /**
       * The message's weight could not be determined.
       **/
      UnweighableMessage: AugmentedError<ApiType>;
    };
    preimage: {
      /**
       * Preimage has already been noted on-chain.
       **/
      AlreadyNoted: AugmentedError<ApiType>;
      /**
       * The user is not authorized to perform this action.
       **/
      NotAuthorized: AugmentedError<ApiType>;
      /**
       * The preimage cannot be removed since it has not yet been noted.
       **/
      NotNoted: AugmentedError<ApiType>;
      /**
       * The preimage request cannot be removed since no outstanding requests exist.
       **/
      NotRequested: AugmentedError<ApiType>;
      /**
       * A preimage may not be removed when there are outstanding requests.
       **/
      Requested: AugmentedError<ApiType>;
      /**
       * Preimage is too large to store on-chain.
       **/
      TooBig: AugmentedError<ApiType>;
    };
    proxy: {
      /**
       * Account is already a proxy.
       **/
      Duplicate: AugmentedError<ApiType>;
      /**
       * Call may not be made by proxy because it may escalate its privileges.
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * Cannot add self as proxy.
       **/
      NoSelfProxy: AugmentedError<ApiType>;
      /**
       * Proxy registration not found.
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * Sender is not a proxy of the account to be proxied.
       **/
      NotProxy: AugmentedError<ApiType>;
      /**
       * There are too many proxies registered or too many announcements pending.
       **/
      TooMany: AugmentedError<ApiType>;
      /**
       * Announcement, if made at all, was made too recently.
       **/
      Unannounced: AugmentedError<ApiType>;
      /**
       * A call which is incompatible with the proxy type's filter was attempted.
       **/
      Unproxyable: AugmentedError<ApiType>;
    };
    refungible: {
      /**
       * Undefined error during input parsing
       **/
      InvalidInput: AugmentedError<ApiType>;
      /**
       * Not whitelisted or not active investor given
       **/
      NeitherWhitelistedNorCollectionAdmin: AugmentedError<ApiType>;
      /**
       * When creating multiple items balances and properties must be defined for each item
       **/
      NotCompleteItemsData: AugmentedError<ApiType>;
      /**
       * Not Refungible item data used to mint in Refungible collection.
       **/
      NotRefungibleDataUsedToMintFungibleCollectionToken: AugmentedError<ApiType>;
      /**
       * Refungible token can't nest other tokens.
       **/
      RefungibleDisallowsNesting: AugmentedError<ApiType>;
      /**
       * Refungible token can't be repartitioned by user who isn't owns all pieces.
       **/
      RepartitionWhileNotOwningAllPieces: AugmentedError<ApiType>;
      /**
       * Setting item properties is not allowed.
       **/
      SettingPropertiesNotAllowed: AugmentedError<ApiType>;
      /**
       * User duplicates given in initial token balances
       **/
      UserDuplicatesGiven: AugmentedError<ApiType>;
      /**
       * Maximum refungibility exceeded.
       **/
      WrongRefungiblePieces: AugmentedError<ApiType>;
    };
    scheduler: {
      /**
       * Failed to schedule a call
       **/
      FailedToSchedule: AugmentedError<ApiType>;
      /**
       * Attempt to use a non-named function on a named task.
       **/
      Named: AugmentedError<ApiType>;
      /**
       * Cannot find the scheduled call.
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * Reschedule failed because it does not change scheduled time.
       **/
      RescheduleNoChange: AugmentedError<ApiType>;
      /**
       * Given target block number is in the past.
       **/
      TargetBlockNumberInPast: AugmentedError<ApiType>;
    };
    session: {
      /**
       * Registered duplicate key.
       **/
      DuplicatedKey: AugmentedError<ApiType>;
      /**
       * Invalid ownership proof.
       **/
      InvalidProof: AugmentedError<ApiType>;
      /**
       * Key setting account is not live, so it's impossible to associate keys.
       **/
      NoAccount: AugmentedError<ApiType>;
      /**
       * No associated validator ID for account.
       **/
      NoAssociatedValidatorId: AugmentedError<ApiType>;
      /**
       * No keys are associated with this account.
       **/
      NoKeys: AugmentedError<ApiType>;
    };
    society: {
      /**
       * User has already made a bid.
       **/
      AlreadyBid: AugmentedError<ApiType>;
      /**
       * User is already a candidate.
       **/
      AlreadyCandidate: AugmentedError<ApiType>;
      /**
       * Society already founded.
       **/
      AlreadyFounded: AugmentedError<ApiType>;
      /**
       * User is already a member.
       **/
      AlreadyMember: AugmentedError<ApiType>;
      /**
       * Member is already vouching or banned from vouching again.
       **/
      AlreadyVouching: AugmentedError<ApiType>;
      /**
       * An incorrect position was provided.
       **/
      BadPosition: AugmentedError<ApiType>;
      /**
       * Cannot remove the founder.
       **/
      Founder: AugmentedError<ApiType>;
      /**
       * Cannot remove the head of the chain.
       **/
      Head: AugmentedError<ApiType>;
      /**
       * Not enough in pot to accept candidate.
       **/
      InsufficientPot: AugmentedError<ApiType>;
      /**
       * Too many members in the society.
       **/
      MaxMembers: AugmentedError<ApiType>;
      /**
       * Nothing to payout.
       **/
      NoPayout: AugmentedError<ApiType>;
      /**
       * User is not a candidate.
       **/
      NotCandidate: AugmentedError<ApiType>;
      /**
       * The caller is not the founder.
       **/
      NotFounder: AugmentedError<ApiType>;
      /**
       * The caller is not the head.
       **/
      NotHead: AugmentedError<ApiType>;
      /**
       * User is not a member.
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * User is not suspended.
       **/
      NotSuspended: AugmentedError<ApiType>;
      /**
       * Member is not vouching.
       **/
      NotVouching: AugmentedError<ApiType>;
      /**
       * User is suspended.
       **/
      Suspended: AugmentedError<ApiType>;
    };
    sudo: {
      /**
       * Sender must be the Sudo account
       **/
      RequireSudo: AugmentedError<ApiType>;
    };
    system: {
      /**
       * The origin filter prevent the call to be dispatched.
       **/
      CallFiltered: AugmentedError<ApiType>;
      /**
       * Failed to extract the runtime version from the new runtime.
       * 
       * Either calling `Core_version` or decoding `RuntimeVersion` failed.
       **/
      FailedToExtractRuntimeVersion: AugmentedError<ApiType>;
      /**
       * The name of specification does not match between the current runtime
       * and the new runtime.
       **/
      InvalidSpecName: AugmentedError<ApiType>;
      /**
       * Suicide called when the account has non-default composite data.
       **/
      NonDefaultComposite: AugmentedError<ApiType>;
      /**
       * There is a non-zero reference count preventing the account from being purged.
       **/
      NonZeroRefCount: AugmentedError<ApiType>;
      /**
       * The specification version is not allowed to decrease between the current runtime
       * and the new runtime.
       **/
      SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
    };
    technicalCommittee: {
      /**
       * Members are already initialized!
       **/
      AlreadyInitialized: AugmentedError<ApiType>;
      /**
       * Duplicate proposals not allowed
       **/
      DuplicateProposal: AugmentedError<ApiType>;
      /**
       * Duplicate vote ignored
       **/
      DuplicateVote: AugmentedError<ApiType>;
      /**
       * Account is not a member
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Proposal must exist
       **/
      ProposalMissing: AugmentedError<ApiType>;
      /**
       * The close call was made too early, before the end of the voting.
       **/
      TooEarly: AugmentedError<ApiType>;
      /**
       * There can only be a maximum of `MaxProposals` active proposals.
       **/
      TooManyProposals: AugmentedError<ApiType>;
      /**
       * Mismatched index
       **/
      WrongIndex: AugmentedError<ApiType>;
      /**
       * The given length bound for the proposal was too low.
       **/
      WrongProposalLength: AugmentedError<ApiType>;
      /**
       * The given weight bound for the proposal was too low.
       **/
      WrongProposalWeight: AugmentedError<ApiType>;
    };
    technicalMembership: {
      /**
       * Already a member.
       **/
      AlreadyMember: AugmentedError<ApiType>;
      /**
       * Not a member.
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Too many members.
       **/
      TooManyMembers: AugmentedError<ApiType>;
    };
    tips: {
      /**
       * The tip was already found/started.
       **/
      AlreadyKnown: AugmentedError<ApiType>;
      /**
       * The account attempting to retract the tip is not the finder of the tip.
       **/
      NotFinder: AugmentedError<ApiType>;
      /**
       * The tip cannot be claimed/closed because it's still in the countdown period.
       **/
      Premature: AugmentedError<ApiType>;
      /**
       * The reason given is just too big.
       **/
      ReasonTooBig: AugmentedError<ApiType>;
      /**
       * The tip cannot be claimed/closed because there are not enough tippers yet.
       **/
      StillOpen: AugmentedError<ApiType>;
      /**
       * The tip hash is unknown.
       **/
      UnknownTip: AugmentedError<ApiType>;
    };
    tokens: {
      /**
       * Cannot convert Amount into Balance type
       **/
      AmountIntoBalanceFailed: AugmentedError<ApiType>;
      /**
       * The balance is too low
       **/
      BalanceTooLow: AugmentedError<ApiType>;
      /**
       * Beneficiary account must pre-exist
       **/
      DeadAccount: AugmentedError<ApiType>;
      /**
       * Value too low to create account due to existential deposit
       **/
      ExistentialDeposit: AugmentedError<ApiType>;
      /**
       * Transfer/payment would kill account
       **/
      KeepAlive: AugmentedError<ApiType>;
      /**
       * Failed because liquidity restrictions due to locking
       **/
      LiquidityRestrictions: AugmentedError<ApiType>;
      /**
       * Failed because the maximum locks was exceeded
       **/
      MaxLocksExceeded: AugmentedError<ApiType>;
      TooManyReserves: AugmentedError<ApiType>;
    };
    treasury: {
      /**
       * The spend origin is valid but the amount it is allowed to spend is lower than the
       * amount to be spent.
       **/
      InsufficientPermission: AugmentedError<ApiType>;
      /**
       * Proposer's balance is too low.
       **/
      InsufficientProposersBalance: AugmentedError<ApiType>;
      /**
       * No proposal or bounty at that index.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Proposal has not been approved.
       **/
      ProposalNotApproved: AugmentedError<ApiType>;
      /**
       * Too many approvals in the queue.
       **/
      TooManyApprovals: AugmentedError<ApiType>;
    };
    utility: {
      /**
       * Too many calls batched.
       **/
      TooManyCalls: AugmentedError<ApiType>;
    };
    vesting: {
      /**
       * Amount being transferred is too low to create a vesting schedule.
       **/
      AmountLow: AugmentedError<ApiType>;
      /**
       * The account already has `MaxVestingSchedules` count of schedules and thus
       * cannot add another one. Consider merging existing schedules in order to add another.
       **/
      AtMaxVestingSchedules: AugmentedError<ApiType>;
      /**
       * Failed to create a new schedule because some parameter was invalid.
       **/
      InvalidScheduleParams: AugmentedError<ApiType>;
      /**
       * The account given is not vesting.
       **/
      NotVesting: AugmentedError<ApiType>;
      /**
       * An index was out of bounds of the vesting schedules.
       **/
      ScheduleIndexOutOfBounds: AugmentedError<ApiType>;
    };
    whitelist: {
      /**
       * Account is already investor
       **/
      AccountAlreadyInvestor: AugmentedError<ApiType>;
      /**
       * Two investors with same AccountId given
       **/
      AccountDuplicate: AugmentedError<ApiType>;
      /**
       * Investor status is already active
       **/
      AlreadyActive: AugmentedError<ApiType>;
      /**
       * Investor status is already not active
       **/
      AlreadyNotActive: AugmentedError<ApiType>;
      /**
       * Call argument is invalid (possibly empty)
       **/
      InvalidInput: AugmentedError<ApiType>;
      /**
       * InvestorKey is already used
       **/
      InvestorKeyExists: AugmentedError<ApiType>;
      /**
       * Two investors with same InvestorKey given
       **/
      KeyDuplicate: AugmentedError<ApiType>;
      /**
       * Account is not investor
       **/
      NotInvestor: AugmentedError<ApiType>;
      /**
       * Account is not `Admin`
       **/
      NotWhitelistAdmin: AugmentedError<ApiType>;
      /**
       * Account is not `Manager`
       **/
      NotWhitelistManager: AugmentedError<ApiType>;
      /**
       * RuntimeOrigin does not pass roles check
       **/
      PermissionDenied: AugmentedError<ApiType>;
      /**
       * Can't change investor's
       **/
      SameAddress: AugmentedError<ApiType>;
      /**
       * `Admin` with this AccountId already exists
       **/
      WhitelistAdminExists: AugmentedError<ApiType>;
      /**
       * `Manager` with this AccountId already exists
       **/
      WhitelistManagerExists: AugmentedError<ApiType>;
    };
    xcmpQueue: {
      /**
       * Bad overweight index.
       **/
      BadOverweightIndex: AugmentedError<ApiType>;
      /**
       * Bad XCM data.
       **/
      BadXcm: AugmentedError<ApiType>;
      /**
       * Bad XCM origin.
       **/
      BadXcmOrigin: AugmentedError<ApiType>;
      /**
       * Failed to send XCM message.
       **/
      FailedToSend: AugmentedError<ApiType>;
      /**
       * Provided weight is possibly not enough to execute the message.
       **/
      WeightOverLimit: AugmentedError<ApiType>;
    };
  } // AugmentedErrors
} // declare module
