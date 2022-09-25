use crate::global::{
    structs::Message,
    traits::{AsUrlString, ConfigTrait, SearchFilterItem},
};
use serde::{Deserialize, Serialize};
use typemap::Key;

// can be turned into URL Params for the search term with filters
/// Search query & filters
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

/// Search filters, read comments for function docs
#[derive(Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchFilters {
    pub sort: SearchFilterSort,
    pub date: SearchFilterDate,
    pub duration: SearchFilterDuration,
    pub r#type: SearchFilterType,
}

// These functions are used in the search filter item
// Think of the ordering of stuff in the popup menu
impl SearchFilters {
    // This function returns [(Option name, [Options])]
    // Option name is a string like "sory by" and "type"
    // Options are the options that you can select like "relevance" and "upload date" for sorting
    pub fn get_all() -> [(&'static str, Vec<&'static str>); 5] {
        [
            (SearchFilterSort::NAME, SearchFilterSort::ordering()),
            (SearchFilterDate::NAME, SearchFilterDate::ordering()),
            (SearchFilterDuration::NAME, SearchFilterDuration::ordering()),
            (SearchFilterType::NAME, SearchFilterType::ordering()),
            ("Reset filters", vec!["Are you sure?"]),
        ]
    }

    // uses the index of the selected option name to get the index of the selected option
    // aka using the hover location of the left textlist to get the hover location of the right text list
    pub fn get_selected(&self, index: usize) -> usize {
        match index {
            0 => self.sort.selected_index(),
            1 => self.date.selected_index(),
            2 => self.duration.selected_index(),
            3 => self.r#type.selected_index(),
            4 => 0,
            _ => unreachable!(),
        }
    }

    // saves the settings for a filter given the index of the option (among other options) and the index of the selected options
    //                           v left textlist   v right textlist
    pub fn set_index(&mut self, index_at: usize, set_index: usize, message: &mut Message) {
        match index_at {
            0 => self.sort = SearchFilterSort::at_index(set_index),
            1 => self.date = SearchFilterDate::at_index(set_index),
            2 => self.duration = SearchFilterDuration::at_index(set_index),
            3 => self.r#type = SearchFilterType::at_index(set_index),
            4 => {
                if set_index == 0 {
                    self.reset(message)
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn reset(&mut self, message: &mut Message) {
        *self = Self::default();
        *message = Message::Success(String::from("Search filters has been reset"));
    }
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

const SORT_ORDERING: [SearchFilterSort; 4] = [
    SearchFilterSort::Relevance,
    SearchFilterSort::Rating,
    SearchFilterSort::Date,
    SearchFilterSort::Views,
];
const DATE_ORDERING: [SearchFilterDate; 6] = [
    SearchFilterDate::None,
    SearchFilterDate::Hour,
    SearchFilterDate::Day,
    SearchFilterDate::Week,
    SearchFilterDate::Month,
    SearchFilterDate::Year,
];
const DURATION_ORDERING: [SearchFilterDuration; 4] = [
    SearchFilterDuration::None,
    SearchFilterDuration::Short,
    SearchFilterDuration::Medium,
    SearchFilterDuration::Long,
];
const TYPE_ORDERING: [SearchFilterType; 4] = [
    SearchFilterType::All,
    SearchFilterType::Video,
    SearchFilterType::Channel,
    SearchFilterType::Playlist,
];

impl SearchFilterItem for SearchFilterSort {
    const NAME: &'static str = "Sort by";

    fn option_name(&self) -> &'static str {
        match self {
            Self::Relevance => "Relevance",
            Self::Rating => "Rating",
            Self::Date => "Upload date",
            Self::Views => "View count",
        }
    }

    fn ordering() -> Vec<&'static str> {
        SORT_ORDERING
            .iter()
            .map(|item| item.option_name())
            .collect::<Vec<_>>()
    }

    fn selected_index(&self) -> usize {
        match self {
            Self::Relevance => 0,
            Self::Rating => 1,
            Self::Date => 2,
            Self::Views => 3,
        }
    }

    fn at_index(index: usize) -> Self {
        SORT_ORDERING[index]
    }
}

impl SearchFilterItem for SearchFilterDate {
    const NAME: &'static str = "Upload date";

    fn option_name(&self) -> &'static str {
        match self {
            Self::None => "Any date",
            Self::Hour => "Last hour",
            Self::Day => "Today",
            Self::Week => "This week",
            Self::Month => "This month",
            Self::Year => "This year",
        }
    }

    fn ordering() -> Vec<&'static str> {
        DATE_ORDERING
            .iter()
            .map(|item| item.option_name())
            .collect::<Vec<_>>()
    }

    fn selected_index(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Hour => 1,
            Self::Day => 2,
            Self::Week => 3,
            Self::Month => 4,
            Self::Year => 5,
        }
    }

    fn at_index(index: usize) -> Self {
        DATE_ORDERING[index]
    }
}

impl SearchFilterItem for SearchFilterDuration {
    const NAME: &'static str = "Duration";

    fn option_name(&self) -> &'static str {
        match self {
            Self::None => "Any duration",
            Self::Short => "Short (< 4 minutes)",
            Self::Medium => "Medium (4 - 20 minutes)",
            Self::Long => "Long (> 20 minutes)",
        }
    }

    fn ordering() -> Vec<&'static str> {
        DURATION_ORDERING
            .iter()
            .map(|item| item.option_name())
            .collect::<Vec<_>>()
    }

    fn selected_index(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Short => 1,
            Self::Medium => 2,
            Self::Long => 3,
        }
    }

    fn at_index(index: usize) -> Self {
        DURATION_ORDERING[index]
    }
}

impl SearchFilterItem for SearchFilterType {
    const NAME: &'static str = "Type";

    fn option_name(&self) -> &'static str {
        match self {
            Self::All => "Any type",
            Self::Video => "Video",
            Self::Channel => "Channel",
            Self::Playlist => "Playlist",
        }
    }

    fn ordering() -> Vec<&'static str> {
        TYPE_ORDERING
            .iter()
            .map(|item| item.option_name())
            .collect::<Vec<_>>()
    }

    fn selected_index(&self) -> usize {
        match self {
            Self::All => 0,
            Self::Video => 1,
            Self::Channel => 2,
            Self::Playlist => 3,
        }
    }

    fn at_index(index: usize) -> Self {
        TYPE_ORDERING[index]
    }
}
