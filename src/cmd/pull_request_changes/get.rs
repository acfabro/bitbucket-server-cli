use crate::bitbucket::PullRequestArgs;
use crate::cmd::{CommandError, CommandResult};
use bitbucket_server_rs::client::{ApiRequest, Client};
use clap_derive::Args;
use serde_json;

#[derive(Debug, Args)]
pub struct PullRequestChangesArgs {
    /// Pull request identification arguments
    #[command(flatten)]
    pub pull_request: PullRequestArgs,
    
    /// The commit ID to use as the base of the comparison
    #[arg(long)]
    pub since_id: Option<String>,
    
    /// The scope of changes to include in the response
    #[arg(long)]
    pub change_scope: Option<String>,
    
    /// The commit ID to use as the tip of the comparison
    #[arg(long)]
    pub until_id: Option<String>,
    
    /// The 0-based start index of the page of results to return
    #[arg(long)]
    pub start: Option<u32>,
    
    /// The maximum number of changes to return per page
    #[arg(long)]
    pub limit: Option<u32>,
    
    /// Whether to include comments in the response
    #[arg(long)]
    pub with_comments: Option<bool>,
}

/// Get changes for a pull request
///
/// [Bitbucket Docs](https://developer.atlassian.com/server/bitbucket/rest/v811/api-group-pull-requests/#api-api-latest-projects-projectkey-repos-repositoryslug-pull-requests-pullrequestid-changes-get)
pub async fn handle(args: &PullRequestChangesArgs, client: &Client) -> CommandResult {
    let client = client.clone();
    let mut builder = client.api().pull_request_changes_get(
        &args.pull_request.project_key,
        &args.pull_request.repository_slug,
        &args.pull_request.pull_request_id.to_string(),
    );

    // Add optional parameters
    if let Some(since_id) = &args.since_id {
        builder.since_id(since_id);
    }
    
    if let Some(change_scope) = &args.change_scope {
        builder.change_scope(change_scope);
    }
    
    if let Some(until_id) = &args.until_id {
        builder.until_id(until_id);
    }
    
    if let Some(start) = args.start {
        builder.start(start);
    }
    
    if let Some(limit) = args.limit {
        builder.limit(limit);
    }
    
    if let Some(with_comments) = args.with_comments {
        builder.with_comments(with_comments);
    }

    // Build and send the request
    let response = builder
        .build()
        .map_err(|e| CommandError::UnexpectedError(
            format!("Failed to build request: {}", e),
        ))?
        .send()
        .await;

    // Handle the response
    match response {
        Ok(changes) => {
            let json = serde_json::json!(changes).to_string();
            println!("{}", json);
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}
