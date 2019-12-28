use super::super::exec::parser::TrollLine;

/// appends records to the end to ensure there is always a zero record.

pub fn normalize(arg: &mut Vec<Vec<TrollLine>>) -> Vec<Vec<f64>> {
    let max = find_maximum_base_value(arg);

    // output the final data
    let mut output = Vec::with_capacity(arg.len());

    // align everything to the maximum
    for index in 0..(arg.len()) {
        // ensure every output has at least 1 index
        norm_to_zero(&mut arg[index]);

        // look at the last index within the array
        let local_max = arg[index][arg[index].len() - 1].base_value as usize;
        if local_max < max {
            // if needed we padd this with garabage
            for temp_max in local_max..max {
                arg[index].push(TrollLine::with_base_and_accum(temp_max, 0f64));
            }
        }

        let mut temp = Vec::with_capacity(arg[index].len());
        let mut iter = arg[index].iter().peekable();
        loop {
            match (iter.next(), iter.peek()) {
                (Option::Some(curr), Option::Some(peek)) => {
                    // push more than 1 value if that is required
                    for _ in 0..(peek.base_value - curr.base_value) {
                        temp.push(curr.accum);
                    }
                }
                (Option::Some(curr), Option::None) => {
                    temp.push(curr.accum);
                    break;
                }
                (_, _) => break,
            };
        }
        output.push(temp);
    }
    output
}

fn find_maximum_base_value(arg: &Vec<Vec<TrollLine>>) -> usize {
    let mut maximum = Option::<usize>::None;
    for index in arg.as_slice().iter() {
        let slice = index.as_slice();
        match slice.len() {
            0 => {}
            x => {
                if maximum.is_none()
                    || (maximum.is_some() && slice[x - 1].base_value > maximum.clone().unwrap())
                {
                    maximum = Some(slice[x - 1].base_value);
                }
            }
        };
    }
    match maximum {
        Option::None => 0,
        Option::Some(x) => x,
    }
}

fn norm_to_zero(arg: &mut Vec<TrollLine>) {
    // do we even have an initial value?
    match arg.len() {
        0 => {
            // data is not well formed
            arg.push(TrollLine::zero());
        }
        _ => {
            // pad data into being well formed
            if arg[0].base_value != 0 {
                for value in 0..arg[0].base_value {
                    arg.push(TrollLine::with_base_and_accum(value, 100f64));
                }
            }
        }
    };
}

#[test]
fn test_norm_to_zero_non_empty() {
    let mut idk = vec![
        TrollLine {
            base_value: 1,
            prob: 25.0,
            accum: 100.0,
        },
        TrollLine {
            base_value: 2,
            prob: 75.0,
            accum: 75.0,
        },
    ];
    norm_to_zero(&mut idk);
    assert_eq!(idk.len(), 3);
}

#[test]
fn test_norm_to_zero_empty_vector() {
    let mut idk = Vec::new();
    norm_to_zero(&mut idk);
    assert_eq!(idk.len(), 1);
}
