# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- ✨ add error handling module(pr [#26])
- ✨ add error code mapping for captval API(pr [#28])
- ✨ add response structure for captval API(pr [#29])
- ✨ add domain modules and conditional feature support (pr [#30])
- ✨ add captcha struct for captcha data(pr [#33])

### Changed

- ci(circleci)-update workflows to include save_next_version job and dependencies(pr [#16])
- refactor-restructure project to use workspace and remove unused files(pr [#17])
- 👷 ci(circleci)-update circleci-toolkit orb version(pr [#24])
- 📝 docs(lib)-add comprehensive documentation for hcaptcha(pr [#25])
- ♻️ refactor(lib)-enable error module(pr [#27])
- Add-Captval-trait-for-validation(pr [#31])
- ♻️ refactor(hcaptcha)-clean up unused code and comments(pr [#32])

### Security

- Dependencies: update dependency toolkit to v2(pr [#15])
- Dependencies: update dependency toolkit to v2.0.1(pr [#18])
- Dependencies: update dependency toolkit to v2.0.4(pr [#19])
- Dependencies: update dependency toolkit to v2.0.7(pr [#20])
- Dependencies: update dependency toolkit to v2.0.8(pr [#21])
- Dependencies: update dependency toolkit to v2.0.13(pr [#22])

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
[#17]: https://github.com/jerus-org/captval/pull/17
[#18]: https://github.com/jerus-org/captval/pull/18
[#19]: https://github.com/jerus-org/captval/pull/19
[#20]: https://github.com/jerus-org/captval/pull/20
[#21]: https://github.com/jerus-org/captval/pull/21
[#22]: https://github.com/jerus-org/captval/pull/22
[#24]: https://github.com/jerus-org/captval/pull/24
[#25]: https://github.com/jerus-org/captval/pull/25
[#26]: https://github.com/jerus-org/captval/pull/26
[#27]: https://github.com/jerus-org/captval/pull/27
[#28]: https://github.com/jerus-org/captval/pull/28
[#29]: https://github.com/jerus-org/captval/pull/29
[#30]: https://github.com/jerus-org/captval/pull/30
[#30]: https://github.com/jerus-org/captval/pull/30
[#31]: https://github.com/jerus-org/captval/pull/31
[#32]: https://github.com/jerus-org/captval/pull/32
[#33]: https://github.com/jerus-org/captval/pull/33
[#33]: https://github.com/jerus-org/captval/pull/33
[Unreleased]: https://github.com/jerus-org/captval/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jerus-org/captval/releases/tag/v0.1.0
