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
    Error(String),
    Success(Vec<TrollLine>),
}
impl From<&'static str> for RunResult {
    fn from(arg: &'static str) -> RunResult {
        RunResult::Error(arg.into())
    }
}
impl From<String> for RunResult {
    fn from(arg: String) -> RunResult {
        RunResult::Error(arg)
    }
}
impl<'a> From<&'a [u8]> for RunResult {
    fn from(arg: &'a [u8]) -> RunResult {
        RunResult::from(to_string(arg))
    }
}
impl RunResult {
    // build a run result from a yet to ran process
    fn new(arg: Command) -> RunResult {
        let mut arg = arg;
        let output = match arg.output() {
            Err(err) => return RunResult::ExecErr(err),
            Ok(output) => output,
        };

        // did the command succeed or fail?
        // troll doesn't use return codes, because of course not
        let lines: Vec<TrollLine> = match (output.stdout.len(), output.stderr.len()) {
            (0, 0) => return RunResult::from("no stdout/stderr returned from execution"),
            (_, 0) => String::from_utf8_lossy(output.stdout.as_slice())
                .lines()
                .filter_map(TrollLine::new)
                .collect(),
            _ => return RunResult::from(output.stderr.as_slice()),
        };
        if lines.len() == 0 {
            RunResult::from("valid data was returned, but parsing failed")
        } else {
            RunResult::Success(lines)
        }
    }
}

impl TrollRun {
    /// run handles converting the plan to execute into an actual execution
    pub fn run(self) -> TrollOutput {
        TrollOutput {
            name: self.name,
            result: RunResult::new(self.cmd),
        }
    }
}

fn to_string(slice: &[u8]) -> String {
    use std::str::from_utf8;
    match slice.len() {
        0 => String::new(),
        _ => match from_utf8(slice) {
            Err(_) => String::from_utf8_lossy(slice).into(),
            Ok(slice_str) => slice_str.into(),
        },
    }
}
