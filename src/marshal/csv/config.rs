use std::fs::OpenOptions;
use std::io::{stdout, Write};

use super::super::super::csv::{QuoteStyle, Terminator, Writer, WriterBuilder};
use super::super::super::serde::Deserialize;

use super::ser::CSVWriter;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct CSVConfig {
    pub path: Option<String>,
    pub seperator: Option<char>,
    pub quote: Option<char>,
    pub precision: Option<usize>,
    pub zero_pad: Option<usize>,
    pub flush_to_zero: Option<f64>,
    #[serde(default)]
    pub eol: Option<EOLSpecification>,
}
impl CSVConfig {
    /// handles deserializing the input configuration junk
    pub fn build_config(config: &Option<Self>) -> CSVWriter {
        let normal = Self::default();
        let config = match config {
            &Option::None => &normal,
            &Option::Some(ref arg) => arg,
        };
        let eol = get_eol_specification(&config.eol);

        // determine where to write to
        let output: Box<dyn Write> = match &config.path {
            &Option::Some(ref path) => {
                match OpenOptions::new()
                    .read(false)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                {
                    Err(e) => panic!("could not open file:'{}' error:'{:?}'", path, e),
                    Ok(file) => Box::new(file),
                }
            }
            _ => Box::new(stdout()),
        };

        CSVWriter::new(
            output,
            &config.seperator,
            &config.quote,
            &config.precision,
            &config.zero_pad,
            &config.flush_to_zero,
            eol,
        )
    }
}

/*
 * EOL Handling
 *
 */

#[derive(Clone, Debug, Deserialize, Default)]
pub struct EOLSpecification {
    pub lf: Option<bool>,
    pub cr: Option<bool>,
    pub crlf: Option<bool>,
    pub specify: Option<u8>,
}

fn get_eol_specification(eol: &Option<EOLSpecification>) -> Terminator {
    let eol = match eol {
        &Option::None => return Terminator::CRLF,
        &Option::Some(ref eol) => eol,
    };
    match &eol.lf {
        &Option::Some(true) => return Terminator::Any(10),
        _ => {}
    };
    match &eol.cr {
        &Option::Some(true) => return Terminator::Any(13),
        _ => {}
    };
    match &eol.crlf {
        &Option::Some(true) => return Terminator::CRLF,
        _ => {}
    };
    match &eol.specify {
        &Option::Some(val) => return Terminator::Any(val),
        _ => {}
    };
    Terminator::Any(13)
}
