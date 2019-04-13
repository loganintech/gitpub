# Gitpub

A small program to automagically create github repositories.

Usage:

```
Git Publish 0.2.0
Logan Saso <logansaso@gmail.com>
A small program to create remote github repositories on the command line.

USAGE:
    github_repo_creator <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    github    Create a repo on github.
    gitlab    Create a repo on gitlab.
    help      Prints this message or the help of the given subcommand(s)

```

## Github Setup

1. Create a personal access token with repo scope from https://github.com/settings/tokens
2. Set the environment variable `GITHUB_REPO_TOKEN` to the generated personal access token.

_Note:_ If you want to create org repositories the token also requires `org` scope.

### Github
```
github_repo_creator-github 0.2.0
Logan Saso <logansaso@gmail.com>
Create a repo on github.

USAGE:
    github_repo_creator github [OPTIONS] --name <name> --auth <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --merge <allow_merge_commit>        Enables or disables merging with a merge commit. Defaults to true.
    -r, --rebase <allow_rebase_merge>       Enables or disables rebase-merging for this repo. Defaults to true.
    -s, --squash <allow_squash_merge>       Enables or disables squash-merging for this repo. Defaults to true.
    -a, --auto-init <auto_init>             Creates an initial commit with empty README.md. Defaults to false.
    -d, --description <description>         A short description of the repository.
    -g, --gitignore <gitignore_template>    Language template: ex 'Rust'. View more templates at https://github.com/github/gitignore
    -i, --issues <has_issues>               Enable or disable issues for this repo. Defaults to true.
    -j, --projects <has_projects>           Enables or disables projects for this repo. Defaults to true.
    -w, --wiki <has_wiki>                   Enables or disables wikis for this repo. Defaults to true.
        --homepage <homepage>               A URL with more information about the repository.
    -l, --license <license_template>        License template: ex 'mit' or 'mpl-2.0'. View more at https://choosealicense.com/
    -n, --name <name>                       The name of the new repository.
    -o, --org <org>                         Creates the repo under an organization. Requires you have CREATE REPO permissions in that org.
    -p, --private <private>                 Requires 'repo' scope on your personal access token
    -t, --team <team_id>                    Id of the team that has access to this repo. Only valid when using --org.
    -k, --auth <token>                      A personal access token. Alternatively read from GITHUB_REPO_TOKEN env variable [env: GITHUB_REPO_TOKEN=<YOUR_TOKEN>]
```

## Gitlab Setup

1. Create a personal access token with api scope from https://gitlab.com/profile/personal_access_tokens
2. Set the environment variable `GITLAB_REPO_TOKEN` to the generated personal access token.


### Gitlab
```
github_repo_creator-gitlab 0.2.0
Logan Saso <logansaso@gmail.com>
Create a repo on gitlab.

USAGE:
    github_repo_creator gitlab [OPTIONS] --name <name> --path <path> --auth <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --container_registry <container_registry_enabled>    Enables or disables container registry for this repo.
        --default_branch <default_branch>                    Set default branch. Defaults to master.
    -d, --description <description>                          A short description of the repository.
        --import_url <import_url>                            Supply a URL to import a repo from.
    -i, --issues <issues_enabled>                            Enable or disable issues for this repo. Defaults to true.
        --jobs <jobs_enabled>                                Enables or disables jobs for this repo.
    -m, --merge <merge_requests_enabled>                     Enables or disables merge requests. Defaults to true.
    -n, --name <name>                                        The name of the new project. Equals path if not provided.
        --namespace_id <namespace_id>                        Namespace for the new project (defaults to the current user’s namespace)
    -p, --path <path>                                        Repository name for new project. Generated based on name if not provided (generated lowercased with dashes).
        --shared_runners <shared_runners_enabled>            Enables or disables shared runners for this repo.
        --snippets <snippets_enabled>                        Enables or disables snippets for this repo.
    -k, --auth <token>                                       A personal access token. Alternatively read from GITLAB_REPO_TOKEN env variable [env: GITLAB_REPO_TOKEN=<YOUR_TOKEN>]
        --visibility <visibility>                            The visibility of the project `public, internal, private` [possible values: public, internal, private]
    -w, --wiki <wiki_enabled>                                Enables or disables wikis for this repo. Defaults to true.
```

