use async_trait::async_trait;
use gluesql::core::store::{Index, IndexMut};

use super::Storage;

#[async_trait(? Send)]
impl Index for Storage {}

#[async_trait(? Send)]
impl IndexMut for Storage {}
