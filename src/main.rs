#![allow(unused_imports, dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate crossbeam;
extern crate csv;
extern crate regex;
extern crate serde;
extern crate toml;

pub mod exec;
use self::exec::config::RunKind;
use self::exec::exec::collect_output;
pub mod marshal;
pub mod unmarshal;
use self::unmarshal::ConfigFormat;
//pub mod norm;
//use self::norm::marshal::serialize_output_to_csv;

/*
fn main() {
    // load config
    let config = ConfigFormat::new();
    // figure out what we're executing
    let troll_path = config.get_troll_path();
    // run the simulation, format the output, and collect it
    let simulation_result = collect_output(
        config
            .runs
            .iter()
            .map(|(k, v)| v.into_exec(k.clone(), &troll_path))
            .collect(),
    );
    serialize_output_to_csv(simulation_result, &config.csv);
}

*/
