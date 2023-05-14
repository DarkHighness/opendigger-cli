mod active_dates_and_times;
mod activity;
mod attention;
mod bus_factor;
mod change_request;
mod code_change_lines;
mod inactive_contributors;
mod issue;
mod network;
mod new_contributors;
mod openrank;
mod participants;
mod stars;
mod technical_fork;
mod types;

use std::{ops::Deref, sync::Arc};

pub use activity::{ACTIVITY_TABLE_NAME, ACTIVITY_TABLE_SCHEMA};
pub use attention::{ATTENTION_TABLE_NAME, ATTENTION_TABLE_SCHEMA};
use gluesql::{
    core::data::Schema,
    prelude::{Key, Row},
};

pub use active_dates_and_times::{
    ACTIVE_DATES_AND_TIMES_TABLE_NAME, ACTIVE_DATES_AND_TIMES_TABLE_SCHEMA,
};
use lazy_static::lazy_static;

pub use openrank::{OPENRANK_TABLE_NAME, OPENRANK_TABLE_SCHEMA};
pub use stars::{STARS_TABLE_NAME, STARS_TABLE_SCHEMA};
pub use technical_fork::{TECHNICAL_FORK_TABLE_NAME, TECHNICAL_FORK_TABLE_SCHEMA};

pub use types::{DataFetchError, TableEntry, TableOwner, TableType};

pub use bus_factor::{
    BUS_FACTOR_DETAIL_TABLE_NAME, BUS_FACTOR_DETAIL_TABLE_SCHEMA, BUS_FACTOR_TABLE_NAME,
    BUS_FACTOR_TABLE_SCHEMA,
};
pub use change_request::{
    CHANGE_REQUESTS_ACCEPTED_TABLE_NAME, CHANGE_REQUESTS_ACCEPTED_TABLE_SCHEMA,
    CHANGE_REQUESTS_OPEN_TABLE_NAME, CHANGE_REQUESTS_OPEN_TABLE_SCHEMA,
    CHANGE_REQUESTS_REVIEWS_TABLE_NAME, CHANGE_REQUESTS_REVIEWS_TABLE_SCHEMA,
    CHANGE_REQUESTS_TABLE_NAME, CHANGE_REQUESTS_TABLE_SCHEMA, CHANGE_REQUEST_AGE_TABLE_NAME,
    CHANGE_REQUEST_AGE_TABLE_SCHEMA, CHANGE_REQUEST_RESOLUTION_DURATION_TABLE_NAME,
    CHANGE_REQUEST_RESOLUTION_DURATION_TABLE_SCHEMA, CHANGE_REQUEST_RESPONSE_TIME_TABLE_NAME,
    CHANGE_REQUEST_RESPONSE_TIME_TABLE_SCHEMA,
};
pub use code_change_lines::{
    CODE_CHANGE_LINES_ADD_TABLE_NAME, CODE_CHANGE_LINES_ADD_TABLE_SCHEMA,
    CODE_CHANGE_LINES_REMOVE_TABLE_NAME, CODE_CHANGE_LINES_REMOVE_TABLE_SCHEMA,
    CODE_CHANGE_LINES_SUM_TABLE_NAME, CODE_CHANGE_LINES_SUM_TABLE_SCHEMA,
    CODE_CHANGE_LINES_TABLE_NAME, CODE_CHANGE_LINE_TABLE_SCHEMA,
};
pub use inactive_contributors::{
    INACTIVE_CONTRIBUTORS_TABLE_NAME, INACTIVE_CONTRIBUTORS_TABLE_SCHEMA,
};
pub use issue::{
    ISSUES_CLOSED_TABLE_NAME, ISSUES_CLOSED_TABLE_SCHEMA, ISSUES_NEW_TABLE_NAME,
    ISSUES_NEW_TABLE_SCHEMA, ISSUES_TABLE_NAME, ISSUES_TABLE_SCHEMA, ISSUE_AGE_TABLE_NAME,
    ISSUE_AGE_TABLE_SCHEMA, ISSUE_COMMENTS_TABLE_NAME, ISSUE_COMMENTS_TABLE_SCHEMA,
    ISSUE_RESOLUTION_DURATION_TABLE_NAME, ISSUE_RESOLUTION_DURATION_TABLE_SCHEMA,
    ISSUE_RESPONSE_TIME_TABLE_NAME, ISSUE_RESPONSE_TIME_TABLE_SCHEMA,
};
pub use network::{
    DEVELOPER_NETWORK_TABLE_NAME, DEVELOPER_NETWORK_TABLE_SCHEMA, REPO_NETWORK_TABLE_NAME,
    REPO_NETWORK_TABLE_SCHEMA,
};
pub use new_contributors::{
    NEW_CONTRIBUTORS_DETAIL_TABLE_NAME, NEW_CONTRIBUTORS_DETAIL_TABLE_SCHEMA,
    NEW_CONTRIBUTORS_TABLE_NAME, NEW_CONTRIBUTORS_TABLE_SCHEMA,
};
pub use participants::{PARTICIPANTS_TABLE_NAME, PARTICIPANTS_TABLE_SCHEMA};

