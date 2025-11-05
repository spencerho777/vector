use anyhow::Result;

use crate::testing::{
    config::ComposeTestConfig,
    integration::{ComposeTest, ComposeTestLocalConfig},
};

/// Start an integration test environment
/// Integration tests don't build the image during start - they build lazily during test
pub(crate) fn exec_integration(integration: &str, environment: Option<&String>) -> Result<()> {
    let environment = select_environment(
        ComposeTestLocalConfig::integration(),
        integration,
        environment,
    )?;
    debug!("Selected environment: {environment:#?}");
    ComposeTest::generate(
        ComposeTestLocalConfig::integration(),
        integration,
        environment,
        false, // Integration tests build lazily in test(), not here
        0,
    )?
    .start()
}

/// Start an E2E test environment
/// E2E tests build the image during start because Vector runs as a service in compose
pub(crate) fn exec_e2e(test: &str, environment: Option<&String>, all_features: bool) -> Result<()> {
    let environment = select_environment(ComposeTestLocalConfig::e2e(), test, environment)?;
    debug!("Selected environment: {environment:#?}");
    ComposeTest::generate(
        ComposeTestLocalConfig::e2e(),
        test,
        environment,
        all_features,
        0,
    )?
    .start()
}

fn select_environment(
    local_config: ComposeTestLocalConfig,
    test_name: &str,
    environment: Option<&String>,
) -> Result<String> {
    if let Some(environment) = environment {
        Ok(environment.clone())
    } else {
        let (_test_dir, config) = ComposeTestConfig::load(local_config.directory, test_name)?;
        let envs = config.environments();
        trace!("Available environments: {envs:#?}");
        let env = envs.keys().next().expect("Test has no environments");
        Ok(env.clone())
    }
}
