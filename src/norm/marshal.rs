use super::super::marshal::csv::{BuildCSVOutput, CSVConfig};

use super::super::exec::parser::TrollLine;
use super::super::exec::runs::{RunResult, TrollOutput};

pub fn do_everything(stuff: Vec<TrollOutput>, config: &Option<CSVConfig>) {
    // panic if a run failed
    let (output_names, output_data) = filter_output(stuff);

    // build config
    let cfg = match config {
        &Option::None => CSVConfig::default(),
        &Option::Some(ref config) => config.clone(),
    };
    let mut output_method = cfg.build();

    // write out headers
    output_method.write_record(&output_names).unwrap();
    for arg in output_data {
        ensure_count(&arg);
    }
}

fn ensure_count(arg: &Vec<TrollLine>) {
    let mut base_value = Option::<f64>::None;
    for item in arg.iter() {
        if base_value.is_some() {
            if base_value.clone().unwrap() != item.base_value {
                panic!("non-normalized output detected. Base values do not align");
            }
        } else {
            ::std::mem::replace(&mut base_value, Some(item.base_value));
        }
    }
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
