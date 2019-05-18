use super::Provider;
use clap::{App, Arg, ArgMatches, SubCommand};
use serde::Serialize;
use serde_json::to_string;

#[derive(Serialize)]
pub struct BitbucketArgs<'a> {
    name: &'a str,
    #[serde(skip_serializing)]
    token: &'a str,
    #[serde(skip_serializing)]
    username: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    is_private: bool,
    has_wiki: bool,
    has_issues: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    fork_policy: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scm: Option<&'a str>,
    language: Option<&'a str>,
}

const ENDPOINT: &str = "https://api.bitbucket.org/2.0/repositories/{username}/{slug}";

impl<'a> Provider for BitbucketArgs<'a> {
    fn payload(&self) -> String {
        to_string(&self).unwrap()
    }

    fn endpoint(&self) -> String {
        ENDPOINT
            .replace("{username}", &self.username)
            .replace("{slug}", &self.name)
    }

    fn extract_url(&self, _: &reqwest::header::HeaderMap) -> String {
        format!("https://bitbucket.org/{}/{}", &self.username, &self.name)
    }

    fn token(&self) -> String {
        format!(
            "Basic {}",
            base64::encode(&format!("{}:{}", &self.username, &self.token))
        )
    }

    fn auth_header(&self) -> String {
        "Authorization".to_string()
    }

    fn ssh_url(&self, _: &reqwest::header::HeaderMap) -> Option<String> {
        Some(format!(
            "git@bitbucket.com:{}/{}.git",
            &self.username, &self.name
        ))
    }
}

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("bitbucket")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Create a repo on bitbucket.")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("The name of the new repository.")
                .takes_value(true)
                .required(true)
        ).arg(
            Arg::with_name("username")
                .long("username")
                .help("Your bitbucket username. Alternatively read from BITBUCKET_USERNAME env variable.")
                .takes_value(true)
                .required(true)
                .env("BITBUCKET_USERNAME")
        ).arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .help(
                    "A personal access token. Alternatively read from BITBUCKET_REPO_TOKEN env variable.",
                )
                .env("BITBUCKET_REPO_TOKEN")
                .takes_value(true)
                .required(true)
        ).arg(
            Arg::with_name("description")
                .short("d")
                .long("description")
                .help("A short description of the repository.")
                .takes_value(true)
        ).arg(
            Arg::with_name("private")
                .short("p")
                .long("private")
                .help("Sets the repository to private.")
        ).arg(
            Arg::with_name("disable_wiki")
                .short("w")
                .long("disable_wiki")
                .help("Disables wikis for this repo. Defaults to false.")
        ).arg(
            Arg::with_name("disable_issues")
                .short("i")
                .long("disable_issues")
                .help("Disables issues for this repo. Defaults to false.")
        ).arg(
            Arg::with_name("fork_policy")
                .long("fork_policy")
                .help("Changes the allowed forking method for this repo.")
                .possible_values(&["allow_forks", "no_public_forks", "no_forks"])
                .takes_value(true)
        ).arg(
            Arg::with_name("scm")
                .long("scm")
                .help("Control underlying source control method.")
                .possible_values(&["git", "hg"])
                .takes_value(true)
        ).arg(
            Arg::with_name("language")
                .long("language")
                .help("Give bitbucket a hint about the programming language.")
                .takes_value(true)
        )
}

pub fn from_matches<'a>(matches: &'a ArgMatches) -> BitbucketArgs<'a> {
    BitbucketArgs {
        name: matches
            .value_of("name")
            .expect("This property is required. This error should be unreachable."),
        token: matches
            .value_of("token")
            .expect("Couldn't get the token. This should be unreachable."),
        username: matches
            .value_of("username")
            .expect("This property is requred. This error should be unreachable."),
        description: matches.value_of("description"),
        is_private: matches.is_present("private"),
        has_issues: !matches.is_present("disable_issues"),
        has_wiki: !matches.is_present("disable_wiki"),
        fork_policy: matches.value_of("fork_policy"),
        scm: matches.value_of("scm"),
        language: matches.value_of("language"),
    }
}
