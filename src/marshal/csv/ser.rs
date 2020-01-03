use std::io;

use super::super::super::csv::{QuoteStyle, Result, Terminator, Writer, WriterBuilder};
use super::super::super::serde::Deserialize;

use super::super::super::exec::runs::TrollRecordable;

/// CSVWriter handles the semantics of writing data to the underlying file
pub struct CSVWriter {
    zero_pad: usize,
    precision: usize,
    flush_to_zero: f64,
    writer: Writer<Box<dyn io::Write>>,
}
impl CSVWriter {
    /// Create a new writer from the configuration arguments
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

    /// handles splitting & mangling the data before writing it
    pub fn serialize_output(&mut self, data: Vec<TrollRecordable>) -> Result<()> {
        let mut data = data;
        // remove cut off data, and padd to equal length
        let longest = preprocess(&self.flush_to_zero, &mut data);
        // split our data into 2 different components (names & stats)
        let (names, stats) = break_it_up(data, longest);

        // build a buffer to hold our serialized data
        let mut output_buffer: Vec<f64> = (0..names.len()).map(|_| 0.0).collect();

        // write the names of the columns
        self.write_headers(names.as_slice())?;

        // loop over our stats (row by row)
        for row in 0..longest {
            // loop over each collect (column by column)
            for column in 0..names.len() {
                // row values into our temporary buffer
                output_buffer[column] = stats[column][row];
            }
            // serialize the output
            self.write_data(output_buffer.as_slice())?;
        }
        Ok(())
    }

    /// writer_headers starts the CSV serialization process by creating the header structure
    fn write_headers(&mut self, headers: &[String]) -> Result<()> {
        for header in headers {
            self.writer.write_field(&header)?;
        }
        self.writer.write_record(Option::<&[u8]>::None)?;
        Ok(())
    }

    /// writes a well formatted field
    fn write_data(&mut self, row: &[f64]) -> Result<()> {
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
 * Pre-Processing Helpers to manage data alignment
 *
 */

/// remove values which config says are unimportant
fn drop_trivial(flush_to_zero: &f64, data: &mut Vec<TrollRecordable>) {
    if *flush_to_zero <= 0.0 {
        return;
    }
    for vector in data.iter_mut() {
        vector.result.trim_less_than(flush_to_zero);
    }
}

/// find the longest dataset
fn find_data_maximum(data: &Vec<TrollRecordable>) -> usize {
    data.iter()
        .map(|vector| vector.result.last_value())
        .fold(0usize, |max, curr| if curr > max { curr } else { max })
}

/// insert junk 0's to ensure each dataset is of equal length
fn pad_to_maximum(data: &mut Vec<TrollRecordable>, maximum: usize) {
    for vector in data.iter_mut() {
        vector.result.pad_to(maximum);
    }
}

fn preprocess(cutoff: &f64, data: &mut Vec<TrollRecordable>) -> usize {
    drop_trivial(cutoff, data);
    let longest = find_data_maximum(data);
    pad_to_maximum(data, longest);
    longest
}

/*
 * Pre-Processing Helpers which handle formatting
 *
 */

fn break_it_up(data: Vec<TrollRecordable>, maximum: usize) -> (Vec<String>, Vec<Vec<f64>>) {
    // actual max for this function
    let max = maximum + 1;
    let mut names: Vec<String> = vec!["Damage".into()];
    let damages: Vec<f64> = (0..max).map(|x| x as f64).collect();
    let mut values: Vec<Vec<f64>> = vec![damages];
    for item in data {
        let (name, stats) = item.split();
        assert_eq!(stats.len(), max, "expected stats to have same length as everything else. length:'{}' expected:'{}' for value:'{}'", stats.len(), max, &name);
        names.push(name);
        values.push(stats);
    }

    // debug assertions
    assert_eq!(
        names.len(),
        values.len(),
        "expected # of names to be the same as # of columns. names:'{}' columns:'{}'",
        names.len(),
        values.len()
    );
    (names, values)
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
