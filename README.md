# Gitpo

A small program to automagically create github repositories.

## Setup

1. Create a personal access token with repo scope from https://github.com/settings/tokens
2. Set the environment variable `GITHUB_REPO_TOKEN` to the generated personal access token.

_Note:_ If you want to create org repositories the token also requires `org` scope.

Usage:

```
Github Repo Creator 0.2.0
Logan Saso <logansaso@gmail.com>
A small program to create github repos from the command line.

USAGE:
    github_repo_creator.exe [OPTIONS] --name <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --merge <allow_merge_commit>
    -r, --rebase <allow_rebase_merge>
    -s, --squash <allow_squash_merge>
    -a, --auto-init <auto_init>
    -d, --description <description>
    -g, --gitignore <gitignore_template>    Language template: ex 'Rust'
    -i, --issues <has_issues>
    -j, --projects <has_projects>
    -w, --wiki <has_wiki>
        --homepage <homepage>
    -l, --license <license_template>
    -n, --name <name>
    -o, --org <org>                         Creates the repo under an organization. Requires you have CREATE REPO permissions in that org.
    -p, --private <private>                 Requires 'repo' scope on your personal access token
    -t, --team <team_id>
```
