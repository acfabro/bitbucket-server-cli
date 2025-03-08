use bitbucket_server_cli::cmd::pull_request_changes::get::{handle, PullRequestChangesArgs};
use bitbucket_server_cli::bitbucket::PullRequestArgs;
use bitbucket_server_cli::cmd::Command;
use bitbucket_server_rs::client;
use clap::{Parser, CommandFactory};
use mockito::Server;
use std::error::Error;

#[test]
fn test_get_pull_request_changes_success() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();
    let _m = server.mock("GET", "/rest/api/latest/projects/TEST/repos/repo/pull-requests/1/changes")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "fromHash": "from123",
            "toHash": "to456",
            "values": [
                {
                    "contentId": "content123",
                    "fromContentId": "from123",
                    "path": {
                        "components": ["src", "main.rs"],
                        "parent": "src",
                        "name": "main.rs",
                        "toString": "src/main.rs"
                    },
                    "executable": false,
                    "percentUnchanged": 98,
                    "type": "MODIFY",
                    "nodeType": "FILE",
                    "srcExecutable": false,
                    "links": {
                        "self": [{"href": "http://example.com/changes/1"}]
                    }
                }
            ],
            "size": 1,
            "isLastPage": true,
            "start": 0,
            "limit": 25
        }"#)
        .create();

    let args = PullRequestChangesArgs {
        pull_request: PullRequestArgs {
            project_key: "TEST".to_string(),
            repository_slug: "repo".to_string(),
            pull_request_id: "1".to_string(),
        },
        since_id: None,
        change_scope: None,
        until_id: None,
        start: None,
        limit: None,
        with_comments: None,
    };

    let client = client::new(&format!("{}/rest", server.url()), "token");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(handle(&args, &client))?;
    Ok(())
}

#[test]
fn test_get_pull_request_changes_with_options() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();
    let _m = server.mock("GET", "/rest/api/latest/projects/TEST/repos/repo/pull-requests/1/changes")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("sinceId".into(), "abc123".into()),
            mockito::Matcher::UrlEncoded("changeScope".into(), "ALL".into()),
            mockito::Matcher::UrlEncoded("untilId".into(), "def456".into()),
            mockito::Matcher::UrlEncoded("start".into(), "0".into()),
            mockito::Matcher::UrlEncoded("limit".into(), "10".into()),
            mockito::Matcher::UrlEncoded("withComments".into(), "true".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"fromHash": "from123", "toHash": "to456", "values": []}"#)
        .create();

    let args = PullRequestChangesArgs {
        pull_request: PullRequestArgs {
            project_key: "TEST".to_string(),
            repository_slug: "repo".to_string(),
            pull_request_id: "1".to_string(),
        },
        since_id: Some("abc123".to_string()),
        change_scope: Some("ALL".to_string()),
        until_id: Some("def456".to_string()),
        start: Some(0),
        limit: Some(10),
        with_comments: Some(true),
    };

    let client = client::new(&format!("{}/rest", server.url()), "token");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(handle(&args, &client))?;
    Ok(())
}

