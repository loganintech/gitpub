use serde::Serialize;
use serde_json::to_string;
use std::env::var;
use structopt::StructOpt;

const ENDPOINT: &'static str = "https://api.github.com/user/repos";

#[derive(Serialize, StructOpt)]
#[structopt(
    name = "Github Repo Creator",
    about = "A small program to create github repos from the command line."
)]
struct RepoParams {
    #[structopt(short = "n", long = "name")]
    name: String,
    #[structopt(short = "d", long = "description")]
    description: Option<String>,
    #[structopt(long = "homepage")]
    homepage: Option<String>,
    #[structopt(short = "p", long = "private")]
    private: Option<bool>,
    #[structopt(short = "i", long = "issues")]
    has_issues: Option<bool>,
    #[structopt(short = "j", long = "projects")]
    has_projects: Option<bool>,
    #[structopt(short = "w", long = "wiki")]
    has_wiki: Option<bool>,
    #[structopt(short = "t", long = "team")]
    team_id: Option<u32>,
    #[structopt(short = "a", long = "auto-init")]
    auto_init: Option<bool>,
    #[structopt(
        short = "gl",
        long = "gitignore",
        help = "Language template: ex 'Rust'"
    )]
    gitignore_template: Option<String>,
    #[structopt(short = "l", long = "license")]
    license_template: Option<String>,
    #[structopt(short = "s", long = "squash")]
    allow_squash_merge: Option<bool>,
    #[structopt(short = "m", long = "merge")]
    allow_merge_commit: Option<bool>,
    #[structopt(short = "r", long = "rebase")]
    allow_rebase_merge: Option<bool>,
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
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = if let Ok(token) = var("GITHUB_REPO_TOKEN") {
        token
    } else {
        eprintln!("You must add a token with REPO scope to the env variable GITHUB_REPO_TOKEN");
        std::process::exit(1);
    };

    let params = RepoParams::from_args();
    let body = to_string(&params)?;

    let client = reqwest::Client::new();
    client
        .post(ENDPOINT)
        .body(body)
        .header("Authorization", format!("token {}", token))
        .send()?;
    Ok(())
}
