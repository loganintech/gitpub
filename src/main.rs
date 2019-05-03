use std::process::exit;

use reqwest::StatusCode;
use structopt::StructOpt;

mod cli;
mod git;
mod provider;

use cli::Gitpo;
use git::Git;
use provider::Provider;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = cli::Gitpo::from_args();
    let client = reqwest::Client::new();
    let request = client
        .post(&config.endpoint())
        .body(config.payload())
        .header("Content-Type", "application/json");

    let request = match &config {
        Gitpo::Github(config) => request.header("Authorization", format!("token {}", config.token)),
        Gitpo::Gitlab(config) => request.header("Private-Token", config.token.to_string()),
        Gitpo::BitBucket(config) => request.header(
            "Authorization",
            format!(
                "Basic {}",
                base64::encode(&format!("{}:{}", &config.username, &config.token))
            ),
        ),
    };

    let result = request.send()?;
    let status = result.status();
    let headers = result.headers();
    match status {
        StatusCode::OK | StatusCode::CREATED => {
            let apiloc = config.extract_url(&headers);
            println!("Repo created: {}", apiloc);
            if let Err(e) = Git::add_remote_origin(apiloc.as_str()) {
                eprintln!("{}", e);
            }
        }
        StatusCode::UNPROCESSABLE_ENTITY | StatusCode::BAD_REQUEST => {
            eprintln!("The provider had an issue processing this request. Perhaps the repository already exists, or you're using an unsupported option. e.g. Enabling projects on a repo in an org that has them disabled.");
            exit(2);
        }
        StatusCode::UNAUTHORIZED => {
            eprintln!("You are unauthorized to create that repo.");
            exit(3);
        }
        _ => {
            eprintln!("An unknown response was sent by the provider.");
            exit(42);
        }
    };

    Ok(())
}
