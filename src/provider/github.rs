use super::Provider;
use serde::Serialize;
use serde_json::to_string;
use structopt::StructOpt;

#[derive(Serialize, StructOpt)]
pub struct GithubArgs {
    #[structopt(short = "n", long = "name", help = "The name of the new repository.")]
    name: String,
    #[structopt(
        short = "t",
        long = "token",
        help = "A personal access token. Alternatively read from GITHUB_REPO_TOKEN env variable",
        env = "GITHUB_REPO_TOKEN"
    )]
    #[serde(skip_serializing)]
    pub token: String,
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
        help = "Enable or disable issues for this repo. Defaults to true."
    )]
    has_issues: Option<bool>,
    #[structopt(
        long = "projects",
        help = "Enables or disables projects for this repo. Defaults to true."
    )]
    has_projects: Option<bool>,
    #[structopt(
        short = "w",
        long = "wiki",
        help = "Enables or disables wikis for this repo. Defaults to true."
    )]
    has_wiki: Option<bool>,
    #[structopt(
        long = "team",
        help = "Id of the team that has access to this repo. Only valid when using --org.",
        requires = "org"
    )]
    team_id: Option<u32>,
    #[structopt(
        short = "r",
        long = "initialize_with_readme",
        help = "Creates an initial commit with empty README.md. Defaults to false."
    )]
    auto_init: Option<bool>,
    #[structopt(
        long = "gitignore",
        help = "Language template: ex 'Rust'. View more templates at https://github.com/github/gitignore"
    )]
    gitignore_template: Option<String>,
    #[structopt(
        long = "license",
        help = "License template: ex 'mit' or 'mpl-2.0'. View more at https://choosealicense.com/"
    )]
    license_template: Option<String>,
    #[structopt(
        long = "squash",
        help = "Enables or disables squash-merging for this repo. Defaults to true."
    )]
    allow_squash_merge: Option<bool>,
    #[structopt(
        long = "merge",
        help = "Enables or disables merging with a merge commit. Defaults to true."
    )]
    allow_merge_commit: Option<bool>,
    #[structopt(
        long = "rebase",
        help = "Enables or disables rebase-merging for this repo. Defaults to true."
    )]
    allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing)]
    #[structopt(
        long = "org",
        help = "Creates the repo under an organization. Requires you have CREATE REPO permissions in that org."
    )]
    org: Option<String>,
}

const ENDPOINT: &str = "https://api.github.com/user/repos";
const ORG_ENDPOINT: &str = "https://api.github.com/orgs/{}/repos";

impl Provider for GithubArgs {
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

}
