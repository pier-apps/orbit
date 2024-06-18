//! Implements the dfx extension CLI commands for making requests.

pub mod canister;
pub mod permission;

use crate::args::request::Args;

/// The main entry point for the `dfx orbit` CLI.
pub async fn main(args: Args) -> anyhow::Result<()> {
    match args {
        Args::Canister(canister_args) => canister::main(canister_args).await,
        Args::Permission(permission_args) => permission::main(permission_args).await,
    }
}
