// use fedimint_cli::FedimintCli;
// use std::path::Path;
// use fedimint_core::config::ClientConfig;
// use fedimint_core::config::load_from_file;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // Setup omitted for brevity

//     // Hypothetical function to obtain ClientConfig
//     let client_config: ClientConfig = load_client_config().await?;

//     // Access the CoreConsensusVersion from GlobalClientConfig within ClientConfig
//     let _core_consensus_version = client_config.global.consensus_version;

//     // Proceed with the application logic
//     let _app = FedimintCli::new(env!("FEDIMINT_BUILD_CODE_VERSION"))?
//         .with_default_modules()
//         .run()
//         .await;

//     Ok(())
// }

// // Example function to load ClientConfig (implementation depends on your app's architecture)
// async fn load_client_config() -> anyhow::Result<ClientConfig> {
//     // Implement loading logic here. For example, loading from a local JSON file:
//     load_from_file(Path::new("path_to_your_client_config.json"))
// }

use fedimint_cli::FedimintCli;
use fedimint_core::module::{CoreConsensusVersion, ModuleConsensusVersion};
use std::env;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Example version information; replace with actual dynamic retrieval
    let core_version = CoreConsensusVersion::new(1, 0);
    let module_version = ModuleConsensusVersion::new(1, 0);

    // Log the consensus version info
    tracing::info!("Core Consensus Version: {}.{}", core_version.major, core_version.minor);
    tracing::info!("Module Consensus Version: {}.{}", module_version.major, module_version.minor);

    // Proceed with the rest of the application startup
    FedimintCli::new(env!("FEDIMINT_BUILD_CODE_VERSION"))?
        .with_default_modules()
        .run()
        .await;

    Ok(())
}
