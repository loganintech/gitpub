use super::Provider;
use clap::{App, Arg, ArgMatches, SubCommand};
use serde::Serialize;
use serde_json::to_string;

#[derive(Serialize)]
pub struct GitlabArgs<'a> {
    #[serde(skip_serializing)]
    token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    issues_enabled: bool,
    wiki_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_id: Option<u32>,
    merge_requests_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch: Option<&'a str>,
    jobs_enabled: bool,
    snippets_enabled: bool,
    container_registry_enabled: bool,
    shared_runners_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_url: Option<&'a str>,
    public_builds: bool,
    only_allow_merge_if_pipeline_succeeds: bool,
    only_allow_merge_if_all_discussions_are_resolved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_method: Option<&'a str>,
    lfs_enabled: bool,
    request_access_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_list: Option<Vec<&'a str>>,
    printing_merge_request_link_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_config_path: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_storage: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approvals_before_merge: Option<u32>,
    mirror: bool,
    mirror_trigger_builds: bool,
    initialize_with_readme: bool,
}
//TODO: The rest of the options in https://docs.gitlab.com/ee/api/projects.html#create-project

impl<'a> GitlabArgs<'a> {
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

impl<'a> Provider for GitlabArgs<'a> {
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

    fn token(&self) -> String {
        self.token.to_string()
    }

    fn auth_header(&self) -> String {
        "Private-Token".to_string()
    }
}

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("gitlab")
        .about("Create a repo on gitlab.")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("The name of the new repository.")
                .conflicts_with("path")
                .required_unless("path")
                .takes_value(true)
        ).arg(
            Arg::with_name("path")
                .long("path")
                .help("The path for the new repo. Generated based on name if not provided (generated lowercased with dashes).")
                .conflicts_with("name")
                .required_unless("name")
                .takes_value(true)
        ).arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .help(
                    "A personal access token. Alternatively read from GITLAB_REPO_TOKEN env variable.",
                )
                .env("GITLAB_REPO_TOKEN")
                .takes_value(true)
                .required(true)
        ).arg(
            Arg::with_name("visibility")
                .long("visibility")
                .takes_value(true)
                .help("The visibility of the project")
                .possible_values(&["public", "internal", "private"])
        ).arg(
            Arg::with_name("description")
                .short("d")
                .long("description")
                .help("A short description of the repository.")
                .takes_value(true)
        ).arg(
            Arg::with_name("disable_issues")
                .short("i")
                .long("disable_issues")
                .help("Disables issues for this repo.")
        ).arg(
            Arg::with_name("disable_wiki")
                .short("w")
                .long("disable_wiki")
                .help("Disables wikis for this repo.")
        ).arg(
            Arg::with_name("namespace_id")
                .long("namespace_id")
                .help("Namespace for the new project (defaults to the current user’s namespace)")
                .takes_value(true)
        ).arg(
            Arg::with_name("disable_merge")
                .long("disable_merge")
                .help("Disables merging with a merge commit.")
        ).arg(
            Arg::with_name("default_branch")
                .long("default_branch")
                .help("Set default branch. Defaults to master.")
                .takes_value(true)
        ).arg(
            Arg::with_name("disable_jobs")
                .long("disable_jobs")
                .help("Disables jobs for this repo.")
        ).arg(
            Arg::with_name("disable_snippets")
                .long("disable_snippets")
                .help("Disables snippets for this repo.")
        ).arg(
            Arg::with_name("disable_container_registry")
                .long("disable_container_registry")
                .help("Disables container registry for this repo.")
        ).arg(
            Arg::with_name("disable_shared_runners")
                .long("disable_shared_runners")
                .help("Disables shared runners for this repo.")
        ).arg(
            Arg::with_name("import_url")
                .long("import_url")
                .help("Supply a URL to import a repo from.")
                .takes_value(true)
        ).arg(
            Arg::with_name("public_builds")
                .long("public_builds")
                .help("Lets jobs be viewed by non-project members.")
        ).arg(
            Arg::with_name("pipeline_success_required")
                .long("pipeline_success_required")
                .help("Requires pipelines to succeed before a merge.")
        ).arg(
            Arg::with_name("discussion_resolution_required")
                .long("discussion_resolution_required")
                .help("Requires discussions are resolved before a merge.")
        ).arg(
            Arg::with_name("merge_method")
                .long("merge_method")
                .help("Sets the merge method.")
                .takes_value(true)
                .possible_values(&["merge", "rebase_merge", "ff"])
        ).arg(
            Arg::with_name("lfs")
                .long("large_file_support")
                .alias("lfs_enabled")
                .alias("lfs")
                .help("Enables git large file support.")
        ).arg(
            Arg::with_name("request_access_enabled")
                .long("request_access_enabled")
                .help("Allow users to request member access.")
        ).arg(
            Arg::with_name("tag_list")
                .long("tag_list")
                .takes_value(true)
                .multiple(true)
                .max_values(4)
                .help("A list of tags for a repo.")
        )
        .arg(
            Arg::with_name("merge_request_link_on_commit")
                .long("merge_request_link_on_commit")
                .help("Print the merge request link when committing to the repository.")
        ).arg(
            Arg::with_name("ci_config_path")
                .long("ci_config_path")
                .takes_value(true)
                .help("Path to CI config file.")
        ).arg(
            Arg::with_name("repository_storage")
                .long("repository_storage")
                .takes_value(true)
                .help("An advanced parameter to designate which shard the repo is on, available only to admins.")
        ).arg(
            Arg::with_name("approvals_before_merge")
                .long("approvals_before_merge")
                .takes_value(true)
                .help("Number of approvals required before a merge.")
        ).arg(
            Arg::with_name("mirror")
                .long("mirror")
                .help("Enables pull mirroring in a project.")
        ).arg(
            Arg::with_name("mirror_triggers_builds")
                .long("mirror_triggers_builds")
                .help("Enables builds on pull mirroring.")
        ).arg(
            Arg::with_name("initialize_with_readme")
                .short("r")
                .long("initialize_with_readme")
                .help("Create a blank README for the project.")
        )
}

