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

use crate::{
	chain_spec::{
		self, RuntimeId, RuntimeIdentification, default_runtime,
	},
	cli::{Cli, RelayChainCli, Subcommand},
	service::{new_partial, start_parachain_node},
};
use std::net::SocketAddr;

use codec::Encode;
use cumulus_client_cli::generate_genesis_block;
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use log::info;
use sc_cli::{
	ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
	NetworkParams, Result, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::{
	config::{BasePath, PrometheusConfig},
	TaskManager,
};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::traits::{AccountIdConversion, Block as BlockT};

use primitives::Block;

#[cfg(feature = "curio-mainnet-runtime")]
use crate::service::MainnetRuntimeExecutor;

#[cfg(feature = "curio-testnet-runtime")]
use crate::service::TestnetRuntimeExecutor;

use crate::service::{DevnetRuntimeExecutor, DefaultRuntimeExecutor};

fn parachain_node_name() -> String {
	"Curio".into()
}

macro_rules! no_runtime_err {
	($chain_name:expr) => {
		format!(
			"No runtime valid runtime was found for chain {}",
			$chain_name
		)
	};
}

fn load_spec(id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
	Ok(match id {
		"dev" => Box::new(chain_spec::development_config()),
		"" | "local" => Box::new(chain_spec::local_testnet_config()),
		path => {
			let path = std::path::PathBuf::from(path);
			let spec = Box::new(chain_spec::DevnetChainSpec::from_json_file(path.clone())?)
				as Box<dyn sc_service::ChainSpec>;

			match spec.runtime_id() {
				#[cfg(feature = "curio-mainnet-runtime")]
				RuntimeId::CurioMainnet => Box::new(chain_spec::MainnetChainSpec::from_json_file(path)?),

				#[cfg(feature = "curio-testnet-runtime")]
				RuntimeId::CurioTestnet => Box::new(chain_spec::TestnetChainSpec::from_json_file(path)?),

				RuntimeId::CurioDevnet => spec,
				RuntimeId::Unknown(chain) => return Err(no_runtime_err!(chain)),
			}
		}
	})
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		format!("{} Node", parachain_node_name())
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"{} Node\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		{} [parachain-args] -- [relaychain-args]",
			parachain_node_name(),
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/paritytech/cumulus/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2020
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		load_spec(id)
	}

	fn native_runtime_version(spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		match spec.runtime_id() {
			#[cfg(feature = "curio-mainnet-runtime")]
			RuntimeId::CurioMainnet => &curio_mainnet_runtime::VERSION,

			#[cfg(feature = "curio-testnet-runtime")]
			RuntimeId::CurioTestnet => &curio_testnet_runtime::VERSION,

			RuntimeId::CurioDevnet => &curio_devnet_runtime::VERSION,
			RuntimeId::Unknown(chain) => panic!("{}", no_runtime_err!(chain)),
		}
	}
}

impl SubstrateCli for RelayChainCli {
	fn impl_name() -> String {
		format!("{} Node", parachain_node_name())
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"{} Node\n\nThe command-line arguments provided first will be \
			passed to the parachain node, while the arguments provided after -- will be passed \
			to the relaychain node.\n\n\
			parachain-collator [parachain-args] -- [relaychain-args]",
			parachain_node_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/paritytech/cumulus/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2020
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		polkadot_cli::Cli::native_runtime_version(chain_spec)
	}
}

macro_rules! async_run_with_runtime {
	(
		$runtime_api:path, $executor:path,
		$runner:ident, $components:ident, $cli:ident, $cmd:ident, $config:ident,
		$( $code:tt )*
	) => {
		$runner.async_run(|$config| {
			let $components = new_partial::<
				$runtime_api, $executor, _
			>(
				&$config,
				crate::service::parachain_build_import_queue,
			)?;
			let task_manager = $components.task_manager;

			{ $( $code )* }.map(|v| (v, task_manager))
		})
	};
}

