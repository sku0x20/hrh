// Basic integration test to ensure the tests directory is set up correctly

// TDD: Add a test that provides vm-agent.yaml and verifies it is read correctly.
// The current implementation is a stub that returns an empty/default response,
// so this test is expected to fail until the implementation is provided.
#[test]
fn reads_vm_agent_yaml_correctly() {
    use hrh::vm_agent::{read_vm_agent_config, VmAgentConfig};
    let path = std::path::Path::new("tests").join("resources").join("vm-agent.yaml");

    let cfg = read_vm_agent_config(&path).expect("should read vm-agent.yaml");

    let expected = VmAgentConfig {
        name: "victoria-metrics-agent".to_string(),
        repo: "https://victoriametrics.github.io/helm-charts".to_string(),
        release_name: "pod-collector".to_string(),
        namespace: "observability".to_string(),
        values_file: "pod-collector.yaml".to_string(),
    };

    assert_eq!(cfg, expected);
}
