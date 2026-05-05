//! Integration smoke test for the rust-fake-consumer fixture.

use rust_fake_consumer::ping;

#[test]
fn integration_ping() {
    assert_eq!(ping(), "ok");
}
