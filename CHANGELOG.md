# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- 👷 ci(circleci)-remove github release job(pr [#60])
- 🔧 chore(config)-update renovate configuration for improved dependency management(pr [#61])
- 🔧 chore(vscode)-add sonarlint connected mode configuration(pr [#62])
- ♻️ refactor(tests)-update test directory naming for clarity(pr [#63])
- 💄 style(hcaptcha)-simplify string formatting in tests(pr [#72])

### Fixed

- deps: update rust crate reqwest to 0.12.20(pr [#65])
- deps: update rust crate mockd to 0.4.49(pr [#64])
- deps: update rust crate serde to 1.0.219(pr [#66])
- deps: update rust crate syn to 2.0.104(pr [#67])
- deps: update rust crate tokio to 1.45.1(pr [#68])
- deps: update rust crate wiremock to 0.6.4(pr [#69])
- deps: update rust crate lambda_runtime to 0.14.2(pr [#70])
- deps: update rust crate uuid to 1.17.0(pr [#71])

## [0.1.2] - 2025-05-15

### Added

- ✨ add error handling module(pr [#26])
- ✨ add error code mapping for captval API(pr [#28])
- ✨ add response structure for captval API(pr [#29])
- ✨ add domain modules and conditional feature support (pr [#30])
- ✨ add captcha struct for captcha data(pr [#33])
- ✨ add request module for captval API(pr [#34])
- ✨ add new Captval client implementation(pr [#35])
- ✨ add new captval_derive crate(pr [#40])
- ✨ add derive macro for Captval trait(pr [#42])
- ✨ add initial test-wasm crate(pr [#51])

### Changed

- ci(circleci)-update workflows to include save_next_version job and dependencies(pr [#16])
- refactor-restructure project to use workspace and remove unused files(pr [#17])
- 👷 ci(circleci)-update circleci-toolkit orb version(pr [#24])
- 📝 docs(lib)-add comprehensive documentation for hcaptcha(pr [#25])
- ♻️ refactor(lib)-enable error module(pr [#27])
- Add-Captval-trait-for-validation(pr [#31])
- ♻️ refactor(hcaptcha)-clean up unused code and comments(pr [#32])
- 📝 docs(README)-add initial README for captval library(pr [#36])
- 📝 docs(funding)-add FUNDING.yml configuration(pr [#37])
- 📝 docs(changelog)-add changelog symlinks for sub-crates(pr [#38])
- 📝 docs(license)-add Apache and MIT licenses(pr [#39])
- 🔧 chore(release)-add release configuration file(pr [#41])
- ✅ test(captval_derive)-add basic test cases for macro expansion(pr [#43])
- ♻️ refactor(captval_derive)-remove unused variable(pr [#44])
- ✅ test(expand)-add test for expanded contact form(pr [#45])
- ✅ test(compile_fail)-add tests for compile-time enum and struct validation(pr [#46])
- ✅ test(compiletest)-add compile test for ui scenarios(pr [#47])
- ♻️ refactor(client)-remove deprecated verify_client_response method(pr [#48])
- 📝 docs(lib)-update code example annotations(pr [#49])
- 👷 ci(circleci)-add test-features job to workflow(pr [#50])
- 🔧 chore(cargo)-update package metadata and configuration(pr [#52])
- ♻️ refactor(test-wasm)-rename struct field for clarity(pr [#53])
- 🔧 chore(release)-update release configuration(pr [#54])
- 👷 ci(circleci)-remove redundant jobs from release workflow(pr [#55])
- ♻️ refactor(release)-update release configuration for captval(pr [#56])
- 👷 ci(circleci)-update toolkit orb and refine release jobs(pr [#57])
- 👷 ci(circleci)-fix test command syntax in config(pr [#58])
- 📦 build(captval)-bump version to 0.1.1(pr [#59])

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
[#34]: https://github.com/jerus-org/captval/pull/34
[#35]: https://github.com/jerus-org/captval/pull/35
[#36]: https://github.com/jerus-org/captval/pull/36
[#37]: https://github.com/jerus-org/captval/pull/37
[#38]: https://github.com/jerus-org/captval/pull/38
[#39]: https://github.com/jerus-org/captval/pull/39
[#40]: https://github.com/jerus-org/captval/pull/40
[#41]: https://github.com/jerus-org/captval/pull/41
[#42]: https://github.com/jerus-org/captval/pull/42
[#43]: https://github.com/jerus-org/captval/pull/43
[#44]: https://github.com/jerus-org/captval/pull/44
[#45]: https://github.com/jerus-org/captval/pull/45
[#46]: https://github.com/jerus-org/captval/pull/46
[#47]: https://github.com/jerus-org/captval/pull/47
[#48]: https://github.com/jerus-org/captval/pull/48
[#49]: https://github.com/jerus-org/captval/pull/49
[#50]: https://github.com/jerus-org/captval/pull/50
[#51]: https://github.com/jerus-org/captval/pull/51
[#52]: https://github.com/jerus-org/captval/pull/52
[#53]: https://github.com/jerus-org/captval/pull/53
[#54]: https://github.com/jerus-org/captval/pull/54
[#55]: https://github.com/jerus-org/captval/pull/55
[#56]: https://github.com/jerus-org/captval/pull/56
[#57]: https://github.com/jerus-org/captval/pull/57
[#58]: https://github.com/jerus-org/captval/pull/58
[#59]: https://github.com/jerus-org/captval/pull/59
[#60]: https://github.com/jerus-org/captval/pull/60
[#61]: https://github.com/jerus-org/captval/pull/61
[#62]: https://github.com/jerus-org/captval/pull/62
[#63]: https://github.com/jerus-org/captval/pull/63
[#65]: https://github.com/jerus-org/captval/pull/65
[#64]: https://github.com/jerus-org/captval/pull/64
[#66]: https://github.com/jerus-org/captval/pull/66
[#67]: https://github.com/jerus-org/captval/pull/67
[#68]: https://github.com/jerus-org/captval/pull/68
[#69]: https://github.com/jerus-org/captval/pull/69
[#70]: https://github.com/jerus-org/captval/pull/70
[#71]: https://github.com/jerus-org/captval/pull/71
[#72]: https://github.com/jerus-org/captval/pull/72
[Unreleased]: https://github.com/jerus-org/captval/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/jerus-org/captval/compare/v0.1.0...v0.1.2
[0.1.0]: https://github.com/jerus-org/captval/releases/tag/v0.1.0
