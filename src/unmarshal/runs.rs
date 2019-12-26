use std::process::{Command};
use std::io;

use super::super::crossbeam::thread;

use super::parser::{TrollLine,producer,TrollExecution};

/// TrollRun is a labled run of troll ready to be executed
pub struct TrollRun {
    pub name: String,
    pub cmd: Command,
}

/// converts our list of setup to run commands into actual processes we can run
pub fn collect_output<'a>(runs: Vec<TrollRun>) -> Vec<(String,Vec<TrollLine>)> {

    // run all the various troll programs and collect their output
    thread::scope(|s| {
        runs.into_iter()
            .map(|item| s.spawn(|_| (item.name, producer(item.cmd))))
            .filter_map(|runs| runs.join().ok())
            .filter_map(|(name,status)| -> Option<(String,Vec<TrollLine>)> {
                match status {
                    TrollExecution::Success(data) => {
                        println!("{} success", &name);
                        Some((name,data))
                    },
                    TrollExecution::ExecFailure(err) => {
                        println!("{} exec failed. error: {:?}", &name, &err);
                        None
                    },
                    TrollExecution::RunFailure(exit_status, stderr) => {
                        println!("{} troll returned error. exitcode:{} stderr:'{}'", &name, &exit_status, &stderr);
                        None
                    },
                }
            })
            .collect()
    }).unwrap()
}
