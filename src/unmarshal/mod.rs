
use std::collections::{BTreeMap};
use std::process::{Command,Stdio};

use super::serde::{Deserialize};
use super::toml::{Value,from_str};

mod traits;
pub use self::traits::{SetArgs,BuildCommands};
mod runs;
pub use self::runs::{TrollRun};
mod parser;
pub use self::parser::{TrollLine};

/// Top level configuration format
#[derive(Clone,Deserialize,Debug)]
pub struct ConfigFormat {
    pub troll: Option<TrollConfig>,
    pub gnuplot: Option<GNUPlotConfig>,
    pub runs: BTreeMap<String,RunKind>,
}
impl ConfigFormat {

    // boostrap sets up the initial arguments of a command
    fn bootstrap(&self) -> Command {
        let mut cmd = match &self.troll {
            &Option::None => {
                let mut cmd = Command::new("troll");
                cmd.arg("0");
                cmd
            },
            &Option::Some(ref config) => {
                let mut cmd = Command::new(&config.path);
                match &config.iterations {
                    &Option::Some(ref val) if *val <= 12 => {
                        cmd.arg(format!("{}", val));
                    },
                    _ => { cmd.arg("0"); }
                };
                cmd
            },
        };
        cmd.stdin(Stdio::null());
        cmd.stderr(Stdio::piped());
        cmd.stdout(Stdio::piped()); 
        cmd
    }
}
impl BuildCommands for ConfigFormat {
    fn build_cmd(&self) -> Vec<TrollRun> {
        self.runs.iter()
            .map(|(k,v)| -> TrollRun {
                let mut cmd = self.bootstrap();
                v.set_args(&mut cmd); 
                TrollRun{ cmd, name: k.clone() } 
            })
            .collect()
    }
}

#[derive(Clone,Deserialize,Debug)]
pub struct TrollConfig {
    pub path: String,
    pub iterations: Option<usize>,
}

#[derive(Clone,Deserialize,Debug)]
pub struct GNUPlotConfig {
    pub path: Option<String>,
    pub x_size: Option<usize>,
    pub y_size: Option<usize>,
    pub font: Option<String>,
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub omit_zero: Option<bool>,
}

#[derive(Clone,Deserialize,Debug)]
#[serde(untagged)]
pub enum RunKind {
    Trivial(String),
    ComplexRun(ComplexRun),
}
impl SetArgs for RunKind {
    fn set_args(&self, cmd: &mut Command) {
        match self {
            &RunKind::Trivial(ref arg) => {
                cmd.arg(arg);
            },
            &RunKind::ComplexRun(ref complex) => {
                complex.set_args(cmd);
            },
        }
    }
}

#[derive(Clone,Deserialize,Debug)]
pub struct ComplexRun {
    pub path: String,
    #[serde(default)]
    pub args: BTreeMap<String,usize>,
}
impl SetArgs for ComplexRun {
    fn set_args(&self, cmd: &mut Command) {
        cmd.arg(&self.path);
        for (k,v) in self.args.iter() {
            cmd.arg(format!("{}={}", k,v)); 
        }
    }
}

#[test]
fn test_simple_config() {
    let dut = r#"
[troll]
path = "/home/valarauca/Documents/troll/Troll/troll"

[gnuplot]
path = "/usr/local/bin/gnuplot"
x_size = 1920
y_size = 1080
font = "arial"
title = "dda damage comparison"
x_label = "probability (%)"
y_label = "damage"
omit_zero = false

[runs]
base_dda = "lol_dda.t"
reroll_one_dda = { path = "lol_dda.t", args = { rr1 = 1 } }
"#;

    let _output = from_str::<ConfigFormat>(dut).unwrap();
}
