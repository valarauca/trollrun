use super::super::crossbeam::thread;

use super::super::marshal::csv::ser::CSVWriter;
use super::runs::{TrollRecordable, TrollRun};

pub fn run_program(runs: Vec<TrollRun>, writer: &mut CSVWriter) {
    // run all the various troll programs and collect their output
    let mut output_data: Vec<TrollRecordable> = thread::scope(|s| {
        runs.into_iter()
            .map(|item| s.spawn(|_| item.run().into_result()))
            .filter_map(|runs| runs.join().ok())
            .flat_map(|runs| runs)
            .collect()
    })
    .unwrap();

    // find the maximum index
    let mut max = 0usize;
    for item in output_data.iter() {
        let local_max = item.result.last_value();
        max = if local_max > max { local_max } else { max };
    }
    // pad to maximum index
    for item in output_data.iter_mut() {
        item.result.pad_to(max);
    }

    // record headers
    let mut names: Vec<String> = output_data.iter().map(|item| item.name.clone()).collect();
    names.insert(0, "Damage".into());
    writer.write_headers(names.as_slice()).unwrap();

    // build a vector to store information about row
    let mut data_row: Vec<f64> = Vec::with_capacity(names.len());
    for _ in 0..names.len() {
        data_row.push(0.0);
    }

    // now that we have our row we walk all our data serializing it
    for i in 0..max {
        // populate the row
        for ii in 0..names.len() {
            if ii == 0 {
                data_row[ii] = i as f64;
            } else {
                data_row[ii] = output_data[ii - 1].result[i].accum;
            }
        }
        // serialize the row
        writer.write_data(data_row.as_slice()).unwrap();
    }
}
