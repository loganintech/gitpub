# Gitpub

A small program to create remote git repositories from the command line.

Usage:

```
Git Publish 0.2.1
Logan Saso <logansaso+tech@gmail.com>
A small program to create remote git repositories from the command line.

USAGE:
    gitpub <SUBCOMMAND>

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
gitpub-github 0.2.1
Logan Saso <logansaso+tech@gmail.com>
Create a repo on github.

USAGE:
    gitpub github [OPTIONS] --name <name> --token <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --merge <allow_merge_commit>            Enables or disables merging with a merge commit. Defaults to true.
        --rebase <allow_rebase_merge>           Enables or disables rebase-merging for this repo. Defaults to true.
        --squash <allow_squash_merge>           Enables or disables squash-merging for this repo. Defaults to true.
    -r, --initialize_with_readme <auto_init>    Creates an initial commit with empty README.md. Defaults to false.
    -e, --endpoint <custom_endpoint>            Custom endpoint to enable uploading to private repos.
    -d, --description <description>             A short description of the repository.
        --gitignore <gitignore_template>        Language template: ex 'Rust'. View more templates at
                                                https://github.com/github/gitignore
    -i, --issues <has_issues>                   Enable or disable issues for this repo. Defaults to true.
        --projects <has_projects>               Enables or disables projects for this repo. Defaults to true.
    -w, --wiki <has_wiki>                       Enables or disables wikis for this repo. Defaults to true.
        --homepage <homepage>                   A URL with more information about the repository.
        --license <license_template>            License template: ex 'mit' or 'mpl-2.0'. View more at
                                                https://choosealicense.com/
    -n, --name <name>                           The name of the new repository.
        --org <org>                             Creates the repo under an organization. Requires you have CREATE REPO
                                                permissions in that org.
    -p, --private <private>                     Requires 'repo' scope on your personal access token
        --team <team_id>                        Id of the team that has access to this repo. Only valid when using
                                                --org.
    -t, --token <token>                         A personal access token. Alternatively read from GITHUB_REPO_TOKEN env
                                                variable [env: GITHUB_REPO_TOKEN=<YOUR_TOKEN]
```

## Gitlab Setup

1. Create a personal access token with api scope from https://gitlab.com/profile/personal_access_tokens
2. Set the environment variable `GITLAB_REPO_TOKEN` to the generated personal access token.

_Note_: Optionally set the `GITLAB_USERNAME` environment variable to enable printing a link to the repo.


### Gitlab
```
gitpub-gitlab 0.2.1
Logan Saso <logansaso+tech@gmail.com>
Create a repo on gitlab.

USAGE:
    gitpub gitlab [OPTIONS] --name <name> --path <path> --token <token>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
        --approvals_before_merge <approvals_before_merge>
            Number of approvals required before a merge.

        --ci_config_path <ci_config_path>
            Path to CI config file.

        --container_registry <container_registry_enabled>
            Enables or disables container registry for this repo.

        --default_branch <default_branch>
            Set default branch. Defaults to master.

    -e, --endpoint <custom_endpoint>
            Allows redirection of requests to enterprise providers.

    -d, --description <description>
            A short description of the repository.

        --import_url <import_url>
            Supply a URL to import a repo from.

    -r, --initialize_with_readme <initialize_with_readme>
            Create a blank README in the project.

    -i, --issues <issues_enabled>
            Enable or disable issues for this repo. Defaults to true.

        --jobs <jobs_enabled>
            Enables or disables jobs for this repo.

        --lfs <lfs_enabled>
            Enables git large file support.

        --merge_method <merge_method>
            Sets the merge method.

        --merge <merge_requests_enabled>
            Enables or disables merge requests. Defaults to true.

        --mirror <mirror>
            Enables pull mirroring in a project.

        --mirror_trigger_builds <mirror_trigger_builds>
            Enables builds on pull mirroring.

    -n, --name <name>
            The name of the new project. Equals path if not provided.

        --namespace_id <namespace_id>
            Namespace for the new project (defaults to the current user’s namespace)

        --discussion_resolution_required <only_allow_merge_if_all_discussions_are_resolved>
            Requires discussions are resolved before a merge.

        --pipeline_success_required <only_allow_merge_if_pipeline_succeeds>
            Requires pipelines to succeed before a merge.

        --path <path>
            Repository name for new project. Generated based on name if not provided (generated lowercased with dashes).

        --merge_request_link_on_commit <printing_merge_request_link_enabled>
            Print the merge request link when committing to the repository.

        --public_builds <public_builds>
            Lets jobs be viewed by non-project members.

        --repository_storage <repository_storage>
            Which storage shard the repo is one. Available only to admins.

        --request_access_enabled <request_access_enabled>
            Allow users to request member access.

        --shared_runners <shared_runners_enabled>
            Enables or disables shared runners for this repo.

        --snippets <snippets_enabled>
            Enables or disables snippets for this repo.

        --tag_list <tag_list>...
            A list of tags for a repo.

    -t, --token <token>
            A personal access token. Alternatively read from GITLAB_REPO_TOKEN env variable [env: GITLAB_REPO_TOKEN=<YOUR_TOKEN>]

    -p, --visibility <visibility>
            The visibility of the project. [possible values: public, internal, private]

    -w, --wiki <wiki_enabled>
            Enables or disables wikis for this repo. Defaults to true.
```

