use super::super::regex::{Match, Regex};
use std::borrow::Cow;
use std::io;
use std::str::FromStr;

lazy_static! {
    static ref TROLL_LINE: Regex =
        Regex::new(r#"^\s*(\d*\.?E?~?\d*):\s*(\d+\.?\d*E?~?\d*)\s*(\d+\.?\d*E?~?\d*)\s*$"#)
            .unwrap();
    static ref NUM: Regex = Regex::new(r#"^(\d*)\.?(\d*)?E?~?(\d*)?$"#).unwrap();
}

/// TrollLine returns a line of tests of troll testing
#[derive(Clone, Copy)]
pub struct TrollLine {
    pub base_value: usize,
    pub prob: f64,
    pub accum: f64,
}
impl TrollLine {
    pub fn with_base_and_accum(base_value: usize, accum: f64) -> Self {
        Self {
            base_value,
            accum,
            prob: 0f64,
        }
    }
    pub fn zero() -> Self {
        Self {
            base_value: 0,
            prob: 0f64,
            accum: 0f64,
        }
    }
    /// new will construct a new value
    pub fn new(arg: &str) -> Option<TrollLine> {
        TROLL_LINE
            .captures(arg)
            .into_iter()
            .flat_map(|caps| {
                reformat::<_, usize>(caps.get(1))
                    .into_iter()
                    .zip(reformat::<_, f64>(caps.get(2)))
                    .zip(reformat::<_, f64>(caps.get(3)))
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
fn reformat<'b, I, F>(iter: I) -> Option<F>
where
    I: IntoIterator<Item = Match<'b>>,
    F: FromStr,
{
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
        .filter_map(|arg| <F as FromStr>::from_str(&arg).ok())
        .next()
}

#[test]
fn non_expodential_test() {
    let dut = "     37:    0.0151748971193     0.0388695987654";
    let out = TrollLine::new(dut).unwrap();
    assert_eq!(out.base_value, 37);
    assert_eq!(out.prob, 0.0151748971193f64);
    assert_eq!(out.accum, 0.0388695987654f64);
}

#[test]
fn expodential_test() {
    let dut = "     45:    7.14449016918E~5    9.82367398262E~5  ";
    let out = TrollLine::new(dut).unwrap();
    assert_eq!(out.base_value, 45);
    assert_eq!(out.prob, 7.14449016918e-5f64);
    assert_eq!(out.accum, 9.82367398262e-5f64);
}

#[test]
fn lol_wtf() {
    let dut = include_str!("dda_test_run_output");
    let output: Vec<TrollLine> = dut.lines().filter_map(TrollLine::new).collect();
    assert_eq!(output.len(), 37);

    // check a few cases
    assert_eq!(output[0].base_value, 0);
    assert_eq!(output[0].prob, 45.419654612);
    assert_eq!(output[0].accum, 100.0);
}
