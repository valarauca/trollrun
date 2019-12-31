use super::super::super::csv::{QuoteStyle, Result, Terminator, Writer, WriterBuilder};
use super::super::super::serde::Deserialize;
use std::io;

/// CSVWriter handles the semantics of writing data to the underlying file
pub struct CSVWriter {
    zero_pad: usize,
    precision: usize,
    flush_to_zero: f64,
    writer: Writer<Box<dyn io::Write>>,
}
impl CSVWriter {
    /// Create a new writer
    pub fn new(
        writer: Box<dyn io::Write>,
        seperator: &Option<char>,
        quote: &Option<char>,
        precision: &Option<usize>,
        zero_pad: &Option<usize>,
        flush_to_zero: &Option<f64>,
        eol: Terminator,
    ) -> Self {
        let mut w = WriterBuilder::new();
        w.has_headers(false);
        match seperator {
            &Option::Some(ref c) if c.is_ascii() => w.delimiter(*c as u32 as u8),
            _ => w.delimiter(44),
        };
        match quote {
            &Option::Some(ref c) if c.is_ascii() => w.quote(*c as u32 as u8),
            _ => w.quote(34),
        };
        w.terminator(eol);
        let flush_to_zero = match flush_to_zero {
            &Option::Some(ref f) if *f > 0.0 => *f,
            _ => 0.0,
        };
        let precision = match precision {
            &Option::Some(ref p) => *p,
            _ => 0,
        };
        let zero_pad = match zero_pad {
            &Option::Some(ref z) => *z,
            _ => 0,
        };
        Self {
            flush_to_zero,
            zero_pad,
            precision,
            writer: w.from_writer(writer),
        }
    }

    /// writer_headers starts the CSV serialization process by creating the header structure
    pub fn write_headers(&mut self, headers: &[String]) -> Result<()> {
        for header in headers {
            self.writer.write_field(&header)?;
        }
        self.writer.write_record(Option::<&[u8]>::None)?;
        Ok(())
    }

    /// writes a well formatted field
    pub fn write_data(&mut self, row: &[f64]) -> Result<()> {
        for field in row.iter() {
            self.writer.write_field(format_float(
                field,
                &self.zero_pad,
                &self.precision,
                &self.flush_to_zero,
            ))?;
        }
        self.writer.write_record(Option::<&[u8]>::None)?;
        Ok(())
    }

    /// cleans up the object flushing the underlying IO
    pub fn flush(self) -> io::Result<()> {
        let mut s = self;
        s.writer.flush()
    }
}

/*
 * std::fmt::Arguments is broke
 *
 */

fn format_float(x: &f64, zero_pad: &usize, prec: &usize, flush_to_zero: &f64) -> String {
    let mut x = *x;
    if x <= *flush_to_zero {
        x = 0.0;
    }
    format!("{:0z$.p$}", x, z = *zero_pad, p = *prec)
}