pub fn from_matches<'a>(matches: &'a ArgMatches) -> GitlabArgs<'a> {
    GitlabArgs {
        name: matches.value_of("name"),
        token: matches
            .value_of("token")
            .expect("Couldn't get the token. This should be unreachable."),
        path: matches.value_of("path"),
        visibility: matches.value_of("visibility"),
        description: matches.value_of("description"),
        issues_enabled: !matches.is_present("disable_issues"),
        wiki_enabled: !matches.is_present("disable_wiki"),
        namespace_id: matches
            .value_of("namespace_id")
            .and_then(|x| x.parse::<u32>().ok()),
        merge_requests_enabled: matches.is_present("disable_merge"),
        default_branch: matches.value_of("default_branch"),
        jobs_enabled: !matches.is_present("disable_jobs"),
        snippets_enabled: !matches.is_present("disable_snippets"),
        container_registry_enabled: !matches.is_present("disable_container_registry"),
        shared_runners_enabled: !matches.is_present("disable_shared_runners"),
        import_url: matches.value_of("import_url"),
        public_builds: matches.is_present("public_builds"),
        only_allow_merge_if_pipeline_succeeds: matches.is_present("pipeline_success_required"),
        only_allow_merge_if_all_discussions_are_resolved: matches
            .is_present("discussion_resolution_required"),
        merge_method: matches.value_of("merge_method"),
        lfs_enabled: matches.is_present("lfs"),
        request_access_enabled: matches.is_present("request_access_enabled"),
        tag_list: matches
            .values_of("tag_list")
            .and_then(|x| Some(x.collect::<Vec<_>>())),
        printing_merge_request_link_enabled: matches.is_present("merge_request_link_on_commit"),
        ci_config_path: matches.value_of("ci_config_path"),
        repository_storage: matches.value_of("repository_storage"),
        approvals_before_merge: matches
            .value_of("approvals_before_merge")
            .and_then(|x| x.parse::<u32>().ok()),
        mirror: matches.is_present("mirror"),
        mirror_trigger_builds: matches.is_present("mirror_trigger_builds"),
        initialize_with_readme: matches.is_present("initialize_with_readme"),
    }
}
