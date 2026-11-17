//! Integration tests for SigmaOS-boot-environment-management.

use sigmaos_boot_environment_management_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
