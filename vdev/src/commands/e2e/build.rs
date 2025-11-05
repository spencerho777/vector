use crate::testing::build::build_e2e_image;
use anyhow::Result;
use clap::Args;

/// Build the E2E test runner image with all E2E features
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(self) -> Result<()> {
        build_e2e_image()
    }
}
