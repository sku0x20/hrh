pub mod vm_agent {
    use std::path::Path;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct VmAgentConfig {
        pub name: String,
        pub repo: String,
        pub release_name: String,
        pub namespace: String,
        pub values_file: String,
    }

    #[derive(Debug)]
    pub struct ReadConfigError;

    impl std::fmt::Display for ReadConfigError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "read config error")
        }
    }

    impl std::error::Error for ReadConfigError {}

    // Stub implementation per TDD: returns an empty/default config for now.
    pub fn read_vm_agent_config(_path: &Path) -> Result<VmAgentConfig, ReadConfigError> {
        Ok(VmAgentConfig::default())
    }
}
