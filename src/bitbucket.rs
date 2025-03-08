use clap_derive::Args;

#[derive(Debug, Args)]
pub struct PullRequestArgs {
    #[arg(short, long)]
    pub pull_request_id: String,
    #[arg(short, long)]
    pub repository_slug: String,
    #[arg(short = 'k', long)]
    pub project_key: String,
}

#[derive(Debug, Args)]
pub struct CommitArgs {
    /// The commit ID
    #[arg(short, long)]
    pub commit_id: String,
    /// The repository slug. e.g. `my-repository-name`
    #[arg(short, long)]
    pub repository_slug: String,
    /// The project key
    #[arg(short = 'k', long)]
    pub project_key: String,
}
