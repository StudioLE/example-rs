# example-rs

Exemplary template for Rust CLI projects.

## Highlights

- `studiole-di` service container
- `studiole-report` error rendering
- `usage-rs` CLI parsing

## Install

Install the latest version with homebrew:

```shell
brew install studiole/tap/example-rs
```

Or download the binary from [GitHub Releases](https://github.com/StudioLE/example-rs/releases).

## Structure

Cargo workspace with single crate:

- `crates/example/src/app/` - CLI bootstrap, options, subcommand dispatch, DI registration
- `crates/example/src/commands/` - subcommand handlers (`start`, `stop`)
- `crates/example/src/services/` - shared services (`StatusProvider` trait + adapter)

## Stack

- Rust nightly, edition 2024
- `studiole-di` for dependency injection (local path: `../studiole-di`)
- `studiole-report` for structured error rendering
- `usage-rs` (derive) for CLI parsing
- `thiserror` for error enums
- `mockall` + `insta` for testing

> [!NOTE]
> The template switched from `clap` to `usage-rs`. The last `clap` version is [`345be5a`](https://github.com/StudioLE/example-rs/tree/345be5a89ef4f2cd641082fc3d7570f596e36084).

## CI/CD

Reusable workflow: `StudioLE/Actions/.github/workflows/ci-cd-rust.yml@v7`

Publishes to Homebrew (`StudioLE/homebrew-tap`) on release.
