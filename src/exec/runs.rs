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
                    Option::None | Option::Some(0) => {
                        if output.stdout.len() == 0 && output.stderr.len() == 0 {
                            RunResult::Error(1, "not stdout/stderr given".into(), "".into())
                        } else if output.stdout.len() == 0 && output.stderr.len() != 0 {
                            RunResult::Error(
                                1,
                                "not stdout given".into(),
                                to_string(&output.stderr),
                            )
                        } else {
                            let data: Vec<TrollLine> =
                                String::from_utf8_lossy(output.stdout.as_slice())
                                    .lines()
                                    .filter_map(TrollLine::new)
                                    .collect();
                            if data.len() == 0 {
                                RunResult::Error(
                                    1,
                                    "failed to parse any output".to_string(),
                                    to_string(&output.stdout),
                                )
                            } else {
                                RunResult::Success(data)
                            }
                        }
                    }
                    Option::Some(x) => RunResult::Error(
                        x as isize,
                        to_string(&output.stdout),
                        to_string(&output.stderr),
                    ),
                },
            },
        }
    }
}

fn to_string<T: AsRef<[u8]>>(arg: &T) -> String {
    use std::str::from_utf8;
    let slice = arg.as_ref();
    match slice.len() {
        0 => String::new(),
        _ => match from_utf8(slice) {
            Err(_) => String::from_utf8_lossy(slice).into(),
            Ok(slice_str) => slice_str.into(),
        },
    }
}
