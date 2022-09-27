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

// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use cumulus_primitives_core::ParaId;
use curio_primitives::{AccountId, Balance, CurrencyId, TradingPair, TokenSymbol};
use parachain_node_runtime::{
	AuraId, CouncilConfig, DOLLARS, DemocracyConfig, ElectionsConfig, IndicesConfig,
	Signature, SocietyConfig, TechnicalCommitteeConfig
};
use module_support::token_unit;
use pallet_parachain_staking::{InflationInfo, Range};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{
	Perbill,
	traits::{IdentifyAccount, Verify}
};
use sc_telemetry::TelemetryEndpoints;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<parachain_node_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_public_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn node_session_keys(keys: AuraId) -> parachain_node_runtime::SessionKeys {
	parachain_node_runtime::SessionKeys { aura: keys }
}

pub fn inflation_config(blocks_per_round: u32) -> InflationInfo<Balance> {
	fn to_round_inflation(annual: Range<Perbill>, blocks_per_round: u32) -> Range<Perbill> {
		use pallet_parachain_staking::inflation::{
			perbill_annual_to_perbill_round, BLOCKS_PER_YEAR,
		};
		perbill_annual_to_perbill_round(annual, BLOCKS_PER_YEAR / blocks_per_round)
	}

	let annual = Range {
		min: Perbill::from_percent(4),
		ideal: Perbill::from_percent(5),
		max: Perbill::from_percent(5),
	};

	InflationInfo {
		// staking expectations
		expect: Range { 
			min: 0_020_000,
			ideal: 0_600_000,
			max: 0_100_000
		},
		annual,
		round: to_round_inflation(annual, blocks_per_round),
	}
}

const TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "CGT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				Some(vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				]),
				2000.into(),
			)
		},
		Vec::new(),
		Some(
			TelemetryEndpoints::new(vec![(TELEMETRY_URL.to_string(), 0)])
				.expect("TelemetryURL is invalid")
		),
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "CGT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				Some(vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				]),
				2000.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		Some(
			TelemetryEndpoints::new(vec![(TELEMETRY_URL.to_string(), 0)])
				.expect("TelemetryURL is invalid")
		),
		// Protocol ID
		Some("node-local"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Option<Vec<AccountId>>,
	id: ParaId
) -> parachain_node_runtime::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ]
    });
    let num_endowed_accounts = endowed_accounts.len();

	let liquidity_provider = endowed_accounts[0].clone();

	let endowed_currencies: Vec<(CurrencyId, Balance)> = vec![
		(CurrencyId::Token(TokenSymbol::DOT), 100000000000000),
		(CurrencyId::Token(TokenSymbol::QTZ), 10000000000000000000000),
		(CurrencyId::Token(TokenSymbol::ETH), 10000000000000000000000),
	];

	let initial_enabled_trading_pairs: Vec<TradingPair> = endowed_currencies
		.iter()
		.cloned()
		.map(|(currency_id, _)| TradingPair::from_currency_ids(CurrencyId::Token(TokenSymbol::CGT), currency_id).unwrap())
		.collect();

	let initial_liquidity = initial_enabled_trading_pairs
		.iter()
		.cloned()
		.map(|trading_pair| (trading_pair, (1000 * token_unit(trading_pair.first()), 1000 * token_unit(trading_pair.second()))))
		.collect();

    const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
    const STASH: Balance = 500_000 * DOLLARS;

	parachain_node_runtime::GenesisConfig {
		dex: parachain_node_runtime::DexConfig {
			initial_listing_trading_pairs: vec![],
			initial_added_liquidity_pools: vec![
				(liquidity_provider, initial_liquidity)
			],
			initial_enabled_trading_pairs: initial_enabled_trading_pairs
		},
		tokens: parachain_node_runtime::TokensConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.flat_map::<Vec<(AccountId, CurrencyId, Balance)>, _>(|account| 
					endowed_currencies
						.iter()
						.cloned()
						.map(|(currency_id, amount)| (account.clone(), currency_id, amount))
						.collect()
				)
				.collect(),
		},
		system: parachain_node_runtime::SystemConfig {
			code: parachain_node_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: parachain_node_runtime::BalancesConfig {
			balances: endowed_accounts
			.iter()
			.cloned()
			.map(|k| (k, ENDOWMENT))
			.collect(),
		},
		parachain_info: parachain_node_runtime::ParachainInfoConfig { parachain_id: id },
		indices: IndicesConfig { indices: vec![] },
		session: parachain_node_runtime::SessionConfig {
			keys: invulnerables
				.iter()
				.cloned()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						node_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		democracy: DemocracyConfig::default(),
        elections: ElectionsConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, STASH))
                .collect(),
        },
        council: CouncilConfig::default(),
        technical_committee: TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        },
		//im_online: ImOnlineConfig { keys: vec![] },
        //authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		technical_membership: Default::default(),
        treasury: Default::default(),
        society: SocietyConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            pot: 0,
            max_members: 999,
        },
        vesting: Default::default(),
        parachain_staking: parachain_node_runtime::ParachainStakingConfig {
			candidates: invulnerables
				.iter()
				.cloned()
				.map(|(acc, _)| (acc, parachain_node_runtime::MinCollatorStk::get()))
				.collect(),
			delegations: vec![],
			inflation_config: inflation_config(parachain_node_runtime::DefaultBlocksPerRound::get()),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: parachain_node_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		sudo: parachain_node_runtime::SudoConfig { key: Some(root_key) },
	}
}
