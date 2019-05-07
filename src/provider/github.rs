use super::Provider;
use clap::{App, Arg, ArgMatches, SubCommand};
use serde::Serialize;
use serde_json::to_string;

#[derive(Serialize)]
pub struct GithubArgs<'a> {
    name: &'a str,
    #[serde(skip_serializing)]
    token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage: Option<&'a str>,
    private: bool,
    has_issues: bool,
    has_projects: bool,
    has_wiki: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_id: Option<u32>,
    auto_init: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitignore_template: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_template: Option<&'a str>,
    allow_squash_merge: bool,
    allow_merge_commit: bool,
    allow_rebase_merge: bool,
    org: Option<&'a str>,
}

const ENDPOINT: &str = "https://api.github.com/user/repos";
const ORG_ENDPOINT: &str = "https://api.github.com/orgs/{}/repos";

impl<'a> Provider for GithubArgs<'a> {
    fn payload(&self) -> String {
        to_string(&self).unwrap()
    }

    fn endpoint(&self) -> String {
        if let Some(org) = &self.org {
            ORG_ENDPOINT.replace("{}", &org)
        } else {
            ENDPOINT.to_string()
        }
    }

    fn extract_url(&self, headers: &reqwest::header::HeaderMap) -> String {
        let src = headers
            .get("location")
            .and_then(|x| x.to_str().ok())
            .unwrap_or("https://github.com");
        src.replace("api.", "").replace("repos/", "")
    }

    fn token(&self) -> String {
        format!("token {}", self.token)
    }

    fn auth_header(&self) -> String {
        "Authorization".to_string()
    }
}

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("github")
        .about("Create a repo on github.")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("The name of the new repository.")
                .takes_value(true)
                .required(true)
        ).arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .help(
                    "A personal access token. Alternatively read from GITHUB_REPO_TOKEN env variable.",
                )
                .env("GITHUB_REPO_TOKEN")
                .takes_value(true)
                .required(true)
        ).arg(
            Arg::with_name("description")
                .short("d")
                .long("description")
                .help("A short description of the repository.")
                .takes_value(true)
        ).arg(
            Arg::with_name("homepage")
                .long("homepage")
                .help("A URL with more information about the repository.")
                .takes_value(true)
        ).arg(
            Arg::with_name("private")
                .short("p")
                .long("private")
                .help("Sets the repository to private. Required 'repo' scope on your personal access token.")
        ).arg(
            Arg::with_name("disable_issues")
                .short("i")
                .long("disable_issues")
                .help("Disables issues for this repo. Defaults to false.")
        ).arg(
            Arg::with_name("disable_projects")
                .long("disable_projects")
                .help("Disables projects for this repo. Defaults to false.")
        ).arg(
            Arg::with_name("disable_wiki")
                .short("w")
                .long("disable_wiki")
                .help("Disables wikis for this repo. Defaults to false.")
        ).arg(
            Arg::with_name("team")
                .long("team")
                .help("ID of the team that has access to this repo. Only valid when using --org")
                .requires("org")
        ).arg(
            Arg::with_name("readme")
                .short("r")
                .long("initialize_with_readme")
                .help("Creates an initial commit with empty README.md. Defaults to false.")
        ).arg(
            Arg::with_name("gitignore")
                .long("gitignore")
                .takes_value(true)
                .help("Language template: ex 'Rust'. View more templates at https://github.com/github/gitignore/")
        ).arg(
            Arg::with_name("license")
                .long("license")
                .takes_value(true)
                .help("License template: ex 'mit' or 'mpl-2.0'. View more at https://choosealicense.com/")
        ).arg(
            Arg::with_name("disable_squash")
                .long("squash")
                .help("Enables or disables squash-merging for this repo. Defaults to false.")
        ).arg(
            Arg::with_name("disable_merge")
                .long("merge")
                .help("Enables or disables merging with a merge commit. Defaults to false.")
        ).arg(
            Arg::with_name("disable_rebase")
                .long("rebase")
                .help("Enables or disables rebase-merging for this repo. Defaults to false.")
        ).arg(
            Arg::with_name("org")
                .long("org")
                .help("Creates the repo under an organization. Requires you have CREATE REPO permissions in that org.")
        )
}

pub fn from_matches<'a>(matches: &'a ArgMatches) -> GithubArgs<'a> {
    GithubArgs {
        name: matches
            .value_of("name")
            .expect("This property is required. This statement should be unreachable."),
        token: matches
            .value_of("token")
            .expect("This property is required. This error should be unreachable."),
        description: matches.value_of("description"),
        homepage: matches.value_of("homepage"),
        private: matches.is_present("private"),
        has_issues: !matches.is_present("disable_issues"),
        has_projects: !matches.is_present("disable_projects"),
        has_wiki: !matches.is_present("disable_wiki"),
        team_id: matches.value_of("team").and_then(|x| x.parse::<u32>().ok()),
        auto_init: matches.is_present("readme"),
        gitignore_template: matches.value_of("gitignore"),
        license_template: matches.value_of("license"),
        allow_squash_merge: !matches.is_present("disable_squash"),
        allow_merge_commit: !matches.is_present("disable_merge"),
        allow_rebase_merge: !matches.is_present("disable_rebase"),
        org: matches.value_of("org"),
    }
}
