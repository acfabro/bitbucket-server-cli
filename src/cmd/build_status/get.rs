use crate::bitbucket::CommitArgs;
use crate::cmd::{CommandError, CommandResult};
use bitbucket_server_rs::client::{ApiRequest, Client};
use clap_derive::Args;
use serde_json::json;

// todo move to own module
#[derive(Debug, Args)]
pub struct BuildStatusGetArgs {
    /// The key of the build status
    #[arg(long)]
    pub key: Option<String>,
}

/// Get a specific build status
///
/// [Bitbucket Docs](https://developer.atlassian.com/server/bitbucket/rest/v811/api-group-builds-and-deployments/#api-api-latest-projects-projectkey-repos-repositoryslug-commits-commitid-builds-get)
pub async fn handle(
    commit_args: &CommitArgs,
    get_args: &BuildStatusGetArgs,
    client: &Client,
) -> CommandResult {
    let client = client.clone();
    let mut builder =
        client
            .api()
            .build_status_get(&commit_args.project_key, &commit_args.commit_id, &commit_args.repository_slug);

    if let Some(key) = &get_args.key {
        builder.key(key);
    }

    let response = builder
        .build()
        .map_err(|e| CommandError::UnexpectedError(
            format!("Failed to build request: {}", e),
        ))?
        .send()
        .await;

    match response {
        Ok(build_status) => {
            let json = json!(build_status).to_string();
            println!("{}", json);
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}
