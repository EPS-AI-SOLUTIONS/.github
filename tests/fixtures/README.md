# Reusable workflow smoke fixtures

These minimal fake-consumer projects are exercised weekly by
`.github/workflows/test-reusables.yml` to catch regressions in the org-wide
reusable workflows (`rust-test.yml`, `ts-test.yml`, `docker-publish.yml`,
`rust-publish.yml`, `npm-publish.yml`) before they hit real consumer repos.

## Layout

- `rust-fake-consumer/` — minimal `Cargo.toml` + `src/lib.rs` + integration test
  + `deny.toml`. Trivial public symbol with a doctest so `cargo test --doc`
  has something to bite. Edition 2024 to match the org default.
- `ts-fake-consumer/` — minimal `package.json` + `biome.json` + `tsconfig.json`
  + `src/index.ts` + `src/index.test.ts`. `test:coverage` script wired to
  vitest as `ts-test.yml` expects (B1237-T18 fix: vitest invoked via npm
  script, not Bun native runner).
- `ts-package-fake/` — same shape as `ts-fake-consumer` plus `publishConfig`
  pointing at `npm.pkg.github.com`. Used only by the publish-flavoured
  smoke (currently `if: false`-gated; covered by template-validation runs
  in template repos).

## Limitation (TODO B1237-FU)

The reusable workflows currently checkout the caller repo at root (no
`crate-path` / `package-path` input). Calling them from this `.github`
repo against `tests/fixtures/<name>/` requires either:

1. Adding a `working-directory` / `path` input to each reusable, or
2. Having `test-reusables.yml` symlink/copy the fixture content to the
   repo root before invoking the reusable.

Until that lands, the smoke workflow exercises the reusables via
`uses: ./.github/workflows/<name>.yml` from this repo's root context;
the fixture trees are reserved for the eventual path-input upgrade.

The publish-flavoured smokes (`rust-publish`, `npm-publish`) are gated
with `if: false` because they would push real artefacts; their regression
coverage lives in template-validation workflows inside the
`rust-crate-template`, `rust-app-template`, `ts-package-template`,
`ts-app-template` repos.
