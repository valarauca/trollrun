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
    pub fn into_exec(&self, name: &String, troll_path: &Option<String>) -> TrollRun {
        // build the initial command structure
        let mut cmd = match troll_path {
            &Option::None => Command::new("troll"),
            &Option::Some(ref path) => Command::new(path),
        };
        self.add_args(&mut cmd);
        TrollRun {
            cmd,
            name: name.to_string(),
        }
    }

    fn add_args(&self, cmd: &mut Command) {
        cmd.arg("0");
        match self {
            &RunKind::Trivial(ref path) => {
                cmd.arg(path);
            }
            &RunKind::ComplexRun(ref complex) => {
                cmd.arg(&complex.path);
                complex.add_optional_args(cmd);
                complex.add_mmap(cmd);
            }
        };
    }
}

/// ComplexRun describes the semantics of runs with variables
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct ComplexRun {
    pub path: String,
    #[serde(default)]
    pub mmap_override: Option<bool>,
    #[serde(default)]
    pub args: BTreeMap<String, usize>,
}
impl ComplexRun {
    /// manage specifying the arguments
    fn add_optional_args(&self, cmd: &mut Command) {
        for (k, v) in self.args.iter() {
            cmd.arg(&format!("{}={}", k, v));
        }
    }

    /// add_mmap majorly fucks with trolls execution environment.
    ///
    /// The reason for this is weird. But you can get the gist if
    /// you read the Moscow-ML compiler's, TODO list https://github.com/kfl/mosml/blob/master/src/notes/TODO#L1906
    ///
    /// > Cody?! Moscow-ML?!? This program executes troll! Not Moscow-ML.
    ///
    /// Yes, yes you are correct. Alas, Troll is not a necessarily
    /// a stand-alone executable. Troll is written in Standard-ML,
    /// this means that Troll gets compiled to a bytecode (by
    /// Moscow-ML). Now the Moscowhhhhhhhh-ML's virtual-machine will execute
    /// Troll. This looks like a normal binary because of some
    /// clever dynamic linking.
    ///
    /// > But, why do we need to mess with Moscow-ML's runtime?
    ///
    /// Well we need to ensure `malloc()` calls are served by the `mmap()`
    /// system call instead of `sbrk()`. This is because (as far as I can tell)
    /// when `malloc()` starts returning pointers which are "too-far-away"
    /// from other `malloc()` calls, and it'll OOM itself.
    ///
    /// So the plan is to force it to always use MMAP.
    fn add_mmap(&self, cmd: &mut Command) {
        match &self.mmap_override {
            &Option::None | &Option::Some(false) => return,
            &Option::Some(true) => {}
        };

        // remove all the stuff that'll interfere
        for term in PURGE_GNU_MALLOC_OPTIONS {
            cmd.env_remove(term);
        }

        // add stuff that can work together
        cmd.env("M_MMAP_THRESHOLD", "0");
        cmd.env("MALLOC_MMAP_THRESHOLD_", "0");
        cmd.env("M_MMAP_MAX", "4294967295");
        cmd.env("MALLOC_MMAP_MAX_", "4294967295");
    }
}

const PURGE_GNU_MALLOC_OPTIONS: &'static [&'static str] = &[
    "M_MMAP_MAX",
    "MALLOC_MMAP_MAX_",
    "M_MMAP_THRESHOLD",
    "MALLOC_MMAP_THRESHOLD_",
    "M_PERTURB",
    "MALLOC_MMAP_PERTURB_",
    "M_TOP_PAD",
    "MALLOC_TOP_PAD_",
    "M_TRIM_THRESHOLD",
    "MALLOC_TRIM_THRESHOLD_",
    "M_ARENA_TEST",
    "MALLOC_ARENA_TEST",
    "M_ARENA_MAX",
    "MALLOC_ARENA_MAX",
];

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
