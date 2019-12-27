use super::super::crossbeam::thread;

use super::runs::{TrollOutput, TrollRun};

/// converts our list of setup to run commands into actual processes we can run
pub fn collect_output(runs: Vec<TrollRun>) -> Vec<TrollOutput> {
    // run all the various troll programs and collect their output
    thread::scope(|s| {
        runs.into_iter()
            .map(|item| s.spawn(|_| item.run()))
            .filter_map(|runs| runs.join().ok())
            .collect()
    })
    .unwrap()
}
