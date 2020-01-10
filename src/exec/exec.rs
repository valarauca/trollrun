use std::thread::{Builder, JoinHandle};

use super::super::marshal::csv::ser::CSVWriter;
use super::runs::{TrollRecordable, TrollRun};

pub fn run_program(runs: Vec<TrollRun>, writer: &mut CSVWriter) {
    // run all the various troll programs and collect their output
    let output_data: Vec<TrollRecordable> = {
        // spawn all threads, collect all handles.
        let mut handles = Vec::with_capacity(runs.len());
        for item in runs.into_iter() {
            handles.push(build_thread(item));
        }

        // we will block ourselves once we try to join and be scheduled off

        // collect their output
        handles
            .into_iter()
            .filter_map(|handle| handle.join().ok())
            .filter_map(|output| output)
            .collect()
    };
    writer.serialize_output(output_data).unwrap();
}

fn build_thread(arg: TrollRun) -> JoinHandle<Option<TrollRecordable>> {
    Builder::new()
        .name(format!("trollrun_{}", &arg.name))
        .stack_size(1024 * 1024)
        .spawn(move || arg.run().into_result())
        .unwrap()
}
