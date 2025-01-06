# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- ci(circleci)-update workflows to include save_next_version job and dependencies(pr [#16])

### Security

- Dependencies: update dependency toolkit to v2(pr [#15])

## [0.1.0] - 2024-12-27

### Changed

- refactor-move add function to lib.rs and remove main.rs(pr [#3])
- chore-add CODEOWNERS file to define code ownership(pr [#6])
- docs-add metadata to Cargo.toml including description, license, documentation, and repository links(pr [#7])
- chore-add release configuration file with signing and branch restrictions(pr [#8])
- Create FUNDING.yml(pr [#9])
- refactor(circleci)-remove custom commands and jobs in favour of toolkit orbs(pr [#10])
- chore(circleci)-update toolkit orb to version 1.23.0(pr [#12])
- ci(circleci)-add when_cargo_release parameter to config file(pr [#13])
- chore-add first-release configuration file for release process(pr [#14])

### Fixed

- circleci: correct publish flag in config file(pr [#11])

### Security

- Dependencies: update dependency toolkit to v1.23.0(pr [#4])

[#3]: https://github.com/jerus-org/captval/pull/3
[#6]: https://github.com/jerus-org/captval/pull/6
[#7]: https://github.com/jerus-org/captval/pull/7
[#8]: https://github.com/jerus-org/captval/pull/8
[#9]: https://github.com/jerus-org/captval/pull/9
[#10]: https://github.com/jerus-org/captval/pull/10
[#11]: https://github.com/jerus-org/captval/pull/11
[#4]: https://github.com/jerus-org/captval/pull/4
[#13]: https://github.com/jerus-org/captval/pull/13
[#14]: https://github.com/jerus-org/captval/pull/14
[#15]: https://github.com/jerus-org/captval/pull/15
[#16]: https://github.com/jerus-org/captval/pull/16
[Unreleased]: https://github.com/jerus-org/captval/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jerus-org/captval/releases/tag/v0.1.0
