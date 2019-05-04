use super::Provider;
use reqwest::Client;

use serde::Serialize;
use serde_json::to_string;
use std::io::{BufRead, Write};

use structopt::StructOpt;
use azure_sdk_for_rust::cosmos::{AuthorizationToken, TokenType};


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
    #[structopt(
        long = "project",
        help = "The name or project id of the new repository."
    )]
    project: String,
    #[structopt(
        short = "t",
        long = "token",
        help = "A personal access token. Alternatively read from AZURE_REPO_TOKEN env variable. Will prompt you for authorization if unset.",
        env = "AZURE_REPO_TOKEN",
        hide_env_values = true
    )]
    #[serde(skip_serializing)]
    azure_code_response: Option<String>,
    #[structopt(
        long = "secret",
        help = "Azure app secret for your gitpub instance. Alternatively read from AZURE_CLIENT_SECRET env variable. Will prompt you for it if unset.",
        env = "AZURE_CLIENT_SECRET",
        hide_env_values = true
    )]
    azure_app_secret: Option<String>,
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

const ENDPOINT: &str =
    "https://dev.azure.com/{organization}/{project}/_apis/git/repositories?api-version=5.0";

// const AZURE_SECRET: &str = include_str!("../../azure_secret.txt");
// const OAUTH_URL: &str = "https://app.vssps.visualstudio.com/oauth2/authorize?client_id=DA74590B-1524-4EA4-A52C-115FB2C0C9AE&response_type=Assertion&state=astate&scope=vso.code_manage&redirect_uri=https://gitpub.jewsofhazard.com/code/";
// const TOKEN_URL: &str = "https://app.vssps.visua
// // let body = format!("client_assertion_type=urn:ietf:params:oauth:client-assertion-type:jwt-bearer&client_assertion={}&grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={}&redirect_uri={}", AZURE_SECRET, code, "https://gitpub.jewsofhazard.com/code/");




impl AzureArgs {
    pub fn token(&self) -> Result<String, Box<dyn std::error::Error>> {

        let url = "https://dev.azure.com/";
        let ctx = "https://login.microsoftonline.common";







        Ok("".to_owned())
    }
}

impl Provider for AzureArgs {
    fn payload(&self) -> String {
        to_string(&self).unwrap()
    }

    fn endpoint(&self) -> String {
        if let Some(custom_endpoint) = &self.custom_endpoint {
            custom_endpoint.to_string()
        } else {
            let project = if &self.project != "" {
                format!("/{}", &self.project)
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

    fn extract_url(&self, _: &reqwest::header::HeaderMap) -> String {
        unimplemented!()
    }
}
