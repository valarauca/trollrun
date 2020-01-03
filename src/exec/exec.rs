use super::super::crossbeam::thread;

use super::super::marshal::csv::ser::CSVWriter;
use super::runs::{TrollRecordable, TrollRun};

pub fn run_program(runs: Vec<TrollRun>, writer: &mut CSVWriter) {
    // run all the various troll programs and collect their output
    let output_data: Vec<TrollRecordable> = thread::scope(|s| {
        runs.into_iter()
            .map(|item| s.spawn(|_| item.run().into_result()))
            .filter_map(|runs| runs.join().ok())
            .flat_map(|runs| runs)
            .collect()
    })
    .unwrap();
    writer.serialize_output(output_data).unwrap();
}
