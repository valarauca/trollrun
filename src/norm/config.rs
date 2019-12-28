use super::super::serde::Deserialize;

/// Dataconfig informs the program what to, or to not emit
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct DataConfig {
    pub accumlate: Option<bool>,
    pub trim_trivial: Option<bool>,
}
