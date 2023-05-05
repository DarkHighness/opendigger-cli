use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

#[derive(
    Debug, Clone, strum::EnumString, strum::EnumIter, strum::IntoStaticStr, strum::AsRefStr,
)]
pub enum Tables {
    #[strum(serialize = "repo")]
    RepoCommonTable,
    #[strum(serialize = "user")]
    UserCommonTable,
}

pub static SUPPORTED_TABLE_NAMES: Lazy<Vec<&'static str>> =
    Lazy::new(Tables::supported_table_names);

impl Tables {
    fn supported_table_names() -> Vec<&'static str> {
        Tables::iter().map(|t| t.into()).collect::<Vec<_>>()
    }
}
