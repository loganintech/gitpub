use super::Provider;
use serde::Serialize;
use serde_json::to_string;
use structopt::StructOpt;

#[derive(Serialize, StructOpt)]
struct GitRepoRef {
    #[structopt(long = "parent_id")]
    id: Option<String>,
    #[structopt(long = "parent_name")]
    name: Option<String>,
    #[structopt(long = "parent_url")]
    url: Option<String>,
    #[structopt(long = "parent_state")]
    state: Option<String>,
}

#[derive(Serialize, StructOpt)]
pub struct AzureArgs {

    // #[structopt(flatten)]
    // #[serde(rename = "parentRepository")]
    // parent_repository: Option<GitRepoRef>,



    #[structopt(long = "project", help = "The name or project id of the new repository.")]
    project: Option<String>,
    #[structopt(
        short = "t",
        long = "token",
        help = "A personal access token. Alternatively read from AZURE_REPO_TOKEN env variable",
        env = "AZURE_REPO_TOKEN"
    )]
    #[serde(skip_serializing)]
    pub token: String,
    #[structopt(
        long = "source_ref",
        help = "Specify the source refs to use while creating a fork repo."
    )]
    #[serde(skip_serializing)]
    source_ref: Option<String>,
    #[structopt(
        long = "team",
        help = "Id of the team that has access to this repo. Only valid when using --org.",
        requires = "org"
    )]
    team_id: Option<u32>,
    #[serde(skip_serializing)]
    #[structopt(
        long = "org",
        help = "Creates the repo under an organization. Requires you have CREATE REPO permissions in that org."
    )]
    org: String,
    #[serde(skip_serializing)]
    #[structopt(
        short = "e",
        long = "endpoint",
        help = "Allows redirection of requests to enterprise providers.",
        conflicts_with = "org"
    )]
    custom_endpoint: Option<String>,
}

const ENDPOINT: &str = "https://dev.azure.com/{organization}/{project}/_apis/git/repositories?api-version=5.0";

impl Provider for AzureArgs {
    fn payload(&self) -> String {
        to_string(&self).unwrap()
    }

    fn endpoint(&self) -> String {
        if let Some(custom_endpoint) = &self.custom_endpoint {
            custom_endpoint.to_string()
        } else {
            let project = if let Some(proj) = &self.project {
                format!("/{}", proj)
            } else {
                "".to_string()
            };

            let mut endpoint = ENDPOINT
                .replace("{organization}", &self.org)
                .replace("/{project}", &project);

            if let Some(src_ref) = &self.source_ref {
                endpoint.push_str(&format!("&sourceRef{}", src_ref))
            };

            endpoint
        }
    }

    fn extract_url(&self, headers: &reqwest::header::HeaderMap) -> String {
        unimplemented!()
    }

}
