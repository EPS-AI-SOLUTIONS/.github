# Contributing

All commits MUST follow [Conventional Commits](https://www.conventionalcommits.org):
`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `test:`, `perf:`, `ci:`

Release cadence is automated via release-please-action@v4 — your conventional commits drive version bumps and CHANGELOG.md updates.

## Local Development

- Rust crates: `cargo nextest run` (NOT `cargo test`)
- TS packages: `bun test`
- Cross-package dev: use `[patch.kellnr]` (Cargo) or `bun link` (npm) — never git submodules
