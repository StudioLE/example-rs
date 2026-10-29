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
- `crates/example/src/commands/` - subcommand handlers (`start`, `status`, `stop`)
- `crates/example/src/services/` - shared services (status traits, file adapter, output formats)

## Patterns

Each pattern has one example, relative to `crates/example/src/`:

| Pattern                                    | Example                                                |
| ------------------------------------------ | ------------------------------------------------------ |
| Register one adapter for two traits        | `app/service_builder_ext.rs`                           |
| Run setup on startup with `Init`           | `services/status/status_file_adapter.rs`               |
| Hand-written `FromServices` with an error  | `app/options.rs`                                       |
| Swap in test doubles for every test        | `with_test_services()` in `app/service_builder_ext.rs` |
| Attach the path to a leaf I/O error        | `services/status/status_file_adapter.rs`               |
| Validate a newtype with `FromStr`          | `services/status/id.rs`                                |
| Global flag with an env var fallback       | `--state-dir` in `app/cli_args.rs`                     |
| Select an output format with `--format`    | `app/cli_args.rs`                                      |
| Render any value as text, JSON, TOML, YAML | `services/output/output_renderer.rs`                   |
| Pick an option in a test with `CliArgs`    | `services/output/output_renderer.rs`                   |
| Inject a writer for stdout                 | `services/console/console_writer.rs`                   |
| Stub with canned answers                   | `services/status/status_provider_stub.rs`              |
| Spy that records calls                     | `services/status/status_writer_spy.rs`                 |
| Fake with an in-memory store               | `services/status/status_fake.rs`                       |
| Fixtures and alternate fixtures            | `StatusFake::fixture_active()`                         |
| Test against a temp dir                    | `services/status/status_file_adapter.rs`               |
| Snapshot test with `insta`                 | `app/cli_args.rs`                                      |

## Stack

- Rust nightly, edition 2024
- `studiole-di` for dependency injection (local path: `../studiole-di`)
- `studiole-report` for structured error rendering
- `usage-rs` (derive) for CLI parsing
- `etcetera` for the platform state dir
- `serde_json`, `serde_yaml`, and `toml` for output formats
- `thiserror` for error enums
- `insta` and `tempfile` for testing

> [!NOTE]
> The template switched from `clap` to `usage-rs`. The last `clap` version is [`345be5a`](https://github.com/StudioLE/example-rs/tree/345be5a89ef4f2cd641082fc3d7570f596e36084).

## CI/CD

Reusable workflow: `StudioLE/Actions/.github/workflows/ci-cd-rust.yml@v7`

Publishes to Homebrew (`StudioLE/homebrew-tap`) on release.
