use std::collections::BTreeMap;
use std::process::{Command, Stdio};

use super::super::serde::Deserialize;

use super::runs::TrollRun;

/// RunKind states what file we will execute
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RunKind {
    Trivial(String),
    ComplexRun(ComplexRun),
}
impl RunKind {
    pub fn into_exec(&self, name: String, troll_path: &Option<String>) -> TrollRun {
        // build the initial command structure
        let mut cmd = match troll_path {
            &Option::None => Command::new("troll"),
            &Option::Some(ref path) => Command::new(path),
        };
        cmd.arg("0");
        match self {
            &RunKind::Trivial(ref path) => {
                cmd.arg(path);
            }
            &RunKind::ComplexRun(ref complex) => {
                cmd.arg(&complex.path);
                for (k, v) in complex.args.iter() {
                    cmd.arg(&format!("{}={}", k, v));
                }
            }
        };
        cmd.stderr(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stdin(Stdio::null());
        TrollRun { cmd, name }
    }
}

/// ComplexRun describes the semantics of runs with variables
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct ComplexRun {
    pub path: String,
    #[serde(default)]
    pub args: BTreeMap<String, usize>,
}

#[test]
fn test_simple_config() {
    use super::super::toml::from_str;

    let dut = r#"
base_dda = "lol_dda.t"
reroll_one_dda = { path = "lol_dda.t", args = { rr1 = 1 } }
"#;

    let o = from_str::<BTreeMap<String, RunKind>>(dut).unwrap();

    assert_eq!(
        o.get("base_dda").unwrap(),
        &RunKind::Trivial("lol_dda.t".to_string())
    );
}
