#![allow(unused_imports, dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate csv;
extern crate regex;
extern crate serde;
extern crate toml;

pub mod exec;
use self::exec::config::RunKind;
use self::exec::exec::run_program;
pub mod marshal;
use self::marshal::csv::config::CSVConfig;
pub mod unmarshal;
use self::unmarshal::ConfigFormat;

fn main() {
    // load config
    let config = ConfigFormat::new();
    let runs = config.build_runs();
    // build our output format
    let mut csv_config = CSVConfig::build_config(&config.csv);
    run_program(runs, &mut csv_config);
    csv_config.flush().unwrap();
}
