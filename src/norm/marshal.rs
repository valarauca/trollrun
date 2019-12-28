use super::super::marshal::csv::{BuildCSVOutput, CSVConfig};

use super::super::exec::parser::TrollLine;
use super::super::exec::runs::{RunResult, TrollOutput};
use super::norm::normalize;

pub fn serialize_output_to_csv(stuff: Vec<TrollOutput>, config: &Option<CSVConfig>) {
    // panic if a run failed
    let (output_names, mut output_data) = filter_output(stuff);
    let data = normalize(&mut output_data);

    // build config
    let cfg = match config {
        &Option::None => CSVConfig::default(),
        &Option::Some(ref config) => config.clone(),
    };
    let mut output_method = cfg.build();

    // write out headers
    output_method.write_record(&output_names).unwrap();
    for arg in data.as_slice().iter() {
        output_method.serialize(arg.as_slice()).unwrap();
    }
    output_method.flush().unwrap();
}

// clear up the data
fn filter_output(stuff: Vec<TrollOutput>) -> (Vec<String>, Vec<Vec<TrollLine>>) {
    let mut v = Vec::with_capacity(stuff.len());
    let mut n = Vec::<String>::with_capacity(stuff.len());
    let mut crash_flag = false;

    for output in stuff.into_iter() {
        match output.result {
            RunResult::ExecErr(err) => {
                println!("{} failed with error:'{:?}'", output.name, err);
                crash_flag = true;
            }
            RunResult::Error(flag, stdout, stderr) => {
                println!(
                    "{} caused troll crash return_code:'{}' stdout:'{}' stderr:'{}'",
                    output.name, flag, stdout, stderr
                );
                crash_flag = true;
            }
            RunResult::Success(data) => {
                v.push(data);
                n.push(output.name);
            }
        };
    }

    if crash_flag {
        panic!("errors detected in output, cannot normalize data");
    }
    (n, v)
}
