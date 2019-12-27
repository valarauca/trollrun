use std::fs::OpenOptions;
use std::io::{stdout, Write};

use super::super::super::csv::{QuoteStyle, Terminator, Writer, WriterBuilder};
use super::super::super::serde::Deserialize;

use super::BuildCSVOutput;

#[derive(Clone, Debug, Deserialize)]
pub struct CSVConfig {
    pub path: Option<String>,
    pub seperator: Option<char>,
    pub quote: Option<char>,
    #[serde(default)]
    pub eol: Option<EOLSpecification>,
}

#[derive(Clone, Debug, Deserialize)]
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

// construct the CSV output
impl BuildCSVOutput for CSVConfig {
    fn build(&self) -> Writer<Box<dyn Write>> {
        let mut w = WriterBuilder::new();
        // we will never have headers, well we will
        // but not as this library thinks, we specify
        // them at runtime
        w.has_headers(false);

        // set up field seperator
        match &self.seperator {
            &Option::Some(ref c) if c.is_ascii() => {
                w.delimiter(*c as u32 as u8);
            }
            _ => {
                // ascii comma
                w.delimiter(44);
            }
        };

        // set up optional wrapping
        match &self.quote {
            &Option::Some(ref c) if c.is_ascii() => {
                w.quote(*c as u32 as u8);
                // if you force a quote style it'll always be used.
                w.quote_style(QuoteStyle::Always);
            }
            _ => {
                // ascii quote
                w.quote(34);
            }
        };

        // figure out how to terminator lines
        w.terminator(get_eol_specification(&self.eol));

        // determine where to write to
        let output: Box<dyn Write> = match &self.path {
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
        w.from_writer(output)
    }
}
