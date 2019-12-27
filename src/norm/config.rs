use super::super::serde::Deserialize;

/// Dataconfig informs the program what to, or to not emit
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct DataConfig {
    pub no_zero: Option<bool>,
    pub accumlate: Option<bool>,
    pub trim_trivial: Option<bool>,
}
