use std::fs::metadata;

use clap::{App, Arg, ArgMatches};

use super::unmarshal::ConfigFormat;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum StatBehavior {
    Accumulate,
    RawStats,
}
impl From<bool> for StatBehavior {
    fn from(x: bool) -> StatBehavior {
        if x {
            StatBehavior::Accumulate
        } else {
            StatBehavior::RawStats
        }
    }
}

/// AppConfig declares a lot of information about how the program should function.
pub struct AppConfig {
    pub file_path: String,
    pub behavior: StatBehavior,
}
impl Default for AppConfig {
    fn default() -> AppConfig {
        let app = build_cli();
        let matches = app.get_matches();
        AppConfig {
            file_path: matches.value_of("FILE").unwrap().to_string(),
            behavior: StatBehavior::from(matches.is_present("accum")),
        }
    }
}

fn build_cli() -> App<'static, 'static> {
    App::new("trollrun")
        .author("valarauca")
        .version("1.0.0")
        .max_term_width(80)
        .about("batch running of troll files")
        .arg(
            Arg::with_name("FILE")
                .index(1)
                .takes_value(true)
                .required(true)
                .env("TROLLRUN_INPUT_FILE_PATH")
                .validator(validate_input_file)
                .help("the configuration toml file for the build"),
        )
        .arg(
            Arg::with_name("accum")
                .short("a")
                .long("accum")
                .takes_value(false)
                .help("return accumulations instead of raw probability"),
        )
}

fn validate_input_file(arg: String) -> Result<(), String> {
    let m = match metadata(&arg) {
        Err(e) => match e.raw_os_error() {
            Option::None => {
                return Err(format!("could not open file '{}' error '{:?}'", &arg, e));
            }
            Option::Some(code) => {
                return Err(format!(
                    "error: {} could not open file '{}' error '{:?}'",
                    code, &arg, e
                ));
            }
        },
        Ok(m) => m,
    };
    if m.is_dir() {
        return Err(format!("cannot read directory {}", &arg));
    }
    match ConfigFormat::new(&arg) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
