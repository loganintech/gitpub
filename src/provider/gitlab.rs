use super::Provider;
use serde::Serialize;
use serde_json::to_string;
use structopt::StructOpt;

#[derive(Serialize, StructOpt)]
pub struct GitlabArgs {
    #[structopt(
        short = "t",
        long = "token",
        help = "A personal access token. Alternatively read from GITLAB_REPO_TOKEN env variable",
        long_help = "A personal access token. Alternatively read from GITLAB_REPO_TOKEN env variable",
        env = "GITLAB_REPO_TOKEN"
    )]
    #[serde(skip_serializing)]
    pub token: String,
    #[structopt(
        short = "n",
        long = "name",
        help = "The name of the new project. Equals path if not provided.",
        long_help = "The name of the new project. Equals path if not provided.",
        conflicts_with = "path",
        required_unless = "path"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[structopt(
        long = "path",
        help = "Repository name for new project. Generated based on name if not provided (generated lowercased with dashes).",
        long_help = "Repository name for new project. Generated based on name if not provided (generated lowercased with dashes).",
        conflicts_with = "name",
        required_unless = "name"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[structopt(
        short = "p", //Matches github's "privacy"
        long = "visibility",
        help = "The visibility of the project.",
        long_help = "The visibility of the project.",
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
        help = "Enable or disable issues for this repo. Defaults to true.",
        long_help = "Enable or disable issues for this repo. Defaults to true."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_enabled: Option<bool>,
    #[structopt(
        short = "w",
        long = "wiki",
        help = "Enables or disables wikis for this repo. Defaults to true.",
        long_help = "Enables or disables wikis for this repo. Defaults to true."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_enabled: Option<bool>,
    #[structopt(
        long = "namespace_id",
        help = "Namespace for the new project (defaults to the current user’s namespace)",
        long_help = "Namespace for the new project (defaults to the current user’s namespace)"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_id: Option<u32>,
    #[structopt(
        long = "merge",
        help = "Enables or disables merge requests. Defaults to true.",
        long_help = "Enables or disables merge requests. Defaults to true."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_enabled: Option<bool>,
    #[structopt(
        long = "default_branch",
        help = "Set default branch. Defaults to master.",
        long_help = "Set default branch. Defaults to master."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch: Option<String>,
    #[structopt(
        long = "jobs",
        long_help = "Enables or disables jobs for this repo.",
        help = "Enables or disables jobs for this repo."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    jobs_enabled: Option<bool>,
    #[structopt(
        long = "snippets",
        help = "Enables or disables snippets for this repo.",
        long_help = "Enables or disables snippets for this repo."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    snippets_enabled: Option<bool>,
    #[structopt(
        long = "container_registry",
        help = "Enables or disables container registry for this repo.",
        long_help = "Enables or disables container registry for this repo."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_enabled: Option<bool>,
    #[structopt(
        long = "shared_runners",
        help = "Enables or disables shared runners for this repo.",
        long_help = "Enables or disables shared runners for this repo."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_enabled: Option<bool>,
    #[structopt(
        long = "import_url",
        long_help = "Supply a URL to import a repo from.",
        help = "Supply a URL to import a repo from."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    import_url: Option<String>,
    #[structopt(
        long = "public_builds",
        help = "Lets jobs be viewed by non-project members.",
        long_help = "Lets jobs be viewed by non-project members."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    public_builds: Option<bool>,
    #[structopt(
        long = "pipeline_success_required",
        help = "Requires pipelines to succeed before a merge.",
        long_help = "Requires pipelines to succeed before a merge."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    only_allow_merge_if_pipeline_succeeds: Option<bool>,
    #[structopt(
        long = "discussion_resolution_required",
        help = "Requires discussions are resolved before a merge.",
        long_help = "Requires discussions are resolved before a merge."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    only_allow_merge_if_all_discussions_are_resolved: Option<bool>,
    #[structopt(long = "merge_method", help = "Sets the merge method.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_method: Option<String>,
    #[structopt(
        long = "lfs",
        alias = "lfs_enabled",
        alias = "large_file_support",
        help = "Enables git large file support.",
        long_help = "Enables git large file support."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    lfs_enabled: Option<bool>,
    #[structopt(
        long = "request_access_enabled",
        help = "Allow users to request member access.",
        long_help = "Allow users to request member access."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    request_access_enabled: Option<bool>,
    #[structopt(
        long = "tag_list",
        long_help = "A list of tags for a repo.",
        help = "A list of tags for a repo."
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tag_list: Vec<String>,
    #[structopt(
        long = "merge_request_link_on_commit",
        help = "Print the merge request link when committing to the repository.",
        long_help = "Print the merge request link when committing to the repository."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    printing_merge_request_link_enabled: Option<bool>,
    #[structopt(long = "ci_config_path", help = "Path to CI config file.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_config_path: Option<String>,
    #[structopt(
        long = "repository_storage",
        help = "Which storage shard the repo is one. Available only to admins.",
        long_help = "Which storage shard the repo is one. Available only to admins."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_storage: Option<String>,
    #[structopt(
        long = "approvals_before_merge",
        help = "Number of approvals required before a merge.",
        long_help = "Number of approvals required before a merge."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    approvals_before_merge: Option<u32>,
    #[structopt(
        long = "mirror",
        long_help = "Enables pull mirroring in a project.",
        help = "Enables pull mirroring in a project."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror: Option<bool>,
    #[structopt(
        long = "mirror_trigger_builds",
        help = "Enables builds on pull mirroring.",
        long_help = "Enables builds on pull mirroring."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror_trigger_builds: Option<bool>,
    #[structopt(
        short = "r",
        long = "initialize_with_readme",
        help = "Create a blank README in the project.",
        long_help = "Create a blank README in the project."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    initialize_with_readme: Option<bool>,
}
//TODO: The rest of the options in https://docs.gitlab.com/ee/api/projects.html#create-project

impl GitlabArgs {
    fn project_name(&self) -> String {
        if let Some(ref p) = self.path {
            p.to_string()
        } else if let Some(ref n) = self.name {
            n.to_string()
        } else {
            unreachable!()
        }
    }
}

const ENDPOINT: &str = "https://gitlab.com/api/v4/projects";

impl Provider for GitlabArgs {
    fn payload(&self) -> String {
        to_string(&self).unwrap()
    }

    fn endpoint(&self) -> String {
        ENDPOINT.to_string()
    }

    fn extract_url(&self, _: &reqwest::header::HeaderMap) -> String {
        match std::env::var("GITLAB_USERNAME") {
            Ok(u) => format!("https://gitlab.com/{}/{}", u, self.project_name()),
            _ => "https://gitlab.com - Gitlab doesn't respond with your username or the URL to the created entity. But, it should be there. Alternatively set the GITLAB_USERNAME environment variable.".to_string(),
        }
    }
}
