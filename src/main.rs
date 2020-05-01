#![allow(unused_imports, dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate csv;
extern crate regex;
extern crate serde;
extern crate toml;

pub mod cli;
pub use self::cli::AppConfig;
pub mod exec;
use self::exec::config::RunKind;
use self::exec::exec::run_program;
pub mod marshal;
use self::marshal::csv::config::CSVConfig;
pub mod unmarshal;
use self::unmarshal::ConfigFormat;

fn main() {
    // read OS flags
    let flags = AppConfig::default();

    // load config
    let config = ConfigFormat::new(&flags.file_path).expect("file was validated by cli");
    let runs = config.build_runs();
    // build our output format
    let mut csv_config = CSVConfig::build_config(&config.csv);
    run_program(runs, &mut csv_config);
    csv_config.flush().unwrap();
}
