#![feature(bind_by_move_pattern_guards)]

use cli::Gitpo;
use reqwest::StatusCode;
use std::process::exit;
mod cli;
mod git;
mod provider;

use git::add_remote;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::get_app().get_matches();
    let config = Gitpo::from_matches(&matches);
    let config = config.as_provider();
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

    let result = request.send()?;
    let status = result.status();
    let headers = result.headers();
    match status {
        StatusCode::OK | StatusCode::CREATED => {
            let apiloc = config.extract_url(&headers);
            println!("Repo created: {}", apiloc);
            let remote_name = matches
                .value_of("remote_name")
                .expect("This should default to origin, so something is wrong.");
            if matches.is_present("set_remote") && !add_remote(remote_name, &apiloc) {
                eprintln!("Couldn't set remote.");
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