macro_rules! construct_async_run {
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
		let runner = $cli.create_runner($cmd)?;

		match runner.config().chain_spec.runtime_id() {
			#[cfg(feature = "curio-mainnet-runtime")]
			RuntimeId::CurioMainnet => async_run_with_runtime!(
				curio_mainnet_runtime::RuntimeApi, MainnetRuntimeExecutor,
				runner, $components, $cli, $cmd, $config, $( $code )*
			),

			#[cfg(feature = "curio-testnet-runtime")]
			RuntimeId::CurioTestnet => async_run_with_runtime!(
				curio_testnet_runtime::RuntimeApi, TestnetRuntimeExecutor,
				runner, $components, $cli, $cmd, $config, $( $code )*
			),

			RuntimeId::CurioDevnet => async_run_with_runtime!(
				curio_devnet_runtime::RuntimeApi, DevnetRuntimeExecutor,
				runner, $components, $cli, $cmd, $config, $( $code )*
			),

			RuntimeId::Unknown(chain) => Err(no_runtime_err!(chain).into())
		}
	}}
}

macro_rules! start_node_using_chain_runtime {
	($start_parachain_node_fn:ident($config:expr $(, $($args:expr),+)?) $($code:tt)*) => {
		match $config.chain_spec.runtime_id() {
			#[cfg(feature = "curio-mainnet-runtime")]
			RuntimeId::CurioMainnet => $start_parachain_node_fn::<
				curio_mainnet_runtime::RuntimeApi,
				MainnetRuntimeExecutor,
			>($config $(, $($args),+)?) $($code)*,

			#[cfg(feature = "curio-testnet-runtime")]
			RuntimeId::CurioTestnet => $start_parachain_node_fn::<
				curio_testnet_runtime::RuntimeApi,
				TestnetRuntimeExecutor,
			>($config $(, $($args),+)?) $($code)*,

			RuntimeId::CurioDevnet => $start_parachain_node_fn::<
				curio_devnet_runtime::RuntimeApi,
				DevnetRuntimeExecutor,
			>($config $(, $($args),+)?) $($code)*,

			RuntimeId::Unknown(chain) => Err(no_runtime_err!(chain).into()),
		}
	};
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.database))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.chain_spec))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::Revert(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.backend, None))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()]
					.iter()
					.chain(cli.relay_chain_args.iter()),
				);

				let polkadot_config = SubstrateCli::create_configuration(
					&polkadot_cli,
					&polkadot_cli,
					config.tokio_handle.clone(),
				)
				.map_err(|err| format!("Relay chain argument error: {}", err))?;

				cmd.run(config, polkadot_config)
			})
		},
		Some(Subcommand::ExportGenesisState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				let state_version = Cli::native_runtime_version(&spec).state_version();
				cmd.run::<Block>(&*spec, state_version)
			})
		},
		Some(Subcommand::ExportGenesisWasm(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				cmd.run(&*spec)
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			// Switch on the concrete benchmark sub-command-
			match cmd {
				BenchmarkCmd::Pallet(cmd) =>
					if cfg!(feature = "runtime-benchmarks") {
						runner.sync_run(|config| cmd.run::<Block, DefaultRuntimeExecutor>(config))
					} else {
						Err("Benchmarking wasn't enabled when building the node. \
					You can enable it with `--features runtime-benchmarks`."
							.into())
					},
				BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
					let partials = new_partial::<default_runtime::RuntimeApi, DefaultRuntimeExecutor, _>(
						&config,
						crate::service::parachain_build_import_queue,
					)?;
					cmd.run(partials.client)
				}),
				#[cfg(not(feature = "runtime-benchmarks"))]
				BenchmarkCmd::Storage(_) =>
					return Err(sc_cli::Error::Input(
						"Compile with --features=runtime-benchmarks \
						to enable storage benchmarks."
							.into(),
					)
					.into()),
				#[cfg(feature = "runtime-benchmarks")]
				BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
					let partials = new_partial::<default_runtime::RuntimeApi, DefaultRuntimeExecutor, _>(
						&config,
						crate::service::parachain_build_import_queue,
					)?;
					let db = partials.backend.expose_db();
					let storage = partials.backend.expose_storage();

					cmd.run(config, partials.client.clone(), db, storage)
				}),
				BenchmarkCmd::Machine(cmd) =>
					runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())),
				// NOTE: this allows the Client to leniently implement
				// new benchmark commands without requiring a companion MR.
				#[allow(unreachable_patterns)]
				_ => Err("Benchmarking sub-command unsupported".into()),
			}
		},
		Some(Subcommand::TryRuntime(cmd)) => {
			if cfg!(feature = "try-runtime") {
				let runner = cli.create_runner(cmd)?;

				// grab the task manager.
				let registry = &runner.config().prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager =
					TaskManager::new(runner.config().tokio_handle.clone(), *registry)
						.map_err(|e| format!("Error: {:?}", e))?;

				runner.async_run(|config| {
					Ok((cmd.run::<Block, DefaultRuntimeExecutor>(config), task_manager))
				})
			} else {
				Err("Try-runtime must be enabled by `--features try-runtime`.".into())
			}
		},
		None => {
			let runner = cli.create_runner(&cli.run.normalize())?;
			let collator_options = cli.run.collator_options();

			runner.run_node_until_exit(|config| async move {
				let hwbench = if !cli.no_hardware_benchmarks {
					config.database.path().map(|database_path| {
						let _ = std::fs::create_dir_all(&database_path);
						sc_sysinfo::gather_hwbench(Some(database_path))
					})
				} else {
					None
				};

				let extensions = chain_spec::Extensions::try_get(&*config.chain_spec);

				let para_id = extensions
					.map(|e| e.para_id)
					.ok_or("Could not find parachain ID in chain-spec.")?;

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);

				let id = ParaId::from(para_id);

				let parachain_account =
					AccountIdConversion::<polkadot_primitives::v2::AccountId>::into_account_truncating(&id);

				let state_version = RelayChainCli::native_runtime_version(&config.chain_spec).state_version();
				let block: Block = generate_genesis_block(&*config.chain_spec, state_version)
					.map_err(|e| format!("{:?}", e))?;
				let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));

				let tokio_handle = config.tokio_handle.clone();
				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
						.map_err(|err| format!("Relay chain argument error: {}", err))?;

				info!("Parachain id: {:?}", id);
				info!("Parachain Account: {}", parachain_account);
				info!("Parachain genesis state: {}", genesis_state);
				info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

				start_node_using_chain_runtime! {
					start_parachain_node(config, polkadot_config, collator_options, id, hwbench)
						.await
						.map(|r| r.0)
						.map_err(Into::into)
				}
			})
		},
	}
}

