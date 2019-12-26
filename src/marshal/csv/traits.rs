use std::io::Write;

use super::super::super::csv::Writer;

/// BuildCSVOutput controls out the CSV is produced
pub trait BuildCSVOutput {
    fn build(&self) -> Writer<Box<dyn Write>>;
}
