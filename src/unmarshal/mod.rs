use std::collections::BTreeMap;

use super::serde::Deserialize;
use super::toml::from_str;

use super::exec::config::RunKind;
use super::exec::runs::TrollRun;
use super::marshal::csv::config::CSVConfig;

/// Top level configuration format
#[derive(Clone, Deserialize, Debug)]
pub struct ConfigFormat {
    #[serde(default)]
    pub troll: Option<TrollConfig>,
    #[serde(default)]
    pub csv: Option<CSVConfig>,
    #[serde(default)]
    pub runs: BTreeMap<String, RunKind>,
}
impl ConfigFormat {
    // load a config from the command line interface
    pub fn new(path: &str) -> Result<ConfigFormat, String> {
        let config = match ::std::fs::read_to_string(&path) {
            Err(e) => return Err(format!("failed to open:'{}' error:'{:?}'", &path, e)),
            Ok(config) => config,
        };
        match from_str::<ConfigFormat>(&config) {
            Err(e) => Err(format!(
                "failed to parse config:'{}' error:'{:?}'",
                &path, e
            )),
            Ok(config) => Ok(config),
        }
    }

    /// builds all the items that need to be ran
    pub fn build_runs(&self) -> Vec<TrollRun> {
        let troll_path = self.get_troll_path();
        self.runs
            .iter()
            .map(|(name, run)| run.into_exec(name, &troll_path))
            .collect()
    }

    /// returns the path to the damn executable
    fn get_troll_path<'a>(&'a self) -> Option<String> {
        match &self.troll {
            &Option::Some(ref cfg) => match &cfg.path {
                &Option::Some(ref path) => Some(path.clone()),
                _ => None,
            },
            _ => None,
        }
    }
}

/// How do you want to run troll
#[derive(Clone, Deserialize, Debug)]
pub struct TrollConfig {
    pub path: Option<String>,
}

#[test]
fn test_simple_config() {
    let dut = r#"
[troll]
path = "/home/valarauca/Documents/troll/Troll/troll"

[csv]
path = "/usr/local/bin/gnuplot"

[runs]
base_dda = "lol_dda.t"
reroll_one_dda = { path = "lol_dda.t", args = { rr1 = 1 } }
"#;

    let _output = from_str::<ConfigFormat>(dut).unwrap();
}