lazy_static! {
    pub static ref ALL_SCHEMAS: Vec<Schema> = vec![
        ACTIVITY_TABLE_SCHEMA.clone(),
        ATTENTION_TABLE_SCHEMA.clone(),
        OPENRANK_TABLE_SCHEMA.clone(),
        ACTIVE_DATES_AND_TIMES_TABLE_SCHEMA.clone(),
        STARS_TABLE_SCHEMA.clone(),
        TECHNICAL_FORK_TABLE_SCHEMA.clone(),
        PARTICIPANTS_TABLE_SCHEMA.clone(),
        NEW_CONTRIBUTORS_TABLE_SCHEMA.clone(),
        NEW_CONTRIBUTORS_DETAIL_TABLE_SCHEMA.clone(),
        INACTIVE_CONTRIBUTORS_TABLE_SCHEMA.clone(),
        BUS_FACTOR_TABLE_SCHEMA.clone(),
        BUS_FACTOR_DETAIL_TABLE_SCHEMA.clone(),
        ISSUES_TABLE_SCHEMA.clone(),
        ISSUES_NEW_TABLE_SCHEMA.clone(),
        ISSUES_CLOSED_TABLE_SCHEMA.clone(),
        ISSUE_COMMENTS_TABLE_SCHEMA.clone(),
        ISSUE_RESPONSE_TIME_TABLE_SCHEMA.clone(),
        ISSUE_RESOLUTION_DURATION_TABLE_SCHEMA.clone(),
        ISSUE_AGE_TABLE_SCHEMA.clone(),
        CODE_CHANGE_LINE_TABLE_SCHEMA.clone(),
        CODE_CHANGE_LINES_ADD_TABLE_SCHEMA.clone(),
        CODE_CHANGE_LINES_REMOVE_TABLE_SCHEMA.clone(),
        CODE_CHANGE_LINES_SUM_TABLE_SCHEMA.clone(),
        CHANGE_REQUESTS_TABLE_SCHEMA.clone(),
        CHANGE_REQUESTS_OPEN_TABLE_SCHEMA.clone(),
        CHANGE_REQUESTS_ACCEPTED_TABLE_SCHEMA.clone(),
        CHANGE_REQUESTS_REVIEWS_TABLE_SCHEMA.clone(),
        CHANGE_REQUEST_RESPONSE_TIME_TABLE_SCHEMA.clone(),
        CHANGE_REQUEST_RESOLUTION_DURATION_TABLE_SCHEMA.clone(),
        CHANGE_REQUEST_AGE_TABLE_SCHEMA.clone(),
        DEVELOPER_NETWORK_TABLE_SCHEMA.clone(),
        REPO_NETWORK_TABLE_SCHEMA.clone(),
    ];
}

#[derive(Debug)]
pub struct InnerStorageTable {
    r#type: TableType,
    items: Vec<(Key, Row)>,
}

#[derive(Debug, Clone)]
pub struct StorageTable {
    inner: Arc<InnerStorageTable>,
}

impl Deref for StorageTable {
    type Target = InnerStorageTable;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl InnerStorageTable {
    pub fn name(&self) -> &str {
        self.r#type.as_ref()
    }

    pub fn schema(&self) -> Schema {
        ALL_SCHEMAS
            .iter()
            .find(|schema| schema.table_name == self.name())
            .unwrap()
            .clone()
    }

    pub fn items(&self) -> Vec<(Key, Row)> {
        self.items.clone()
    }
}

impl StorageTable {
    pub fn new(r#type: TableType, items: Vec<(Key, Row)>) -> Self {
        Self {
            inner: Arc::new(InnerStorageTable { r#type, items }),
        }
    }
}
