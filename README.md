# Gitpub

A small program to create remote git repositories from the command line.

_Note:_ Environment variables can also be passed directly via CLI parameters, therefore setting said variables are always optional.

Usage:

```
Git Publish 0.4.0
Logan Saso <logansaso+tech@gmail.com>
A small program to create remote git repositories from the command line.

USAGE:
    gitpub [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help                 Prints help information
        --set_remote           Sets the remote of the local dir after successful creation.
        --ssh_remote_format    Attempts to convert the git remote url into ssh format. If it fails (the provider doesn't support ssh format), the remote isn't set.
    -V, --version              Prints version information

OPTIONS:
        --endpoint <endpoint>          Sets a custom endpoint to POST to, useful if you want a private instance and know the api matches one gitpub supports.
        --remote_name <remote_name>    Designates a custom name for setting remote. Defaults to origin.

SUBCOMMANDS:
    bitbucket    Create a repo on bitbucket.
    github       Create a repo on github.
    gitlab       Create a repo on gitlab.
    help         Prints this message or the help of the given subcommand(s)
```

## Github Setup

1. Create a personal access token with repo scope from https://github.com/settings/tokens
2. Set the environment variable `GITHUB_REPO_TOKEN` to the generated personal access token.

_Note:_ If you want to create org repositories the token also requires `org` scope.

### Github
```
gitpub-github 0.4.0
Create a repo on github.

USAGE:
    gitpub github [FLAGS] [OPTIONS] --name <name> --token <token>

FLAGS:
    -i, --disable_issues            Disables issues for this repo. Defaults to false.
        --merge                     Enables or disables merging with a merge commit. Defaults to false.
        --disable_projects          Disables projects for this repo. Defaults to false.
        --rebase                    Enables or disables rebase-merging for this repo. Defaults to false.
        --squash                    Enables or disables squash-merging for this repo. Defaults to false.
    -w, --disable_wiki              Disables wikis for this repo. Defaults to false.
    -h, --help                      Prints help information
        --org                       Creates the repo under an organization. Requires you have CREATE REPO permissions in that org.
    -p, --private                   Sets the repository to private. Required 'repo' scope on your personal access token.
    -r, --initialize_with_readme    Creates an initial commit with empty README.md. Defaults to false.
        --team                      ID of the team that has access to this repo. Only valid when using --org
    -V, --version                   Prints version information

OPTIONS:
    -d, --description <description>    A short description of the repository.
        --gitignore <gitignore>        Language template: ex 'Rust'. View more templates at https://github.com/github/gitignore/
        --homepage <homepage>          A URL with more information about the repository.
        --license <license>            License template: ex 'mit' or 'mpl-2.0'. View more at https://choosealicense.com/
    -n, --name <name>                  The name of the new repository.
    -t, --token <token>                A personal access token. Alternatively read from GITHUB_REPO_TOKEN env variable. [env: GITHUB_REPO_TOKEN=<YOUR_TOKEN>]

```

## Gitlab Setup

1. Create a personal access token with api scope from https://gitlab.com/profile/personal_access_tokens
2. Set the environment variable `GITLAB_REPO_TOKEN` to the generated personal access token.

_Note_: Optionally set the `GITLAB_USERNAME` environment variable to enable printing a link to the repo.


### Gitlab
```
gitpub-gitlab 0.4.0
Create a repo on gitlab.

USAGE:
    gitpub gitlab [FLAGS] [OPTIONS] --name <name> --path <path> --token <token>

FLAGS:
        --disable_container_registry        Disables container registry for this repo.
    -i, --disable_issues                    Disables issues for this repo.
        --disable_jobs                      Disables jobs for this repo.
        --disable_merge                     Disables merging with a merge commit.
        --disable_shared_runners            Disables shared runners for this repo.
        --disable_snippets                  Disables snippets for this repo.
    -w, --disable_wiki                      Disables wikis for this repo.
        --discussion_resolution_required    Requires discussions are resolved before a merge.
    -h, --help                              Prints help information
    -r, --initialize_with_readme            Create a blank README for the project.
        --large_file_support                Enables git large file support.
        --merge_request_link_on_commit      Print the merge request link when committing to the repository.
        --mirror                            Enables pull mirroring in a project.
        --mirror_triggers_builds            Enables builds on pull mirroring.
        --pipeline_success_required         Requires pipelines to succeed before a merge.
        --public_builds                     Lets jobs be viewed by non-project members.
        --request_access_enabled            Allow users to request member access.
    -V, --version                           Prints version information

OPTIONS:
        --approvals_before_merge <approvals_before_merge>    Number of approvals required before a merge.
        --ci_config_path <ci_config_path>                    Path to CI config file.
        --default_branch <default_branch>                    Set default branch. Defaults to master.
    -d, --description <description>                          A short description of the repository.
        --import_url <import_url>                            Supply a URL to import a repo from.
        --merge_method <merge_method>                        Sets the merge method. [possible values: merge, rebase_merge, ff]
    -n, --name <name>                                        The name of the new repository.
        --namespace_id <namespace_id>                        Namespace for the new project (defaults to the current user’s namespace)
        --path <path>                                        The path for the new repo. Generated based on name if not provided (generated lowercased with dashes).
        --repository_storage <repository_storage>            An advanced parameter to designate which shard the repo is on, available only to admins.
        --tag_list <tag_list>...                             A list of tags for a repo. Takes up to four params. `--tag_list first second third tag`
    -t, --token <token>                                      A personal access token. Alternatively read from GITLAB_REPO_TOKEN env variable. [env: GITLAB_REPO_TOKEN=<YOUR_TOKEN>]
        --visibility <visibility>                            The visibility of the project [possible values: public, internal, private]
```

## BitBucket Setup

1. Create an app password with repo scope from https://bitbucket.org/account/user/<YOUR_USERNAME>/app-passwords
2. Set the environment variable `BITBUCKET_REPO_TOKEN` to the generated personal access token.
3. Set the environment variable `BITBUCKET_USERNAME` to your bitbucket username.

### Bitbucket

```
gitpub-bitbucket 0.4.0
Create a repo on bitbucket.

USAGE:
    gitpub bitbucket [FLAGS] [OPTIONS] --name <name> --token <token> --username <username>

FLAGS:
    -i, --disable_issues    Disables issues for this repo. Defaults to false.
    -w, --disable_wiki      Disables wikis for this repo. Defaults to false.
    -h, --help              Prints help information
        --language          Give bitbucket a hint about the programming language.
    -p, --private           Sets the repository to private.
    -V, --version           Prints version information

OPTIONS:
    -d, --description <description>    A short description of the repository.
        --fork_policy <fork_policy>    Changes the allowed forking method for this repo. [possible values: allow_forks, no_public_forks, no_forks]
    -n, --name <name>                  The name of the new repository.
        --scm <scm>                    Control underlying source control method. [possible values: git, hg]
    -t, --token <token>                A personal access token. Alternatively read from BITBUCKET_REPO_TOKEN env variable. [env: BITBUCKET_REPO_TOKEN=<YOUR_TOKEN>]
        --username <username>          Your bitbucket username. Alternatively read from BITBUCKET_USERNAME env variable. [env: BITBUCKET_USERNAME=<YOUR_USERNAME>]
```
