use std::io::Write;

use super::super::super::csv::{QuoteStyle, Writer, WriterBuilder};
use super::super::super::serde::Deserialize;

use super::BuildCSVOutput;

#[derive(Clone, Debug, Deserialize)]
pub struct CSVConfig {
    pub path: Option<String>,
    pub seperator: Option<char>,
    pub quote: Option<char>,
    pub buffer: Option<usize>,
    #[serde(default)]
    pub eol: Option<EOLSpecification>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EOLSpecifiation {
    pub lf: Option<bool>,
    pub cr: Option<bool>,
    pub lfcr: Option<bool>,
    pub specify: Option<char>,
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
            Option::Some(ref c) if c.is_ascii() => {
                w.delimiter(*c as u32 as u8);
            }
            _ => {
                // ascii comma
                w.delimiter(44);
            }
        };

        // set up optional wrapping
        match &self.quote {
            Option::Some(ref c) if c.is_ascii() => {
                w.quote(*c as u32 as u8);
                // if you force a quote style it'll always be used.
                w.quote_style(QuoteStyle::Always);
            }
            _ => {
                // ascii quote
                w.quote(34);
            }
        };

        // TODO return the output
    }
}
