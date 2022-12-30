# Substrate and Cumulus pallets
In our parachain we use the following substrate pallets:
1. `frame_system` - you can see information about this pallet [here](https://docs.rs/frame-system/9.0.0/frame_system/);
2. `pallet_aura` - you can see information about this pallet [here](https://docs.rs/pallet-aura/9.0.0/pallet_aura/);
3. `pallet_authorship` - you can see information about this pallet [here](https://docs.rs/pallet-authorship/9.0.0/pallet_authorship/);
4. `pallet_balances` - you can see information about this pallet [here](https://docs.rs/pallet-balances/9.0.0/pallet_balances/);
5. `pallet_bounties` - you can see information about this pallet [here](https://docs.rs/pallet-bounties/9.0.0/pallet_bounties/);
6. `pallet_collective` - you can see information about this pallet [here](https://docs.rs/pallet-collective/9.0.0/pallet_collective/);
7. `pallet_democracy` - you can see information about this pallet [here](https://docs.rs/pallet-democracy/9.0.0/pallet_democracy/);
8. `pallet_elections_phragmen` - you can see information about this pallet [here](https://docs.rs/pallet-elections-phragmen/10.0.0/pallet_elections_phragmen/);
9. `pallet_identity` - you can see information about this pallet [here](https://docs.rs/pallet-identity/9.0.0/pallet_identity/);
10. `pallet_indices` - you can see information about this pallet [here](https://docs.rs/pallet-indices/9.0.0/pallet_indices/);
11. `pallet_membership` - you can see information about this pallet [here](https://docs.rs/pallet-membership/9.0.0/pallet_membership/);
12. `pallet_multisig` - you can see information about this pallet [here](https://docs.rs/pallet-multisig/9.0.0/pallet_multisig/);
13. `pallet_preimage` - you can see information about this pallet [here](https://docs.rs/pallet-preimage/9.0.0/pallet_preimage/);
14. `pallet_proxy` - you can see information about this pallet [here](https://docs.rs/pallet-proxy/9.0.0/pallet_proxy/);
15. `pallet_randomness_collective_flip` - you can see information about this pallet [here](https://docs.rs/pallet-randomness-collective-flip/9.0.0/pallet_randomness_collective_flip/);
16. `pallet_scheduler` - you can see information about this pallet [here](https://docs.rs/pallet-scheduler/10.0.0/pallet_scheduler/);
17. `pallet_session` - you can see information about this pallet [here](https://docs.rs/pallet-session/10.0.0/pallet_session/);
18. `pallet_society` - you can see information about this pallet [here](https://docs.rs/pallet-society/9.0.0/pallet_society/);
19. `pallet_sudo` - you can see information about this pallet [here](https://docs.rs/pallet-sudo/9.0.0/pallet_sudo/);
20. `pallet_timestamp` - you can see information about this pallet [here](https://docs.rs/pallet-timestamp/9.0.0/pallet_timestamp/);
21. `pallet_tips` - you can see information about this pallet [here](https://docs.rs/pallet-tips/9.0.0/pallet_tips/);
22. `pallet_transaction_payment` - you can see information about this pallet [here](https://docs.rs/pallet-transaction-payment/9.0.0/pallet_transaction_payment/);
23. `pallet_treasury` - you can see information about this pallet [here](https://docs.rs/pallet-treasury/9.0.0/pallet_treasury/);
24. `pallet_utility` - you can see information about this pallet [here](https://docs.rs/pallet-utility/9.0.0/pallet_utility/);
25. `pallet_vesting` - you can see information about this pallet [here](https://docs.rs/pallet-vesting/9.0.0/pallet_vesting/).

And Cumulus pallets:
1. `cumulus_pallet_aura_ext` - Cumulus extension pallet for AuRa. This pallets extends the Substrate AuRa pallet to make it compatible with parachains. It provides the [`Pallet`], the [`Config`] and the [`GenesisConfig`]. It is also required that the parachain runtime uses the provided [`BlockExecutor`] to properly check the constructed block on the relay chain;
2. `cumulus_pallet_dmp_queue` - pallet implementing a message queue for downward messages from the relay-chain. Executes downward messages if there is enough weight available and schedules the rest for later execution (by `on_idle` or another `handle_dmp_messages` call). Individual overweight messages are scheduled into a separate queue that is only serviced by explicit extrinsic calls;
3. `cumulus_pallet_parachain_system` - This pallet handles low-level details of being a parachain. It's responsibilities include:
- ingestion of the parachain validation data;
- ingestion of incoming downward and lateral messages and dispatching them;
- coordinating upgrades with the relay-chain;
- communication of parachain outputs, such as sent messages, signalling an upgrade, etc.
Users must ensure that they register this pallet as an inherent provider;
4. `cumulus_pallet_xcm` - pallet for stuff specific to parachains' usage of XCM. Right now that's just the origin used by parachains when receiving `Transact` messages from other parachains or the Relay chain which must be natively represented;
5. `cumulus_pallet_xcmp_queue` - A pallet which uses the XCMP transport layer to handle both incoming and outgoing XCM message sending and dispatch, queuing, signalling and backpressure. To do so, it implements:
* `XcmpMessageHandler`
* `XcmpMessageSource`
Also provides an implementation of `SendXcm` which can be placed in a router tuple for relaying XCM over XCMP if the destination is `Parent/Parachain`. It requires an implementation of `XcmExecutor` for dispatching incoming XCM messages;
6. `pallet_xcm` - pallet to handle XCM messages;
7. `parachain_info` - minimal Pallet that injects a ParachainId into Runtime storage from.