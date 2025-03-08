use bitbucket_server_cli::cmd::build_status::get::{handle, BuildStatusGetArgs};
use bitbucket_server_cli::bitbucket::CommitArgs;
use bitbucket_server_cli::cmd::Command;
use bitbucket_server_rs::client;
use clap::{Parser, CommandFactory};
use mockito::Server;
use std::error::Error;

#[tokio::test]
async fn test_get_build_status_success() {
    let mut server = Server::new_async().await;
    let _m = server.mock("GET", "/rest/api/latest/projects/TEST/repos/repo/commits/abc123/builds")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "state": "SUCCESSFUL",
            "key": "build-1",
            "name": "Build #1",
            "url": "http://example.com/builds/1",
            "description": "Build passed",
            "createdDate": 1738198923,
            "updatedDate": 1738198924
        }"#)
        .create();

    let commit_args = CommitArgs {
        project_key: "TEST".to_string(),
        repository_slug: "repo".to_string(),
        commit_id: "abc123".to_string(),
    };

    let get_args = BuildStatusGetArgs {
        key: None,
    };

    let client = client::new(&format!("{}/rest", server.url()), "token");
    let result = handle(&commit_args, &get_args, &client).await;
    assert!(result.is_ok());
}

#[test]
fn test_get_build_status_with_options() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();
    let _m = server.mock("GET", "/rest/api/latest/projects/TEST/repos/repo/commits/abc123/builds")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("key".into(), "build-1".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "state": "SUCCESSFUL",
            "key": "build-1",
            "name": "Build #1",
            "url": "http://example.com/builds/1",
            "createdDate": 1738198923,
            "updatedDate": 1738198924
        }"#)
        .create();

    let commit_args = CommitArgs {
        project_key: "TEST".to_string(),
        repository_slug: "repo".to_string(),
        commit_id: "abc123".to_string(),
    };

    let get_args = BuildStatusGetArgs {
        key: Some("build-1".to_string()),
    };

    let client = client::new(&format!("{}/rest", server.url()), "token");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(handle(&commit_args, &get_args, &client))?;
    Ok(())
}

#[test]
fn test_get_build_status_not_found() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();
    let _m = server.mock("GET", "/rest/api/latest/projects/TEST/repos/repo/commits/nonexistent/builds")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"errors":[{"message":"Commit does not exist"}]}"#)
        .create();

    let commit_args = CommitArgs {
        project_key: "TEST".to_string(),
        repository_slug: "repo".to_string(),
        commit_id: "nonexistent".to_string(),
    };

    let get_args = BuildStatusGetArgs {
        key: None,
    };

    let client = client::new(&format!("{}/rest", server.url()), "token");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(handle(&commit_args, &get_args, &client));
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_build_status_get_command_line_args() {
    // Simulate command line arguments for build status get
    let args = vec![
        "bitbucket-server-cli",
        "--server", "https://bitbucket.example.com/rest",
        "--api-token", "my-token",
        "build-status",
        "--commit-id", "abc123",
        "--repository-slug", "test-repo",
        "--project-key", "TEST",
        "get",
        "--key", "build-1"
    ];

    // Parse the arguments
    let cmd = Command::parse_from(args.clone());
    
    // Verify server arguments are accessible
    assert_eq!(cmd.server_args.server, Some("https://bitbucket.example.com/rest".to_string()));
    assert_eq!(cmd.server_args.api_token, Some("my-token".to_string()));
    
    // Use clap's try_get_matches to verify the other arguments
    let command = Command::command();
    let matches = command.try_get_matches_from(args).expect("Failed to parse arguments");
    
    // Verify build-status subcommand
    let build_status_matches = matches.subcommand_matches("build-status").expect("No build-status subcommand");
    
    // Verify commit args
    assert_eq!(build_status_matches.get_one::<String>("commit_id").map(|s| s.as_str()), Some("abc123"));
    assert_eq!(build_status_matches.get_one::<String>("repository_slug").map(|s| s.as_str()), Some("test-repo"));
    assert_eq!(build_status_matches.get_one::<String>("project_key").map(|s| s.as_str()), Some("TEST"));
    
    // Verify get subcommand
    let get_matches = build_status_matches.subcommand_matches("get").expect("No get subcommand");
    assert_eq!(get_matches.get_one::<String>("key").map(|s| s.as_str()), Some("build-1"));
    
    // Final assertion
    assert!(true, "Command line arguments were parsed correctly");
}

#[tokio::test]
async fn test_build_status_get_command_line_args_without_key() {
    // Simulate command line arguments for build status get without key
    let args = vec![
        "bitbucket-server-cli",
        "--server", "https://bitbucket.example.com/rest",
        "--api-token", "my-token",
        "build-status",
        "--commit-id", "abc123",
        "--repository-slug", "test-repo",
        "--project-key", "TEST",
        "get"
    ];

    // Parse the arguments
    let cmd = Command::parse_from(args.clone());
    
    // Verify server arguments are accessible
    assert_eq!(cmd.server_args.server, Some("https://bitbucket.example.com/rest".to_string()));
    assert_eq!(cmd.server_args.api_token, Some("my-token".to_string()));
    
    // Use clap's try_get_matches to verify the other arguments
    let command = Command::command();
    let matches = command.try_get_matches_from(args).expect("Failed to parse arguments");
    
    // Verify build-status subcommand
    let build_status_matches = matches.subcommand_matches("build-status").expect("No build-status subcommand");
    
    // Verify commit args
    assert_eq!(build_status_matches.get_one::<String>("commit_id").map(|s| s.as_str()), Some("abc123"));
    assert_eq!(build_status_matches.get_one::<String>("repository_slug").map(|s| s.as_str()), Some("test-repo"));
    assert_eq!(build_status_matches.get_one::<String>("project_key").map(|s| s.as_str()), Some("TEST"));
    
    // Verify get subcommand
    let get_matches = build_status_matches.subcommand_matches("get").expect("No get subcommand");
    assert_eq!(get_matches.get_one::<String>("key"), None);
    
    // Final assertion
    assert!(true, "Command line arguments were parsed correctly");
}
