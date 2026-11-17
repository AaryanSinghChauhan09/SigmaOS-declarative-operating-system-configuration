//! Integration tests for SigmaOS-declarative-operating-system-configuration.

use sigmaos_declarative_operating_system_configuration_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
