use std::iter::{FromIterator, IntoIterator, Iterator};
use std::ops::{Index, IndexMut};

use super::parser::TrollLine;

/// TrollData is an opaque type that contains information prior
#[derive(Clone)]
pub struct TrollData {
    data: DataCollector,
}
impl TrollData {
    /// how long is this chunk of data?
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// is there no data?
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// returns the maximum base value in the array (this should be equal to the last index)
    pub fn last_value(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.last_value()
        }
    }

    /// pad_to will insert junk data to ensure our last value is equal to the requested
    /// this junk data is "zero probability events"
    pub fn pad_to(&mut self, last_value: usize) {
        let own_last = self.last_value();
        if own_last >= last_value {
            return;
        }
        for curr in (own_last + 1)..(last_value + 1) {
            self.data.append(DataPoint::prob_zero(curr));
        }
    }
}
impl Index<usize> for TrollData {
    type Output = DataPoint;
    fn index<'a>(&'a self, i: usize) -> &'a DataPoint {
        &self.data.data[i]
    }
}
impl IndexMut<usize> for TrollData {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut DataPoint {
        &mut self.data.data[i]
    }
}

impl FromIterator<TrollLine> for TrollData {
    fn from_iter<T: IntoIterator<Item = TrollLine>>(iter: T) -> TrollData {
        let iter = iter.into_iter();

        // see if we can do pre-sizing
        let mut vec = match iter.size_hint() {
            (0, Option::None) | (0, Option::Some(0)) => DataCollector::default(),
            (x, Option::None) | (_, Option::Some(x)) => DataCollector::with_capacity(x),
        };

        for item in iter {
            // handle the case if our initial value is not zero
            if vec.is_empty() && item.base_value >= 1 {
                for value in 0..item.base_value {
                    vec.append(DataPoint::prob_zero(value));
                }
            }

            // handle the case if our last value is != next value
            if !vec.is_empty() && (vec.last_value() + 1) < item.base_value {
                let last = vec.last_value() + 1;
                let next = item.base_value;
                for value in last..next {
                    vec.append(DataPoint::prob_zero(value));
                }
            }

            // now we can just push our point
            vec.append(DataPoint::from(item));
        }

        // now we loop over the array (in reverse order) and double check accumlation values
        let mut accum_total = 0.0;
        for i in (0..vec.len()).rev() {
            accum_total += vec[i].prob;
            vec[i].accum = accum_total;
        }

        // shove the data out the door
        TrollData { data: vec }
    }
}

#[derive(Clone, Default)]
pub struct DataPoint {
    pub value: usize,
    pub prob: f64,
    pub accum: f64,
}
impl From<TrollLine> for DataPoint {
    fn from(arg: TrollLine) -> DataPoint {
        DataPoint {
            value: arg.base_value,
            prob: arg.prob,
            accum: arg.accum,
        }
    }
}
impl DataPoint {
    fn prob_zero(value: usize) -> DataPoint {
        DataPoint {
            value,
            prob: 0.0,
            accum: 0.0,
        }
    }
}

#[derive(Clone, Default)]
struct DataCollector {
    data: Vec<DataPoint>,
}
impl Index<usize> for DataCollector {
    type Output = DataPoint;
    fn index<'a>(&'a self, i: usize) -> &'a DataPoint {
        &self.data[i]
    }
}
impl IndexMut<usize> for DataCollector {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut DataPoint {
        &mut self.data[i]
    }
}
impl DataCollector {
    fn with_capacity(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn last_value(&self) -> usize {
        if self.is_empty() {
            panic!("cannot take last value when no values exist");
        }
        self.data[self.data.len() - 1].value
    }

    fn append(&mut self, arg: DataPoint) {
        self.data.push(arg);
    }
}

#[test]
fn test_full_output() {
    let dut = include_str!("dda_test_run_output");
    let output: TrollData = dut.lines().filter_map(TrollLine::new).collect();
    // this is copy & pasted from the test file
    // so it is a basic sanity check
    let expectation: &[f64] = &[
        45.419654612,
        6.04316063934,
        6.43541699727,
        6.84424488453,
        7.27008219743,
        7.71337341428,
        8.17456963838,
        2.61096800175,
        2.32484154962,
        2.00829900696,
        1.66006682212,
        1.27884537361,
        0.863308754948,
        0.412104559602,
        0.316110023804,
        0.231377325701,
        0.15910064581,
        0.100512624153,
        0.0568847904514,
        0.0295279943235,
        0.0197928354791,
        0.0124985645764,
        0.00732498688266,
        0.00392712838685,
        0.00193480560429,
        0.000952195380663,
        0.00055740469627,
        0.00030204047021,
        0.000148675668003,
        6.54327809699e-5,
        2.61989241359e-5,
        1.08409341252e-5,
        5.4204670626e-6,
        2.40909647227e-6,
        9.034111771e-7,
        2.58117479171e-7,
        4.30195798619e-8,
    ];
    assert_eq!(output.last_value(), 36);
    assert_eq!(output.len(), expectation.len());
    for index in 0..output.len() {
        assert_eq!(
            output[index].prob, expectation[index],
            "for index:'{}' found value:'{}' expected:'{}'",
            index, output[index].prob, expectation[index]
        );
    }
}

#[test]
fn test_weird_pylon_table() {
    let dut = include_str!("pylon.data");
    let output: TrollData = dut.lines().filter_map(TrollLine::new).collect();

    assert_eq!(output[0].prob, 13.2305687116);
    assert_eq!(output[14].prob, 9.6660149794);
    assert_eq!(output[15].prob, 0.0);
    assert_eq!(output[16].prob, 9.6660149794);
    assert_eq!(output[17].prob, 0.0);
}