impl DefaultConfigurationValues for RelayChainCli {
	fn p2p_listen_port() -> u16 {
		30334
	}

	fn rpc_ws_listen_port() -> u16 {
		9945
	}

	fn rpc_http_listen_port() -> u16 {
		9934
	}

	fn prometheus_listen_port() -> u16 {
		9616
	}
}

impl CliConfiguration<Self> for RelayChainCli {
	fn shared_params(&self) -> &SharedParams {
		self.base.base.shared_params()
	}

	fn import_params(&self) -> Option<&ImportParams> {
		self.base.base.import_params()
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		self.base.base.network_params()
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		self.base.base.keystore_params()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(self
			.shared_params()
			.base_path()?
			.or_else(|| self.base_path.clone().map(Into::into)))
	}

	fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_http(default_listen_port)
	}

	fn rpc_ipc(&self) -> Result<Option<String>> {
		self.base.base.rpc_ipc()
	}

	fn rpc_ws(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_ws(default_listen_port)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base.base.prometheus_config(default_listen_port, chain_spec)
	}

	fn init<F>(
		&self,
		_support_url: &String,
		_impl_version: &String,
		_logger_hook: F,
		_config: &sc_service::Configuration,
	) -> Result<()>
	where
		F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
	{
		unreachable!("PolkadotCli is never initialized; qed");
	}

	fn chain_id(&self, is_dev: bool) -> Result<String> {
		let chain_id = self.base.base.chain_id(is_dev)?;

		Ok(if chain_id.is_empty() { self.chain_id.clone().unwrap_or_default() } else { chain_id })
	}

	fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
		self.base.base.role(is_dev)
	}

	fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
		self.base.base.transaction_pool(is_dev)
	}

	fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
		self.base.base.trie_cache_maximum_size()
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.base.rpc_methods()
	}

	fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
		self.base.base.rpc_ws_max_connections()
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		self.base.base.rpc_cors(is_dev)
	}

	fn default_heap_pages(&self) -> Result<Option<u64>> {
		self.base.base.default_heap_pages()
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.base.force_authoring()
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.base.disable_grandpa()
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.base.max_runtime_instances()
	}

	fn announce_block(&self) -> Result<bool> {
		self.base.base.announce_block()
	}

	fn telemetry_endpoints(
		&self,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
		self.base.base.telemetry_endpoints(chain_spec)
	}

	fn node_name(&self) -> Result<String> {
		self.base.base.node_name()
	}
}
