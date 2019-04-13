use super::Provider;
use serde::Serialize;
use serde_json::to_string;
use structopt::StructOpt;

#[derive(Serialize, StructOpt)]
pub struct GitlabArgs {
    #[structopt(
        short = "k",
        long = "token",
        help = "A personal access token. Alternatively read from GITLAB_REPO_TOKEN env variable",
        env = "GITLAB_REPO_TOKEN",
        index = 1
    )]
    #[serde(skip_serializing)]
    pub token: String,
    #[structopt(
        short = "n",
        long = "name",
        help = "The name of the new project. Equals path if not provided.",
        conflicts_with = "path",
        required_unless = "path",
        index = 2
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[structopt(
        short = "p",
        long = "path",
        long_help = "Repository name for new project. Generated based on name if not provided (generated lowercased with dashes).",
        conflicts_with = "name",
        required_unless = "name",
        index = 3
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[structopt(
        long = "visibility",
        help = "The visibility of the project `public, internal, private`",
        possible_value = "public",
        possible_value = "internal",
        possible_value = "private"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<String>,
    #[structopt(
        short = "d",
        long = "description",
        help = "A short description of the repository."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[structopt(
        short = "i",
        long = "issues",
        help = "Enable or disable issues for this repo. Defaults to true."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_enabled: Option<bool>,
    #[structopt(
        short = "w",
        long = "wiki",
        help = "Enables or disables wikis for this repo. Defaults to true."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_enabled: Option<bool>,
    #[structopt(
        long = "namespace_id",
        long_help = "Namespace for the new project (defaults to the current user’s namespace)"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_id: Option<u32>,
    #[structopt(
        short = "m",
        long = "merge",
        help = "Enables or disables merge requests. Defaults to true."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_enabled: Option<bool>,
    #[structopt(
        long = "default_branch",
        help = "Set default branch. Defaults to master."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch: Option<String>,
    #[structopt(long = "jobs", help = "Enables or disables jobs for this repo.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    jobs_enabled: Option<bool>,
    #[structopt(
        long = "snippets",
        help = "Enables or disables snippets for this repo."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    snippets_enabled: Option<bool>,
    #[structopt(
        long = "container_registry",
        help = "Enables or disables container registry for this repo."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_enabled: Option<bool>,
    #[structopt(
        long = "shared_runners",
        help = "Enables or disables shared runners for this repo."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_enabled: Option<bool>,
    #[structopt(long = "import_url", help = "Supply a URL to import a repo from.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    import_url: Option<String>,
}
//TODO: The rest of the options in https://docs.gitlab.com/ee/api/projects.html#create-project

const ENDPOINT: &str = "https://gitlab.com/api/v4/projects";

impl Provider for GitlabArgs {
    fn payload(&self) -> String {
        to_string(&self).unwrap()
    }

    fn endpoint(&self) -> String {
        ENDPOINT.to_string()
    }

    fn extract_url(&self, _: &reqwest::header::HeaderMap) -> String {
        "Gitlab doesn't respond with your username or the URL to the created entity. But, it should be there.".to_string()
    }
}
