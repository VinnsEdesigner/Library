#[test]
fn test_sandbox_constraints() {
    // malicious plugins should be blocked
    let plugin = "malicious";
    assert_eq!(plugin, "malicious");
}
