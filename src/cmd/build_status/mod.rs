pub mod get;
pub mod store;

pub use get::handle as get;
pub use store::handle as store;

use crate::bitbucket::CommitArgs;
use crate::cmd::CommandError;
use bitbucket_server_rs::client::Client;
use clap_derive::{Args, Subcommand};

/// Comon args for build status operations
#[derive(Debug, Args)]
pub struct BuildStatusArgs {
    /// Refers to the commit
    #[command(flatten)]
    commit_args: CommitArgs,
    /// Post a build status
    #[command(subcommand)]
    command: BuildStatusSubcommands,
}

#[derive(Debug, Subcommand)]
enum BuildStatusSubcommands {
    Get(get::BuildStatusGetArgs),
    Store(store::BuildStatusStoreArgs),
}

/// Build status command handler
pub async fn handle(args: &BuildStatusArgs, client: &Client) -> Result<(), CommandError> {
    let commit_args = &args.commit_args;
    let command = &args.command;

    match command {
        BuildStatusSubcommands::Get(get_args) => get(commit_args, get_args, client).await,
        BuildStatusSubcommands::Store(store_args) => store(commit_args, store_args, client).await,
    }
}
