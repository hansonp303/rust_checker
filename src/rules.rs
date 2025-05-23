/// Holds configuration flags for validation rules
#[derive(Default)]
pub struct RuleConfig {
    pub check_main: bool,
    pub check_unused_var: bool,
    pub check_unused_import: bool,
}

impl RuleConfig {
    /// Construct RuleConfig from command-line args
    pub fn from_args(args: &[String]) -> Self {
        RuleConfig {
            check_main: !args.contains(&"--skip-main".to_string()),
            check_unused_var: !args.contains(&"--allow-unused-var".to_string()),
            check_unused_import: !args.contains(&"--allow-unused-import".to_string()),
        }
    }
}

