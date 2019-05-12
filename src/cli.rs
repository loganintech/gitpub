use crate::provider::{
    bitbucket::{self, BitbucketArgs},
    github::{self, GithubArgs},
    gitlab::{self, GitlabArgs},
    Provider,
};
use clap::{App, AppSettings, Arg, ArgMatches};

/// Enumerates the subcommand provider options available from subcommands.
pub enum Gitpo<'a> {
    Github(GithubArgs<'a>),
    Gitlab(GitlabArgs<'a>),
    BitBucket(BitbucketArgs<'a>),
}

impl<'a> Gitpo<'a> {
    /// from_matches constructs a subcommand config from the selected subcommand.
    pub fn from_matches(matches: &'a ArgMatches) -> Gitpo<'a> {
        match matches.subcommand_name() {
            Some("github") => Gitpo::Github(github::from_matches(
                &matches.subcommand_matches("github").unwrap(),
            )),
            Some("gitlab") => Gitpo::Gitlab(gitlab::from_matches(
                &matches.subcommand_matches("gitlab").unwrap(),
            )),
            Some("bitbucket") => Gitpo::BitBucket(bitbucket::from_matches(
                &matches.subcommand_matches("bitbucket").unwrap(),
            )),
            _ => unreachable!(),
        }
    }

    /// as_provider
    ///
    /// Returns self as a reference to a Provider
    pub fn as_provider(&self) -> &dyn Provider {
        match self {
            Gitpo::Github(x) => x as &Provider,
            Gitpo::Gitlab(x) => x as &Provider,
            Gitpo::BitBucket(x) => x as &Provider,
        }
    }
}

/// ## Returns highest-level clap app.
pub fn get_app() -> App<'static, 'static> {
    App::new("Git Publish")
        .global_setting(AppSettings::ColorAuto)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .bin_name("gitpub")
        .author("Logan Saso <logansaso+tech@gmail.com>")
        .about("A small program to create remote git repositories from the command line.")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("endpoint")
                .long("endpoint")
                .takes_value(true),
        )
        .subcommand(github::subcommand())
        .subcommand(gitlab::subcommand())
        .subcommand(bitbucket::subcommand())
}
