//! Trivial fake consumer crate used by `.github/workflows/test-reusables.yml`
//! to smoke-test the org-wide reusable workflows (rust-test, rust-publish).
//!
//! Keep this file minimal and `cargo fmt`-clean — it doubles as the regression
//! tripwire: any drift in clippy lints or rustfmt defaults breaks this build.

/// Returns the literal string `"ok"`. Exists purely so the smoke test has a
/// public symbol with a doctest to exercise `cargo test --doc`.
///
/// ```
/// assert_eq!(rust_fake_consumer::ping(), "ok");
/// ```
pub fn ping() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_returns_ok() {
        assert_eq!(ping(), "ok");
    }
}
