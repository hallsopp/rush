use anyhow::Result;

use crate::config::Configuration;
use crate::environment::Environment;

// Represents the shell, its state, and provides methods for interacting with it
// ? Should this be called ShellState or something like that?
// TODO: Miscellaneous shell state like command_success, command_history etc might be better off in some sort of bundle struct
pub struct Shell {
    pub(crate) environment: Environment,
    pub(crate) config: Configuration,
    pub(crate) command_success: bool,
    pub(crate) command_history: Vec<String>,
}

impl Shell {
    pub fn new() -> Result<Self> {
        let config =
            Configuration::from_file("config/config.rush").unwrap_or(Configuration::default());

        Ok(Self {
            environment: Environment::new()?,
            config,
            command_success: true,
            command_history: Vec::new(),
        })
    }
}
