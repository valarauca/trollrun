
use std::process::Command;

use super::runs::{TrollRun};

/// SetArgs handles settingup a specific troll run
pub trait SetArgs {
    fn set_args(&self, cmd: &mut Command);
}

/// BuildCommands will handle constructing the commands
pub trait BuildCommands {
    fn build_cmd(&self) -> Vec<TrollRun>;
}
