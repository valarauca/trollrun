use std::io::Error;
use std::process::Command;

use super::super::cli::StatBehavior;
use super::data::TrollData;
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
impl TrollOutput {
    pub fn into_result(self) -> Option<TrollRecordable> {
        match self.result {
            RunResult::ExecErr(e) => {
                eprintln!("{} failed to exec with error:'{:?}'", self.name, e);
                None
            }
            RunResult::Error(stdout, stderr) => {
                eprintln!("{} failed.\n{}\n{}\n", self.name, stdout, stderr);
                None
            }
            RunResult::Success(data) => Some(TrollRecordable {
                name: self.name,
                result: data,
            }),
        }
    }
}

pub struct TrollRecordable {
    pub name: String,
    pub result: TrollData,
}
impl TrollRecordable {
    pub fn split(self, behavior: StatBehavior) -> (String, Vec<f64>) {
        let name = self.name;
        let mut data = Vec::with_capacity(self.result.len());
        for index in 0..self.result.len() {
            // what we output depends on our behavior enum
            match behavior {
                StatBehavior::Accumulate => {
                    data.push(self.result[index].accum);
                }
                StatBehavior::RawStats => {
                    data.push(self.result[index].prob);
                }
            }
        }
        (name, data)
    }
}

/// RunResult contains the output of the orginal data.
pub enum RunResult {
    ExecErr(Error),
    Error(String, String),
    Success(TrollData),
}
impl From<Error> for RunResult {
    fn from(arg: Error) -> Self {
        Self::ExecErr(arg)
    }
}
impl From<&'static str> for RunResult {
    fn from(arg: &'static str) -> Self {
        Self::Error(arg.into(), "".into())
    }
}
impl<'a> From<&'a [u8]> for RunResult {
    fn from(arg: &'a [u8]) -> Self {
        Self::Error(to_string(arg), "".into())
    }
}
impl RunResult {
    // build a run result from a yet to ran process
    fn new(arg: Command) -> RunResult {
        let mut arg = arg;
        let output = match arg.output() {
            Err(err) => return RunResult::from(err),
            Ok(output) => output,
        };

        // did the command succeed or fail?
        // troll doesn't use return codes, because of course not
        let (stdout, stderr) = match (output.stdout.len(), output.stderr.len()) {
            (0, 0) => return RunResult::from("no stdout/stderr returned from execution"),
            (_, _) => (
                String::from_utf8_lossy(output.stdout.as_slice()).to_string(),
                String::from_utf8_lossy(output.stderr.as_slice()).to_string(),
            ),
        };
        let lines: TrollData = stdout.lines().filter_map(TrollLine::new).collect();
        if lines.is_empty() {
            RunResult::Error(stdout, stderr)
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
