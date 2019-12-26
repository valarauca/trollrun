
use std::str::FromStr;
use std::borrow::Cow;
use std::process::{Command};
use std::io;

use super::super::regex::{Match, Regex};

/// producer handles the semantics of streaming the output from the command
pub fn producer(cmd: Command) -> TrollExecution {
    // set up stdio environment
    let mut cmd = cmd;

    // run the command
    let output = match cmd.output() {
        Ok(output) => output,
        Err(e) => return TrollExecution::ExecFailure(e),
    };

    // inspect the output
    match output.status.code() {
        Option::None | Option::Some(0) => { },
        Option::Some(value) => {
            return TrollExecution::RunFailure(value, String::from_utf8_lossy(output.stderr.as_slice()).to_string());
        }
    };

    // parse the output
    TrollExecution::Success(String::from_utf8_lossy(output.stdout.as_slice()).lines().filter_map(TrollLine::new).collect())
}

/// TrollExecution describes the state of the run
pub enum TrollExecution {
    ExecFailure(io::Error),
    RunFailure(i32,String),
    Success(Vec<TrollLine>)
}

lazy_static! {
    static ref TROLL_LINE: Regex =
        Regex::new(r#"^\s*(\d*\.?E?~?\d*):\s*(\d+\.?\d*E?~?\d*)\s*(\d+\.?\d*E?~?\d*)\s*$"#)
            .unwrap();
    static ref NUM: Regex = Regex::new(r#"^(\d*)\.?(\d*)?E?~?(\d*)?$"#).unwrap();
}

/// TrollLine returns a line of tests of troll testing
#[derive(Clone,Copy)]
pub struct TrollLine {
    pub base_value: f64,
    pub prob: f64,
    pub accum: f64,
}
impl TrollLine {
    /// new will construct a new value
    pub fn new(arg: &str) -> Option<TrollLine> {
        TROLL_LINE
            .captures(arg)
            .into_iter()
            .flat_map(|caps| {
                reformat(caps.get(1))
                    .into_iter()
                    .zip(reformat(caps.get(2)))
                    .zip(reformat(caps.get(3)))
            })
            .map(|((a, b), c)| TrollLine {
                base_value: a,
                prob: b,
                accum: c,
            })
            .next()
    }
}

// format handles string formatting
#[inline(always)]
fn reformat<'b, I: IntoIterator<Item = Match<'b>>>(iter: I) -> Option<f64> {
    iter.into_iter()
        .filter(|item| !item.as_str().is_empty())
        .filter_map(|item| NUM.captures(item.as_str()))
        .filter_map(|caps| match (caps.get(1), caps.get(2), caps.get(3)) {
            (Option::Some(ref a), Option::None, Option::None) if !a.as_str().is_empty() => {
                Some(Cow::Borrowed(a.as_str()))
            }
            (Option::Some(ref a), Option::Some(ref b), Option::None)
                if !a.as_str().is_empty() && b.as_str().is_empty() =>
            {
                Some(Cow::Borrowed(a.as_str()))
            }
            (Option::Some(ref a), Option::Some(ref b), Option::Some(ref c))
                if !a.as_str().is_empty() && b.as_str().is_empty() && c.as_str().is_empty() =>
            {
                Some(Cow::Borrowed(a.as_str()))
            }

            (Option::Some(ref a), Option::Some(ref b), Option::None)
                if !a.as_str().is_empty() && !b.as_str().is_empty() =>
            {
                Some(Cow::Owned(format!("{}.{}", a.as_str(), b.as_str())))
            }
            (Option::Some(ref a), Option::Some(ref b), Option::Some(ref c))
                if !a.as_str().is_empty() && !b.as_str().is_empty() && c.as_str().is_empty() =>
            {
                Some(Cow::Owned(format!("{}.{}", a.as_str(), b.as_str())))
            }

            (Option::Some(ref a), Option::Some(ref b), Option::Some(ref c))
                if !a.as_str().is_empty() && !b.as_str().is_empty() && !c.as_str().is_empty() =>
            {
                Some(Cow::Owned(format!(
                    "{}.{}e-{}",
                    a.as_str(),
                    b.as_str(),
                    c.as_str()
                )))
            }
            (_, _, _) => None,
        })
        .filter_map(|arg| <f64 as FromStr>::from_str(&arg).ok())
        .next()
}

#[test]
fn non_expodential_test() {
    let dut = "     37:    0.0151748971193     0.0388695987654";
    let out = TrollLine::new(dut).unwrap();
    assert_eq!(out.base_value, 37f64);
    assert_eq!(out.prob, 0.0151748971193f64);
    assert_eq!(out.accum, 0.0388695987654f64);
}

#[test]
fn expodential_test() {
    let dut = "     45:    7.14449016918E~5    9.82367398262E~5  ";
    let out = TrollLine::new(dut).unwrap();
    assert_eq!(out.base_value, 45f64);
    assert_eq!(out.prob, 7.14449016918e-5f64);
    assert_eq!(out.accum, 9.82367398262e-5f64);
}

