#![feature(custom_attribute)]

use std::env::var;
use std::process::exit;

use structopt::StructOpt;
use reqwest::StatusCode;
use serde::Serialize;

const ENDPOINT: &str = "https://api.github.com/user/repos";
const ORG_ENDPOINT: &str = "https://api.github.com/orgs/{}/repos";

#[derive(Serialize, StructOpt)]
#[structopt(
    name = "Github Repo Creator",
    about = "A small program to create github repos from the command line."
)]
struct RepoParams {
    #[structopt(short = "n", long = "name", help = "The name of the new repository.")]
    name: String,
    #[structopt(
        short = "d",
        long = "description",
        help = "A short description of the repository."
    )]
    description: Option<String>,
    #[structopt(
        long = "homepage",
        help = "A URL with more information about the repository."
    )]
    homepage: Option<String>,
    #[structopt(
        short = "p",
        long = "private",
        help = "Requires 'repo' scope on your personal access token"
    )]
    private: Option<bool>,
    #[structopt(
        short = "i",
        long = "issues",
        help = "Enable or disable issues for this repo. Github defaults this to true."
    )]
    has_issues: Option<bool>,
    #[structopt(
        short = "j",
        long = "projects",
        help = "Enables or disables projects for this repo. Github defaults this to true."
    )]
    has_projects: Option<bool>,
    #[structopt(
        short = "w",
        long = "wiki",
        help = "Enables or disables wikis for this repo. Github defaults this to true."
    )]
    has_wiki: Option<bool>,
    #[structopt(
        short = "t",
        long = "team",
        help = "Id of the team that has access to this repo. Only valid when using --org."
    )]
    team_id: Option<u32>,
    #[structopt(
        short = "a",
        long = "auto-init",
        help = "Creates an initial commit with empty README.md. Github defaults this to false."
    )]
    auto_init: Option<bool>,
    #[structopt(
        short = "g",
        long = "gitignore",
        help = "Language template: ex 'Rust'. View more templates at https://github.com/github/gitignore"
    )]
    gitignore_template: Option<String>,
    #[structopt(
        short = "l",
        long = "license",
        help = "License template: ex 'mit' or 'mpl-2.0'. View more at https://choosealicense.com/"
    )]
    license_template: Option<String>,
    #[structopt(
        short = "s",
        long = "squash",
        help = "Enables or disables squash-merging for this repo. Github defaults this to true."
    )]
    allow_squash_merge: Option<bool>,
    #[structopt(
        short = "m",
        long = "merge",
        help = "Enables or disables merging with a merge commit. Github defaults this to true."
    )]
    allow_merge_commit: Option<bool>,
    #[structopt(
        short = "r",
        long = "rebase",
        help = "Enables or disables rebase-merging for this repo. Github defaults this to true."
    )]
    allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing)]
    #[structopt(
        short = "o",
        long = "org",
        help = "Creates the repo under an organization. Requires you have CREATE REPO permissions in that org."
    )]
    org: Option<String>,
    #[serde(skip_serializing)]
    #[structopt(short = "v", long = "verbose", help = "Prints http response data.")]
    verbose: bool,
}

impl Default for RepoParams {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
            homepage: None,
            private: None,
            has_issues: None,
            has_projects: None,
            has_wiki: None,
            team_id: None,
            auto_init: None,
            gitignore_template: None,
            license_template: None,
            allow_squash_merge: None,
            allow_merge_commit: None,
            allow_rebase_merge: None,
            org: None,
            verbose: false,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = RepoParams::from_args();
    let body = serde_json::to_string(&params)?;

    let token = var("GITHUB_REPO_TOKEN").unwrap_or_else(|_| {
        eprintln!("You must add a token with (public) REPO scope to the env variable GITHUB_REPO_TOKEN: https://github.com/settings/tokens");
        std::process::exit(1);
    });

    let client = reqwest::Client::new();
    let endpoint = if let Some(org) = params.org {
        ORG_ENDPOINT.replace("{}", &org)
    } else {
        ENDPOINT.to_string()
    };
    let result = client
        .post(&endpoint)
        .body(body)
        .header("Authorization", format!("token {}", token))
        .send()?;

    if params.verbose {
        println!("{:#?}", &result);
    }

    let status = result.status();
    let headers = result.headers();
    match status {
        StatusCode::CREATED => {
            let apiloc = headers
                .get("location")
                .and_then(|x| x.to_str().ok())
                .unwrap_or("https://github.com");
            let apiloc = apiloc.replace("api.", "");
            let apiloc = apiloc.replace("repos/", "");
            println!("Repo created: {}", apiloc);
        }
        StatusCode::UNPROCESSABLE_ENTITY => {
            eprintln!("Github had an issue processing this request. Perhaps the repository already exists, or you're using an unsupported option. e.g. Enabling projects on a repo in an org that has them disabled.");
            exit(2);
        }
        _ => {
            eprintln!("An unknown response was sent from github.");
            exit(3);
        }
    };

    Ok(())
}
