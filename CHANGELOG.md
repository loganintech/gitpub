# Changelog

## [0.4.1] - 2019-05-11
### Removed

* Removed `--endpoint` from gitlab subcommand. It is taken care of via a global param now.

## [0.4.0] - 2019-05-11
### Added
* Added `--ssh_remote_format`, `--set_remote`, and `--remote_name` commands. These are global commands and must be used before the subcommand. Example: `gitpub --ssh_remote_format github -n "name"` Check README for more info.

### Changed

* This version encompasses changes to the CLI interface. It was re-written from structopt to use a clap app.
* Any flag that defaults to true and accepted a `true` or `false` value has now been inverted. IE `--issues false` has become `--disable_issues`. `--issues true` is no longer accept as it doesn't effect the result.
* The `--endpoint` flag is now global and allows you to selectively override the endpoint that the request is sent to.

## [0.3.0] - 2019-04-22
### Added

* Initial bitbucket support using app base64-encoded username:apppasword flow. [Read more about app passwords](https://confluence.atlassian.com/bitbucket/app-passwords-828781300.html)

### Changed

* Slightly changed helptext for github privacy flag.
* Github parameters are no longer serialized if they're `None`.

## [0.2.2] - 2019-04-14
### Added

* Custom --endpoint (-e) param allowing request redirection to enterprise providers

## [0.2.1] - 2019-04-14
### Changed

* Modified Cargo manifest entry for the license

## [0.2.0] - 2019-04-14
### Added

* Published to crates.io: https://crates.io/crates/gitpub
* Gitlab support

### Changed

* Short versions of shared properties

## [0.1.0] - 2019-04-10

### Added

* Github support
