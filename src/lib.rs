// #![allow(clippy::nonminimal_bool)]
use std::process::exit;

mod cli;
mod git;
mod provider;

use cli::Gitpo;
use reqwest::StatusCode;

use provider::Provider;

use git::add_remote;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::get_app().get_matches();
    let config = Gitpo::from_matches(&matches);
    let config = config.as_provider();

    let request = configure_request(config, &matches);
    let result = request.send()?;

    handle_result(result, config, &matches);

    Ok(())
}

fn configure_request(config: &dyn Provider, matches: &clap::ArgMatches) -> reqwest::RequestBuilder {
    let client = reqwest::Client::new();
    let endpoint = if let Some(e) = matches.value_of("endpoint") {
        e.to_string()
    } else {
        config.endpoint()
    };
    let request = client
        .post(&endpoint)
        .body(config.payload())
        .header("Content-Type", "application/json")
        .header(config.auth_header().as_bytes(), config.token());

    request
}

fn handle_result(result: reqwest::Response, config: &dyn Provider, matches: &clap::ArgMatches) {
    let status = result.status();
    let headers = result.headers();
    match status {
        StatusCode::OK | StatusCode::CREATED => {
            let apiloc = config.extract_url(&headers);
            let (remote_url, can_use_ssh) = match (
                config.ssh_url(&headers),
                matches.is_present("ssh_remote_format"),
            ) {
                (Some(url), true) => (url, true),
                _ => (config.extract_url(&headers), false),
            };
            println!("Repo created: {}", apiloc);
            let remote_name = matches
                .value_of("remote_name")
                .expect("This should default to origin, so something is wrong.");

            if matches.is_present("set_remote") {
                if matches.is_present("ssh_remote_format") && !can_use_ssh {
                    eprintln!("Can't use ssh format with this provider.");
                    exit(22);
                }

                if !add_remote(remote_name, &remote_url) {
                    eprintln!("Failed to add remote.");
                    exit(404);
                }
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
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn github_integration() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let matches = cli::get_app().get_matches_from(vec![
            "gitpub",
            "github",
            "-t",
            include_str!("../keytest.txt").trim_end(),
            "-i",
            "-w",
            "-r",
            "-d",
            &format!("Description {}", secs),
            "--gitignore",
            "Rust",
            "--homepage",
            &format!("Homepage {}", secs),
            "--license",
            "MIT",
            "-n",
            &format!("Test Repo {}", secs),
        ]);

        let config = Gitpo::from_matches(&matches);
        let config = config.as_provider();

        let request = configure_request(config, &matches);
        let result = request.send().unwrap();

        assert_eq!(result.status(), StatusCode::CREATED);
    }
}
