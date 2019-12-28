use std::collections::BTreeMap;

use super::serde::Deserialize;
use super::toml::from_str;

use super::exec::config::RunKind;
use super::marshal::csv::CSVConfig;

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
    pub fn new() -> ConfigFormat {
        let path = match std::env::args().skip(1).next() {
            Option::None => panic!("provide 1 argument to run that config"),
            Option::Some(path) => path,
        };
        let config = match ::std::fs::read_to_string(&path) {
            Err(e) => panic!("failed to open:'{}' error:'{:?}'", &path, e),
            Ok(config) => config,
        };
        match from_str::<ConfigFormat>(&config) {
            Err(e) => panic!("failed to parse config:'{}' error:'{:?}'", &path, e),
            Ok(config) => config,
        }
    }

    /// returns the path to the damn executable
    pub fn get_troll_path<'a>(&'a self) -> Option<String> {
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
