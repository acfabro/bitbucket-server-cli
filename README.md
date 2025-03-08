# bitbucket-server-cli

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A powerful command-line interface (CLI) tool for interacting with Bitbucket Server (on-premise Bitbucket Data Center). This tool simplifies common Bitbucket operations through an intuitive command-line interface, making it ideal for CI/CD pipelines, automation scripts, and daily development workflows.

## Features

- **Build Status Management**: Get and store build statuses for commits
- **Pull Request Changes**: Retrieve and analyze changes in pull requests
- **JSON Output**: All commands return data in JSON format for easy parsing and integration
- **Flexible Configuration**: Configure via command-line arguments or environment variables
- **Comprehensive Error Handling**: Detailed error codes and messages for troubleshooting

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Examples](#examples)
- [Using Environment Variables](#using-environment-variables)
- [Error Codes](#error-codes)
- [Development](#development)
- [License](#license)

## Installation

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/bitbucket-server-cli.git
   cd bitbucket-server-cli
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/bitbucket-server-cli`

## Configuration

The CLI can be configured using either command-line arguments or environment variables:

| Configuration | Command-line Argument | Environment Variable |
|---------------|----------------------|---------------------|
| Bitbucket Server URL | `--server` | `BITBUCKET_SERVER` |
| API Token | `--api-token` | `BITBUCKET_API_TOKEN` |

**Note:** Using environment variables for the API token is recommended for security reasons.

## Usage

### Basic Usage

```bash
bitbucket-server-cli [OPTIONS] <COMMAND> [ARGS]
```

### Global Options

- `--server <URL>`: The base URL for the Bitbucket server. It must end with `/rest`.
- `--api-token <TOKEN>`: The API token to use for authentication.

### Commands

#### Build Status

Get or store build status information for a commit.

##### Get Build Status

```bash
bitbucket-server-cli build-status \
  --commit-id <COMMIT_ID> \
  --repository-slug <REPO_SLUG> \
  --project-key <PROJECT_KEY> \
  get [--key <KEY>]
```

##### Store Build Status

```bash
bitbucket-server-cli build-status \
  --commit-id <COMMIT_ID> \
  --repository-slug <REPO_SLUG> \
  --project-key <PROJECT_KEY> \
  store \
  --key <KEY> \
  --state <STATE> \
  --url <URL> \
  [--build-number <BUILD_NUMBER>] \
  [--date-added <DATE_ADDED>] \
  [--duration <DURATION>] \
  [--description <DESCRIPTION>] \
  [--name <NAME>] \
  [--parent <PARENT>] \
  [--reference <REFERENCE>] \
  [--test-results <SUCCESSFUL,FAILED,SKIPPED>]
```

Where `<STATE>` is one of: `SUCCESSFUL`, `FAILED`, `INPROGRESS`.

#### Pull Request Changes

Get changes for a pull request.

```bash
bitbucket-server-cli pull-request-changes \
  --pull-request-id <PR_ID> \
  --repository-slug <REPO_SLUG> \
  --project-key <PROJECT_KEY> \
  [--since-id <SINCE_ID>] \
  [--change-scope <CHANGE_SCOPE>] \
  [--until-id <UNTIL_ID>] \
  [--start <START>] \
  [--limit <LIMIT>] \
  [--with-comments <WITH_COMMENTS>]
```

## Examples

### Get Build Status

```bash
# Get all build statuses for a commit
bitbucket-server-cli --server https://bitbucket.example.com/rest --api-token YOUR_API_TOKEN \
  build-status \
  --commit-id abc123 \
  --repository-slug my-repo \
  --project-key PROJ \
  get

# Get a specific build status by key
bitbucket-server-cli --server https://bitbucket.example.com/rest --api-token YOUR_API_TOKEN \
  build-status \
  --commit-id abc123 \
  --repository-slug my-repo \
  --project-key PROJ \
  get \
  --key build-1
```

### Store Build Status

```bash
# Store a successful build status
bitbucket-server-cli --server https://bitbucket.example.com/rest --api-token YOUR_API_TOKEN \
  build-status \
  --commit-id abc123 \
  --repository-slug my-repo \
  --project-key PROJ \
  store \
  --key build-1 \
  --state SUCCESSFUL \
  --url https://ci.example.com/build/1 \
  --description "Build passed" \
  --name "CI Build" \
  --test-results 10,2,1
```

### Get Pull Request Changes

```bash
# Get all changes for a pull request
bitbucket-server-cli --server https://bitbucket.example.com/rest --api-token YOUR_API_TOKEN \
  pull-request-changes \
  --pull-request-id 123 \
  --repository-slug my-repo \
  --project-key PROJ

# Get changes with pagination and filtering
bitbucket-server-cli --server https://bitbucket.example.com/rest --api-token YOUR_API_TOKEN \
  pull-request-changes \
  --pull-request-id 123 \
  --repository-slug my-repo \
  --project-key PROJ \
  --since-id def456 \
  --until-id ghi789 \
  --start 0 \
  --limit 100 \
  --with-comments true
```

## Using Environment Variables

```bash
# Set environment variables
export BITBUCKET_SERVER="https://bitbucket.example.com/rest"
export BITBUCKET_API_TOKEN="YOUR_API_TOKEN"

# Run commands without specifying server and token
bitbucket-server-cli build-status --commit-id abc123 --repository-slug my-repo --project-key PROJ get
```

## Error Codes

The CLI returns different exit codes based on the type of error:

| Exit Code | Description |
|-----------|-------------|
| 0 | Success |
| 1 | Invalid arguments |
| 11 | Error sending request |
| 12 | Unauthorized (check your API token) |
| 13 | Unable to read response |
| 21 | HTTP client error (e.g., 404 Not Found) |
| 22 | HTTP server error |
| 23 | Unexpected response |
| 31 | Unable to deserialize response |
| 101 | Unexpected error |

## Development

This project is built with Rust and uses the following dependencies:
- clap: For command-line argument parsing
- tokio: For async runtime
- serde_json: For JSON serialization/deserialization
- bitbucket-server-rs: A Rust client library for Bitbucket Server REST API

### Running Tests

```bash
cargo test
```

## License

[Add your license information here]
