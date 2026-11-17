// Integration test for SigmaOS-declarative-operating-system-configuration
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use declarative_operating_system_configuration as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
