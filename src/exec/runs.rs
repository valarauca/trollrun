use std::io::Error;
use std::process::Command;

/// TrollRun is a labled run of troll ready to be executed
pub struct TrollRun {
    pub name: String,
    pub cmd: Command,
}

/// TrollOutput describes what happened
pub struct TrollOutput {
    pub name: String,
    pub result: RunResult,
}

pub enum RunResult {
    ExecErr(Error),
    Error(Option<i32>, String, String),
    Success(Vec<TrollLine>),
}

impl TrollRun {
    pub fn run(self) -> RunResult {}
}