#[test]
fn test_get_pull_request_changes_not_found() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();
    let _m = server.mock("GET", "/rest/api/latest/projects/TEST/repos/repo/pull-requests/999/changes")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"errors":[{"message":"Pull request does not exist"}]}"#)
        .create();

    let args = PullRequestChangesArgs {
        pull_request: PullRequestArgs {
            project_key: "TEST".to_string(),
            repository_slug: "repo".to_string(),
            pull_request_id: "999".to_string(),
        },
        since_id: None,
        change_scope: None,
        until_id: None,
        start: None,
        limit: None,
        with_comments: None,
    };

    let client = client::new(&format!("{}/rest", server.url()), "token");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(handle(&args, &client));
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_pull_request_changes_command_line_args() {
    // Simulate command line arguments for pull request changes
    let args = vec![
        "bitbucket-server-cli",
        "--server", "https://bitbucket.example.com/rest",
        "--api-token", "my-token",
        "pull-request-changes",
        "--pull-request-id", "123",
        "--repository-slug", "test-repo",
        "--project-key", "TEST",
        "--since-id", "abc123",
        "--until-id", "def456",
        "--change-scope", "ALL",
        "--start", "0",
        "--limit", "100",
        "--with-comments", "true"
    ];

    // Parse the arguments
    let cmd = Command::parse_from(args.clone());
    
    // Verify server arguments are accessible
    assert_eq!(cmd.server_args.server, Some("https://bitbucket.example.com/rest".to_string()));
    assert_eq!(cmd.server_args.api_token, Some("my-token".to_string()));
    
    // Use clap's try_get_matches to verify the other arguments
    let command = Command::command();
    let matches = command.try_get_matches_from(args).expect("Failed to parse arguments");
    
    // Verify pull-request-changes subcommand
    let pr_changes_matches = matches.subcommand_matches("pull-request-changes").expect("No pull-request-changes subcommand");
    
    // Verify pull request args
    assert_eq!(pr_changes_matches.get_one::<String>("pull_request_id").map(|s| s.as_str()), Some("123"));
    assert_eq!(pr_changes_matches.get_one::<String>("repository_slug").map(|s| s.as_str()), Some("test-repo"));
    assert_eq!(pr_changes_matches.get_one::<String>("project_key").map(|s| s.as_str()), Some("TEST"));
    
    // Verify optional args
    assert_eq!(pr_changes_matches.get_one::<String>("since_id").map(|s| s.as_str()), Some("abc123"));
    assert_eq!(pr_changes_matches.get_one::<String>("until_id").map(|s| s.as_str()), Some("def456"));
    assert_eq!(pr_changes_matches.get_one::<String>("change_scope").map(|s| s.as_str()), Some("ALL"));
    assert_eq!(pr_changes_matches.get_one::<u32>("start").copied(), Some(0));
    assert_eq!(pr_changes_matches.get_one::<u32>("limit").copied(), Some(100));
    assert_eq!(pr_changes_matches.get_one::<bool>("with_comments").copied(), Some(true));
    
    // Final assertion
    assert!(true, "Command line arguments were parsed correctly");
}

#[tokio::test]
async fn test_pull_request_changes_command_line_args_minimal() {
    // Simulate command line arguments for pull request changes with minimal arguments
    let args = vec![
        "bitbucket-server-cli",
        "--server", "https://bitbucket.example.com/rest",
        "--api-token", "my-token",
        "pull-request-changes",
        "--pull-request-id", "123",
        "--repository-slug", "test-repo",
        "--project-key", "TEST"
    ];

    // Parse the arguments
    let cmd = Command::parse_from(args.clone());
    
    // Verify server arguments are accessible
    assert_eq!(cmd.server_args.server, Some("https://bitbucket.example.com/rest".to_string()));
    assert_eq!(cmd.server_args.api_token, Some("my-token".to_string()));
    
    // Use clap's try_get_matches to verify the other arguments
    let command = Command::command();
    let matches = command.try_get_matches_from(args).expect("Failed to parse arguments");
    
    // Verify pull-request-changes subcommand
    let pr_changes_matches = matches.subcommand_matches("pull-request-changes").expect("No pull-request-changes subcommand");
    
    // Verify pull request args
    assert_eq!(pr_changes_matches.get_one::<String>("pull_request_id").map(|s| s.as_str()), Some("123"));
    assert_eq!(pr_changes_matches.get_one::<String>("repository_slug").map(|s| s.as_str()), Some("test-repo"));
    assert_eq!(pr_changes_matches.get_one::<String>("project_key").map(|s| s.as_str()), Some("TEST"));
    
    // Verify optional args are not present
    assert_eq!(pr_changes_matches.get_one::<String>("since_id"), None);
    assert_eq!(pr_changes_matches.get_one::<String>("until_id"), None);
    assert_eq!(pr_changes_matches.get_one::<String>("change_scope"), None);
    assert_eq!(pr_changes_matches.get_one::<u32>("start"), None);
    assert_eq!(pr_changes_matches.get_one::<u32>("limit"), None);
    assert_eq!(pr_changes_matches.get_one::<bool>("with_comments"), None);
    
    // Final assertion
    assert!(true, "Command line arguments were parsed correctly");
}
