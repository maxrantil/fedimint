use fedimint_cli::FedimintCli;
use fedimint_core::fedimint_build_code_version_env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut fedimint_cli = FedimintCli::new(fedimint_build_code_version_env!())?;

    fedimint_cli.discover_versions_and_log().await?;

    fedimint_cli
        .with_default_modules()
        .run()
        .await;

    Ok(())
}
