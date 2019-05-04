use crate::provider::{bitbucket::BitbucketArgs, github::GithubArgs, gitlab::GitlabArgs, Provider};
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
}

// There's a time and a place for that sweet, sweet dynamic dispatch
impl Gitpo {
    pub fn as_provider(&self) -> &dyn Provider {
        match self {
            Gitpo::Github(x) => x as &Provider,
            Gitpo::Gitlab(x) => x as &Provider,
            Gitpo::BitBucket(x) => x as &Provider,
        }
    }
}
