use bitbucket_server_cli::cmd::{Command};
use bitbucket_server_cli::config::Config;
use bitbucket_server_rs::{client};
use clap::Parser;
use std::env;
use bitbucket_server_cli::cmd;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Command::parse();

    let server = args.server_args.server.clone().unwrap_or("".to_string());
    let api_key = args.server_args.api_token.clone().unwrap_or("".to_string());
    let config = Config::new(
        env::var("BITBUCKET_SERVER").unwrap_or(server),
        env::var("BITBUCKET_API_TOKEN").unwrap_or(api_key),
    );

    cmd::handle(client::new(&config.server, &config.api_token), args).await;
}
