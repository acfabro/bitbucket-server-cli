use crate::bitbucket::CommitArgs;
use crate::cmd::CommandResult;
use bitbucket_server_rs::api::build_status::TestResults;
use bitbucket_server_rs::api::build_status_post::BuildStatusPostPayload;
use bitbucket_server_rs::client::{ApiError, ApiRequest, Client};
use clap_derive::Args;

// todo move to own module
#[derive(Debug, Args)]
pub struct BuildStatusStoreArgs {
    /// The key of the build status
    #[arg(long)]
    pub key: String,
    /// The build status state, one of: "SUCCESSFUL", "FAILED", "INPROGRESS"
    #[arg(long)]
    pub state: String,
    /// URL referring to the build result page in the CI tool.
    #[arg(long)]
    pub url: String,
    /// The build number
    #[arg(long)]
    pub build_number: Option<String>,
    /// The date the build status was added
    #[arg(long)]
    pub date_added: Option<String>,
    /// The duration of the build in milliseconds
    #[arg(long)]
    pub duration: Option<u64>,
    /// A description of the build status
    #[arg(long)]
    pub description: Option<String>,
    /// The name of the build status
    #[arg(long)]
    pub name: Option<String>,
    /// The parent of the build status
    #[arg(long)]
    pub parent: Option<String>,
    /// The reference of the build status
    #[arg(long)]
    pub reference: Option<String>,
    /// The number of failed, successful, and skipped tests
    #[arg(long)]
    pub test_results: Option<Vec<u32>>,
}

/// Store a build status
///
/// [Bitbucket Docs](https://developer.atlassian.com/server/bitbucket/rest/v811/api-group-builds-and-deployments/#api-api-latest-projects-projectkey-repos-repositoryslug-commits-commitid-builds-post)
pub async fn handle(
    commit_args: &CommitArgs,
    store_args: &BuildStatusStoreArgs,
    client: &Client,
) -> CommandResult {
    let client = client.clone();

    let response = client
        .api()
        .build_status_post(
            &commit_args.project_key,
            &commit_args.repository_slug,
            &commit_args.commit_id,
            &BuildStatusPostPayload {
                url: store_args.url.to_owned(),
                key: store_args.key.to_owned(),
                state: store_args.state.to_owned().into(),
                build_number: store_args.build_number.to_owned(),
                description: store_args.description.to_owned(),
                duration: store_args.duration,
                name: store_args.name.to_owned(),
                parent: store_args.parent.to_owned(),
                reference: store_args.reference.to_owned(),
                date_added: match store_args.date_added.to_owned() {
                    Some(date_added) => {
                        Some(date_added.parse().map_err(|_| ApiError::RequestError)?)
                    }
                    None => None,
                },
                test_results: match store_args.test_results.to_owned() {
                    Some(test_results) => Some(TestResults {
                        successful: test_results[0],
                        failed: test_results[1],
                        skipped: test_results[2],
                    }),
                    None => None,
                },
            },
        )
        .send()
        .await;

    match response {
        Ok(_) => {
            println!("Build status stored.");
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}
