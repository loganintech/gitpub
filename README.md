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
    github_repo_creator.exe [FLAGS] [OPTIONS] --name <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Prints http response data.

OPTIONS:
    -m, --merge <allow_merge_commit>        Enables or disables merging with a merge commit. Github defaults this to true.
    -r, --rebase <allow_rebase_merge>       Enables or disables rebase-merging for this repo. Github defaults this to true.
    -s, --squash <allow_squash_merge>       Enables or disables squash-merging for this repo. Github defaults this to true.
    -a, --auto-init <auto_init>             Creates an initial commit with empty README.md. Github defaults this to false.
    -d, --description <description>         A short description of the repository.
    -g, --gitignore <gitignore_template>    Language template: ex 'Rust'. View more templates at https://github.com/github/gitignore
    -i, --issues <has_issues>               Enable or disable issues for this repo. Github defaults this to true.
    -j, --projects <has_projects>           Enables or disables projects for this repo. Github defaults this to true.
    -w, --wiki <has_wiki>                   Enables or disables wikis for this repo. Github defaults this to true.
        --homepage <homepage>               A URL with more information about the repository.
    -l, --license <license_template>        License template: ex 'mit' or 'mpl-2.0'. View more at https://choosealicense.com/
    -n, --name <name>                       The name of the new repository.
    -o, --org <org>                         Creates the repo under an organization. Requires you have CREATE REPO permissions in that org.
    -p, --private <private>                 Requires 'repo' scope on your personal access token
    -t, --team <team_id>                    Id of the team that has access to this repo. Only valid when using --org.
```
