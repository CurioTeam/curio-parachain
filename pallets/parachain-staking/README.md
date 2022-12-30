# Parachain Staking

 A simple staking pallet providing means of selecting a set of collators to
 become block authors based on their total backed stake. The main difference
 between this pallet and `frame/pallet-staking` is that this pallet uses
 direct delegation. Delegators choose exactly who they delegate and with what
 stake. This is different from `frame/pallet-staking` where you approval vote
 and then run Phragmen. Moreover, this pallet rewards a collator and their
 delegators immediately when authoring a block. Rewards are calculated
 separately between collators and delegators.

 To join the set of candidates, an account must call `join_candidates` with
 `MinCollatorCandidateStake` <= stake <= `MaxCollatorCandidateStake`.

To leave the set of candidates, the collator calls `leave_candidates`. If
the call succeeds, the collator is removed from the pool of candidates so
they cannot be selected for future collator sets, but they are not unstaking
until executing the exit request by calling the extrinsic
`execute_leave_candidates` at least `ExitQueueDelay` rounds later. After
doing so, the collator candidate as well as their delegators are unstaked.
Both parties then have to wait another `StakeDuration` more blocks to be
able to unlock their stake.

Candidates which requested to leave can still be in the set of authors for
the next round due to the design of the session pallet which at the start of
session s(i) chooses a set for the next session s(i+1). Thus, candidates
have to keep collating at least until the end of the next session (= round).
We extend this by delaying their execute by at least `ExitQueueDelay` many
sessions.

To join the set of delegators, an account must call `join_delegators` with
stake >= `MinDelegatorStake`. There are also runtime methods for delegating
additional collators and revoking delegations.

## Overview

The KILT parachain staking pallet provides functions for:
- Joining the set of collator candidates of which the best
  `MaxSelectedCandidates` are chosen to become active collators for the next
  session. That makes the set of active collators the set of block authors
  by handing it over to the session and the authority pallet.
- Delegating to a collator candidate by staking for them.
- Increasing and reducing your stake as a collator or delegator.
- Revoking your delegation entirely.
- Requesting to leave the set of collator candidates.
- Withdrawing your unstaked balance after waiting for a certain number of
  blocks.

### Terminology

- **Candidate:** A user which locks up tokens to be included into the set of
  authorities which author blocks and receive rewards for doing so.

- **Collator:** A candidate that was chosen to collate this round.

- **Delegator:** A user which locks up tokens for collators they trust. When
  their collator authors a block, the corresponding delegators also receive
  rewards.

- **Total Stake:** A collator’s own stake + the sum of delegated stake to
  this collator.

- **Total collator stake:** The sum of tokens locked for staking from all
  collator candidates.

- **Total delegator stake:** The sum of tokens locked for staking from all
  delegators.

- **To Stake:** Lock tokens for staking.

- **To Unstake:** Unlock tokens from staking.

- **Round (= Session):** A fixed number of blocks in which the set of
  collators does not change. We set the length of a session to the length of
  a staking round, thus both words are interchangeable in the context of
  this pallet.

- **Lock:** A freeze on a specified amount of an account's free balance
  until a specified block number. Multiple locks always operate over the
  same funds, so they "overlay" rather than "stack"

## Genesis config

The ParachainStaking pallet depends on the [`GenesisConfig`].

## Assumptions

- At the start of session s(i), the set of session ids for session s(i+1)
  are chosen. These equal the set of selected candidates. Thus, we cannot
  allow collators to leave at least until the start of session s(i+2).