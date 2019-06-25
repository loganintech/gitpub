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
    fn check_success(matches: clap::ArgMatches) {
        let config = Gitpo::from_matches(&matches);
        let config = config.as_provider();

        let request = configure_request(config, &matches);
        let result = request.send().unwrap();

        let status = result.status();

        if status != StatusCode::CREATED && status != StatusCode::OK {
            eprintln!("Status: {}", status);
            panic!();
        }
    }

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
            include_str!("../test_config.txt")
                .trim_end()
                .split("\n")
                .nth(0)
                .unwrap()
                .split(" ")
                .nth(1)
                .unwrap(),
            "--disable_merge",
            "--disable_rebase",
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

        check_success(matches);
    }

    #[test]
    fn github_integration_disable_squash() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let matches = cli::get_app().get_matches_from(vec![
            "gitpub",
            "github",
            "-t",
            include_str!("../test_config.txt")
                .trim_end()
                .split("\n")
                .nth(0)
                .unwrap()
                .split(" ")
                .nth(1)
                .unwrap(),
            "--disable_squash",
            "-n",
            &format!("Test Disable Squash {}", secs),
        ]);

        check_success(matches);
    }

    #[test]
    fn gitlab() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let matches = cli::get_app().get_matches_from(vec![
            "gitpub",
            "gitlab",
            "-t",
            include_str!("../test_config.txt")
                .trim_end()
                .split("\n")
                .nth(1)
                .unwrap()
                .split(" ")
                .nth(1)
                .unwrap(),
            "--disable_container_registry",
            "--disable_jobs",
            "--disable_merge",
            "--disable_shared_runners",
            "--disable_snippets",
            "--discussion_resolution_required",
            "-i",
            "-w",
            "-r",
            "--large_file_support",
            "--merge_request_link_on_commit",
            "--mirror",
            "--mirror_triggers_builds",
            "--pipeline_success_required",
            "--public_builds",
            "--request_access_enabled",
            "-d", &format!("Description {}", secs),
            "--ci_config_path", "./example",
            "--default_branch", "dev",
            // "--merge_method", "ff",
            "-n",
            &format!("Test Repo {}", secs),
            "--visibility", "private",
        ]);

        check_success(matches);
    }
}
