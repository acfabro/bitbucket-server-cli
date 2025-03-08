pub mod build_status;
pub mod pull_request_changes;

use build_status::BuildStatusArgs;
use bitbucket_server_rs::client::{ApiError, Client};
use clap::{Parser, Subcommand};
use pull_request_changes::get::PullRequestChangesArgs;
use std::process::exit;

#[derive(Debug, Parser)]
#[command(name = "bitbucket-server-cli")]
#[command(version)]
#[command(about = "A simple CLI tool to interact with Bitbucket Data Center", long_about = None)]
pub struct Command {
    /// The base URL for the Bitbucket server. It must end with `/rest`. Alternatively, set the BITBUCKET_SERVER environment variable.
    #[command(flatten)]
    pub server_args: ServerArgs,

    #[command(subcommand)]
    pub(crate) command: Subcommands,
}

#[derive(Debug, Parser)]
pub struct ServerArgs {
    /// The base URL for the Bitbucket server. It must end with `/rest`. Alternatively, set the BITBUCKET_SERVER environment variable.
    #[arg(long, global = true)]
    pub server: Option<String>,

    /// The API token to use for authentication. NOTE: This is not secure. Use the BITBUCKET_API_TOKEN environment variable instead.
    #[arg(long, global = true)]
    pub api_token: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Manage build statuses
    BuildStatus(BuildStatusArgs),

    /// Manage pull request changes
    PullRequestChanges(PullRequestChangesArgs),
}

pub async fn handle(client: Client, args: Command) {
    let result = match args.command {
        Subcommands::BuildStatus(args) => {
            build_status::handle(&args, &client).await
        }
        Subcommands::PullRequestChanges(args) => {
            pull_request_changes::handle(&args, &client).await
        }
    };

    match result {
        // do nothing, success case already handled by the command
        Ok(_) => {}

        // print out the errors
        Err(e) => handle_error(e),
    }
}

pub type CommandResult = Result<(), CommandError>;

#[derive(Debug)]
pub enum CommandError {
    /// Error from API client library
    ApiError(ApiError),
    /// Invalid argument error
    ArgumentError(Vec<String>),
    /// other unexpected error
    UnexpectedError(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::ApiError(e) => write!(f, "API error: {:?}", e),
            CommandError::ArgumentError(errors) => write!(f, "Argument error: {:?}", errors),
            CommandError::UnexpectedError(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}

impl std::error::Error for CommandError {}

/// Convert ApiError to CommandError
impl From<ApiError> for CommandError {
    fn from(error: ApiError) -> Self {
        CommandError::ApiError(error)
    }
}

/// handle CommandError
fn handle_error(error: CommandError) {
    match error {
        CommandError::ArgumentError(errors) => {
            eprintln!("Invalid arguments: {:?}", errors);
            exit(1)
        }
        CommandError::ApiError(ApiError::RequestError) => {
            eprintln!("Error sending request");
            exit(11)
        }
        CommandError::ApiError(ApiError::Unauthorized) => {
            eprintln!("Unauthorized. Please check your API token.");
            exit(12)
        }
        CommandError::ApiError(ApiError::ResponseError) => {
            eprintln!("Unable to read response");
            exit(13)
        }
        // TODO extract messages from the error json
        CommandError::ApiError(ApiError::HttpClientError(404u16, message)) => {
            eprintln!("Target resource not found: {}", message);
            exit(21)
        }
        // TODO extract messages from the error json
        CommandError::ApiError(ApiError::HttpClientError(code, message)) => {
            eprintln!("HTTP client error: {} - {}", code, message);
            exit(21)
        }
        // TODO extract messages from the error json
        CommandError::ApiError(ApiError::HttpServerError(code, message)) => {
            eprintln!("HTTP server error: {} - {}", code, message);
            exit(22)
        }
        CommandError::ApiError(ApiError::UnexpectedResponse(code, message)) => {
            eprintln!("Unexpected response: {} - {}", code, message);
            exit(23)
        }
        CommandError::ApiError(ApiError::DeserializationError(message)) => {
            eprintln!("Unable to deserialize response: {}", message);
            exit(31)
        }
        CommandError::UnexpectedError(e) => {
            eprintln!("Unexpected error: {}", e);
            exit(101)
        }
    }
}
