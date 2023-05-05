mod openrank;

use gluesql::{
    core::data::Schema,
    prelude::{Key, Row},
};
pub use openrank::OpenRankTable;
use std::fmt::Debug;

pub trait StorageTable: Debug {
    fn name(&self) -> &str;

    fn schema(&self) -> Schema;

    fn scan_data(&self) -> Vec<(Key, Row)>;
}
