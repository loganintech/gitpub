use crate::provider::{
    azure::AzureArgs, bitbucket::BitbucketArgs, github::GithubArgs, gitlab::GitlabArgs, Provider,
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Git Publish",
    about = "A small program to create remote git repositories from the command line.",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
pub enum Gitpo {
    #[structopt(
        name = "github",
        about = "Create a repo on github.",
        raw(setting = "structopt::clap::AppSettings::ColoredHelp")
    )]
    Github(GithubArgs),
    #[structopt(
        name = "gitlab",
        about = "Create a repo on gitlab.",
        raw(setting = "structopt::clap::AppSettings::ColoredHelp")
    )]
    Gitlab(GitlabArgs),
    #[structopt(
        name = "bitbucket",
        about = "Create a repo on bitbucket.",
        raw(setting = "structopt::clap::AppSettings::ColoredHelp")
    )]
    BitBucket(BitbucketArgs),
    #[structopt(
        name = "azure",
        about = "Create a repo on azure.",
        raw(setting = "structopt::clap::AppSettings::ColoredHelp")
    )]
    Azure(AzureArgs),
}

// You're probably looking at this and thinking, logan, what are you doing.
// Well, the idea here is to allow the possibility for more complicated provider options in the future.
// We may not want every subcommand to be a provider
impl Provider for Gitpo {
    fn payload(&self) -> String {
        match self {
            Gitpo::Github(config) => config.payload(),
            Gitpo::Gitlab(config) => config.payload(),
            Gitpo::BitBucket(config) => config.payload(),
            Gitpo::Azure(config) => config.payload(),
        }
    }

    fn endpoint(&self) -> String {
        match self {
            Gitpo::Github(config) => config.endpoint(),
            Gitpo::Gitlab(config) => config.endpoint(),
            Gitpo::BitBucket(config) => config.endpoint(),
            Gitpo::Azure(config) => config.endpoint(),
        }
    }
    fn extract_url(&self, src: &reqwest::header::HeaderMap) -> String {
        match self {
            Gitpo::Github(config) => config.extract_url(src),
            Gitpo::Gitlab(config) => config.extract_url(src),
            Gitpo::BitBucket(config) => config.extract_url(src),
            Gitpo::Azure(config) => config.extract_url(src),
        }
    }
}
