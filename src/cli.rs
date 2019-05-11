use crate::provider::{
    bitbucket::{self, BitbucketArgs},
    github::{self, GithubArgs},
    gitlab::{self, GitlabArgs},
    Provider,
};
use clap::{App, AppSettings, Arg, ArgMatches};

pub enum Gitpo<'a> {
    Github(GithubArgs<'a>),
    Gitlab(GitlabArgs<'a>),
    BitBucket(BitbucketArgs<'a>),
}

impl<'a> Gitpo<'a> {
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

    pub fn as_provider(&self) -> &dyn Provider {
        match self {
            Gitpo::Github(x) => x as &Provider,
            Gitpo::Gitlab(x) => x as &Provider,
            Gitpo::BitBucket(x) => x as &Provider,
        }
    }
}

pub fn get_app() -> App<'static, 'static> {
    App::new("Git Publish")
        .global_setting(AppSettings::ColorAuto)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .bin_name("gitpub")
        .author("Logan Saso <logansaso+tech@gmail.com>")
        .about("A small program to create remote git repositories from the command line.")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(github::subcommand())
        .subcommand(gitlab::subcommand())
        .subcommand(bitbucket::subcommand())
        .arg(
            Arg::with_name("endpoint")
                .long("endpoint")
                .takes_value(true)
                .help("Sets a custom endpoint to POST to, useful if you want a private instance and know the api matches one gitpub supports."),
        )
        .arg(
            Arg::with_name("set_remote")
                .long("set_remote")
                .help("Sets the remote of the local dir after successful creation.")
        ).arg(
            Arg::with_name("remote_name")
                .long("remote_name")
                .help("Designates a custom name for setting remote. Defaults to origin.")
                .default_value("origin")
                .hide_default_value(true)
        ).arg(
            Arg::with_name("ssh_remote_format")
                .long("ssh_remote_format")
                .help("Attempts to convert the git remote url into ssh format. If it fails (the provider doesn't support ssh format), the remote isn't set.")
        )
}
