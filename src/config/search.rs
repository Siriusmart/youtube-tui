use crate::{config::ConfigTrait, global::traits::asurlstring::AsUrlString};
use serde::{Deserialize, Serialize};
use typemap::Key;

// can be turned into URL Params for the search term with filters
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Search {
    pub query: String,
    pub filters: SearchFilters,
}

impl Key for Search {
    type Value = Self;
}

impl ConfigTrait for Search {
    const LABEL: &'static str = "search";
}

impl ToString for Search {
    fn to_string(&self) -> String {
        vec![
            format!("q={}", self.query),
            self.filters.sort.as_url_string(),
            self.filters.date.as_url_string(),
            self.filters.duration.as_url_string(),
            self.filters.r#type.as_url_string(),
        ]
        .join("&")
    }
}

// Search filters

#[derive(Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchFilters {
    pub sort: SearchFilterSort,
    pub date: SearchFilterDate,
    pub duration: SearchFilterDuration,
    pub r#type: SearchFilterType,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SearchFilterSort {
    Relevance,
    Rating,
    Date,
    Views,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SearchFilterDate {
    None,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SearchFilterDuration {
    None,
    Short,
    Medium,
    Long,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SearchFilterType {
    All,
    Video,
    Channel,
    Playlist,
}

impl Default for SearchFilterSort {
    fn default() -> Self {
        Self::Relevance
    }
}

impl Default for SearchFilterDate {
    fn default() -> Self {
        Self::None
    }
}

impl Default for SearchFilterDuration {
    fn default() -> Self {
        Self::None
    }
}

impl Default for SearchFilterType {
    fn default() -> Self {
        Self::All
    }
}

impl ToString for SearchFilterSort {
    fn to_string(&self) -> String {
        match self {
            Self::Relevance => String::from("relevance"),
            Self::Rating => String::from("rating"),
            Self::Date => String::from("date"),
            Self::Views => String::from("views"),
        }
    }
}

impl ToString for SearchFilterDate {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::Hour => String::from("hour"),
            Self::Day => String::from("day"),
            Self::Week => String::from("week"),
            Self::Month => String::from("month"),
            Self::Year => String::from("year"),
        }
    }
}

impl ToString for SearchFilterDuration {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::Short => String::from("short"),
            Self::Medium => String::from("medium"),
            Self::Long => String::from("long"),
        }
    }
}

impl ToString for SearchFilterType {
    fn to_string(&self) -> String {
        match self {
            Self::All => String::from("all"),
            Self::Video => String::from("video"),
            Self::Channel => String::from("channel"),
            Self::Playlist => String::from("playlist"),
        }
    }
}

impl AsUrlString for SearchFilterSort {
    const TAG: &'static str = "sort";
}

impl AsUrlString for SearchFilterDate {
    const TAG: &'static str = "date";
}

impl AsUrlString for SearchFilterDuration {
    const TAG: &'static str = "duration";
}

impl AsUrlString for SearchFilterType {
    const TAG: &'static str = "type";
}
