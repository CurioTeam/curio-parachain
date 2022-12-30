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
use primitives::{
	AccountId, AuraId, Balance, CurrencyId, DOLLARS,
	Signature, TokenSymbol
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{
	Perquintill,
	traits::{IdentifyAccount, Verify}
};
use sc_telemetry::TelemetryEndpoints;

#[cfg(feature = "curio-mainnet-runtime")]
pub use curio_mainnet_runtime as default_runtime;

#[cfg(all(not(feature = "curio-mainnet-runtime"), feature = "curio-testnet-runtime"))]
pub use curio_testnet_runtime as default_runtime;

#[cfg(all(not(feature = "curio-mainnet-runtime"), not(feature = "curio-testnet-runtime")))]
pub use curio_devnet_runtime as default_runtime;

/// The `ChainSpec` parameterized for the mainnet runtime.
#[cfg(feature = "curio-mainnet-runtime")]
pub type MainnetChainSpec = sc_service::GenericChainSpec<curio_mainnet_runtime::GenesisConfig, Extensions>;

/// The `ChainSpec` parameterized for the quartz runtime.
#[cfg(feature = "curio-testnet-runtime")]
pub type TestnetChainSpec = sc_service::GenericChainSpec<curio_testnet_runtime::GenesisConfig, Extensions>;

/// The `ChainSpec` parameterized for the opal runtime.
pub type DevnetChainSpec = sc_service::GenericChainSpec<curio_devnet_runtime::GenesisConfig, Extensions>;

#[cfg(feature = "curio-mainnet-runtime")]
pub type DefaultChainSpec = MainnetChainSpec;

#[cfg(all(not(feature = "curio-mainnet-runtime"), feature = "curio-testnet-runtime"))]
pub type DefaultChainSpec = TestnetChainSpec;

#[cfg(all(not(feature = "curio-mainnet-runtime"), not(feature = "curio-testnet-runtime")))]
pub type DefaultChainSpec = DevnetChainSpec;

pub enum RuntimeId {
	#[cfg(feature = "curio-mainnet-runtime")]
	CurioMainnet,

	#[cfg(feature = "curio-testnet-runtime")]
	CurioTestnet,

	CurioDevnet,
	Unknown(String),
}

pub trait RuntimeIdentification {
	fn runtime_id(&self) -> RuntimeId;
}

impl RuntimeIdentification for Box<dyn sc_service::ChainSpec> {
	fn runtime_id(&self) -> RuntimeId {
		#[cfg(feature = "curio-mainnet-runtime")]
		if self.id().starts_with("curio_mainnet") || self.id().starts_with("main") {
			return RuntimeId::CurioMainnet;
		}

		#[cfg(feature = "curio-testnet-runtime")]
		if self.id().starts_with("curio_testnet") || self.id().starts_with("test") {
			return RuntimeId::CurioTestnet;
		}

		if self.id().starts_with("curio_devnet") || self.id() == "dev" || self.id() == "local_testnet" {
			return RuntimeId::CurioDevnet;
		}

		RuntimeId::Unknown(self.id().into())
	}
}

pub enum ServiceId {
	Prod,
	Dev,
}

pub trait ServiceIdentification {
	fn service_id(&self) -> ServiceId;
}

impl ServiceIdentification for Box<dyn sc_service::ChainSpec> {
	fn service_id(&self) -> ServiceId {
		if self.id().ends_with("dev") {
			ServiceId::Dev
		} else {
			ServiceId::Prod
		}
	}
}
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
pub fn node_session_keys(keys: AuraId) -> default_runtime::SessionKeys {
	default_runtime::SessionKeys { aura: keys }
}

const TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

pub fn development_config() -> DefaultChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "CGT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	DefaultChainSpec::from_genesis(
		// Name
		"Curio Development",
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
			relay_chain: "rococo-dev".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

pub fn local_testnet_config() -> DefaultChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "CGT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	DefaultChainSpec::from_genesis(
		// Name
		"Curio Local Testnet",
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
) -> default_runtime::GenesisConfig {
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

	#[cfg(all(not(feature = "curio-mainnet-runtime"), not(feature = "curio-testnet-runtime")))]
	let endowed_currencies: Vec<(CurrencyId, Balance)> = vec![
		(CurrencyId::Token(TokenSymbol::DOT), 100000000000000),
		(CurrencyId::Token(TokenSymbol::QTZ), 10000000000000000000000),
		(CurrencyId::Token(TokenSymbol::ETH), 10000000000000000000000),
	];

	default_runtime::GenesisConfig {
		#[cfg(not(feature = "curio-mainnet-runtime"))]
		whitelist: default_runtime::WhitelistConfig::default(),
		#[cfg(all(not(feature = "curio-mainnet-runtime"), not(feature = "curio-testnet-runtime")))]
		dex: default_runtime::DexConfig {
			initial_listing_trading_pairs: vec![],
			initial_added_liquidity_pools: vec![],
			initial_enabled_trading_pairs: vec![]
		},
		#[cfg(all(not(feature = "curio-mainnet-runtime"), not(feature = "curio-testnet-runtime")))]
		tokens: default_runtime::TokensConfig {
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
		system: default_runtime::SystemConfig {
			code: default_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: default_runtime::BalancesConfig {
			balances: endowed_accounts
			.iter()
			.cloned()
			.map(|k| (k, 10_000_000 * DOLLARS))
			.collect(),
		},
		parachain_info: default_runtime::ParachainInfoConfig { parachain_id: id },
		indices: default_runtime::IndicesConfig { indices: vec![] },
		session: default_runtime::SessionConfig {
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
		democracy: default_runtime::DemocracyConfig::default(),
        elections: default_runtime::ElectionsConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, 500_000 * DOLLARS))
                .collect(),
        },
        council: default_runtime::CouncilConfig::default(),
        technical_committee: default_runtime::TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        },
		technical_membership: Default::default(),
        treasury: Default::default(),
        society: default_runtime::SocietyConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            pot: 0,
            max_members: 999,
        },
        vesting: Default::default(),
        parachain_staking: default_runtime::ParachainStakingConfig {
			stakers: vec![
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					None,
					2 * default_runtime::MinCollatorStake::get(),
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					None,
					2 * default_runtime::MinCollatorStake::get(),
				)
			],
			inflation_config: default_runtime::InflationInfo::new(
				default_runtime::BLOCKS_PER_YEAR,
				// max collator staking rate
				Perquintill::from_percent(60),
				// collator reward rate
				Perquintill::from_percent(10),
				// max delegator staking rate
				Perquintill::from_percent(11),
				// delegator reward rate
				Perquintill::from_percent(8)
			),
			max_candidate_stake: 200_000 * DOLLARS,
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: default_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		#[cfg(not(feature = "curio-mainnet-runtime"))]
		sudo: default_runtime::SudoConfig { key: Some(root_key) },
	}
}
