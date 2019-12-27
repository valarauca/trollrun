use std::io::Error;
use std::process::Command;

use super::parser::TrollLine;

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

/// RunResult contains the output of the orginal data.
pub enum RunResult {
    ExecErr(Error),
    Error(isize, String, String),
    Success(Vec<TrollLine>),
}

impl TrollRun {
    /// run handles converting the plan to execute into an actual execution
    pub fn run(self) -> TrollOutput {
        let mut s = self;
        TrollOutput {
            name: s.name,
            result: match s.cmd.output() {
                Err(e) => RunResult::ExecErr(e),
                Ok(output) => match output.status.code() {
                    Option::None | Option::Some(0) => RunResult::Success(
                        String::from_utf8_lossy(output.stdout.as_slice())
                            .lines()
                            .filter_map(TrollLine::new)
                            .collect(),
                    ),
                    Option::Some(x) => RunResult::Error(
                        x as isize,
                        String::from_utf8_lossy(output.stdout.as_slice()).to_string(),
                        String::from_utf8_lossy(output.stderr.as_slice()).to_string(),
                    ),
                },
            },
        }
    }
}
