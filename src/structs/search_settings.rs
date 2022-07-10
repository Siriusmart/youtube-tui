use crossterm::event::KeyEvent;

use crate::{
    app::config::{Action, Config},
    widgets::text_list::TextList,
};

#[derive(Debug, Clone)]
pub struct SearchSettings {
    pub sort_by: SearchSettingsSortBy,
    pub date: Option<SearchSettingsDate>,
    pub duartion: Option<SearchSettingsDuration>,
    pub r#type: Option<SearchSettingsType>,

    pub row: bool, // left: true, right: false

    pub text_list: TextList,
    pub select_text_list: TextList,
}

impl Default for SearchSettings {
    fn default() -> Self {
        let mut text_list = TextList::default();
        text_list.items = vec![
            String::from("Sort by"),
            String::from("Date"),
            String::from("Duration"),
            String::from("Type"),
            String::from("Reset All"),
        ];

        let mut out = SearchSettings {
            sort_by: SearchSettingsSortBy::Relevance,
            date: None,
            duartion: None,
            r#type: None,

            row: true,

            text_list,
            select_text_list: TextList::default(),
        };

        out.update_left();

        out
    }
}

impl SearchSettings {
    pub fn to_vec(self) -> Vec<String> {
        let mut vec = vec![self.sort_by.as_url_string()];

        if let Some(date) = self.date {
            vec.push(date.as_url_string());
        }

        if let Some(duration) = self.duartion {
            vec.push(duration.as_url_string());
        }

        if let Some(r#type) = self.r#type {
            vec.push(r#type.as_url_string());
        }

        vec
    }

    pub fn key_input(&mut self, key: KeyEvent, config: &Config) {
        let action = match config.keybindings.0.get(&key) {
            Some(action) => *action,
            None => return,
        };

        match action {
            Action::Up => {
                if self.row {
                    self.text_list.up();
                    self.update_left()
                } else {
                    self.select_text_list.up();

                    match self.text_list.selected {
                        0 => {
                            self.sort_by =
                                SearchSettingsSortBy::index_to_value(self.select_text_list.selected)
                        }
                        1 => {
                            self.date =
                                SearchSettingsDate::index_to_value(self.select_text_list.selected)
                        }
                        2 => {
                            self.duartion = SearchSettingsDuration::index_to_value(
                                self.select_text_list.selected,
                            )
                        }
                        3 => {
                            self.r#type =
                                SearchSettingsType::index_to_value(self.select_text_list.selected)
                        }
                        _ => {}
                    }
                }
            }
            Action::Down => {
                if self.row {
                    self.text_list.down();
                    self.update_left()
                } else {
                    self.select_text_list.down();

                    match self.text_list.selected {
                        0 => {
                            self.sort_by =
                                SearchSettingsSortBy::index_to_value(self.select_text_list.selected)
                        }
                        1 => {
                            self.date =
                                SearchSettingsDate::index_to_value(self.select_text_list.selected)
                        }
                        2 => {
                            self.duartion = SearchSettingsDuration::index_to_value(
                                self.select_text_list.selected,
                            )
                        }
                        3 => {
                            self.r#type =
                                SearchSettingsType::index_to_value(self.select_text_list.selected)
                        }
                        _ => {}
                    }
                }
            }
            Action::Left => {
                self.row = true;
            }
            Action::Right => {
                self.row = false;
            }
            Action::Select => {
                if self.text_list.selected == 4 && !self.row {
                    *self = Self::default();
                    self.text_list.selected = 4;
                    self.update_left();
                } else {
                    self.row = !self.row;
                }
            }
            _ => {}
        }
    }

    pub fn update_left(&mut self) {
        match self.text_list.selected {
            0 => {
                self.select_text_list.items = SearchSettingsSortBy::as_vec();
                self.select_text_list.selected = SearchSettingsSortBy::value_to_index(self.sort_by);
            }
            1 => {
                self.select_text_list.items = SearchSettingsDate::as_vec();
                self.select_text_list.selected = SearchSettingsDate::value_to_index(self.date);
            }
            2 => {
                self.select_text_list.items = SearchSettingsDuration::as_vec();
                self.select_text_list.selected =
                    SearchSettingsDuration::value_to_index(self.duartion);
            }
            3 => {
                self.select_text_list.items = SearchSettingsType::as_vec();
                self.select_text_list.selected = SearchSettingsType::value_to_index(self.r#type);
            }
            4 => {
                self.select_text_list.items = vec![String::from("Reset All")];
                self.select_text_list.selected = 0;
            }
            _ => {}
        }
    }
}

pub trait AsUrlString {
    fn as_url_string(&self) -> String;
}

#[derive(Debug, Clone, Copy)]
pub enum SearchSettingsSortBy {
    Relevance,
    Rating,
    Date,
    ViewCount,
}

impl AsUrlString for SearchSettingsSortBy {
    fn as_url_string(&self) -> String {
        format!(
            "sort={}",
            match self {
                SearchSettingsSortBy::Relevance => String::from("relevance"),
                SearchSettingsSortBy::Rating => String::from("rating"),
                SearchSettingsSortBy::Date => String::from("date"),
                SearchSettingsSortBy::ViewCount => String::from("views"),
            }
        )
    }
}

impl SearchSettingsSortBy {
    pub fn as_vec() -> Vec<String> {
        vec![
            String::from("Relevance"),
            String::from("Rating"),
            String::from("Date"),
            String::from("View Count"),
        ]
    }

    pub fn index_to_value(index: usize) -> SearchSettingsSortBy {
        match index {
            0 => SearchSettingsSortBy::Relevance,
            1 => SearchSettingsSortBy::Rating,
            2 => SearchSettingsSortBy::Date,
            3 => SearchSettingsSortBy::ViewCount,
            _ => unreachable!(),
        }
    }

    pub fn value_to_index(value: SearchSettingsSortBy) -> usize {
        match value {
            SearchSettingsSortBy::Relevance => 0,
            SearchSettingsSortBy::Rating => 1,
            SearchSettingsSortBy::Date => 2,
            SearchSettingsSortBy::ViewCount => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SearchSettingsDate {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl AsUrlString for SearchSettingsDate {
    fn as_url_string(&self) -> String {
        format!(
            "date={}",
            match self {
                SearchSettingsDate::Hour => String::from("hour"),
                SearchSettingsDate::Day => String::from("today"),
                SearchSettingsDate::Week => String::from("week"),
                SearchSettingsDate::Month => String::from("month"),
                SearchSettingsDate::Year => String::from("year"),
            }
        )
    }
}

impl SearchSettingsDate {
    pub fn as_vec() -> Vec<String> {
        vec![
            String::from("No Filter"),
            String::from("Hour"),
            String::from("Day"),
            String::from("Week"),
            String::from("Month"),
            String::from("Year"),
        ]
    }

    pub fn index_to_value(index: usize) -> Option<SearchSettingsDate> {
        match index {
            0 => None,
            1 => Some(SearchSettingsDate::Hour),
            2 => Some(SearchSettingsDate::Day),
            3 => Some(SearchSettingsDate::Week),
            4 => Some(SearchSettingsDate::Month),
            5 => Some(SearchSettingsDate::Year),
            _ => unreachable!(),
        }
    }

    pub fn value_to_index(value: Option<SearchSettingsDate>) -> usize {
        match value {
            None => 0,
            Some(SearchSettingsDate::Hour) => 1,
            Some(SearchSettingsDate::Day) => 2,
            Some(SearchSettingsDate::Week) => 3,
            Some(SearchSettingsDate::Month) => 4,
            Some(SearchSettingsDate::Year) => 5,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SearchSettingsDuration {
    Short,
    Long,
}

impl AsUrlString for SearchSettingsDuration {
    fn as_url_string(&self) -> String {
        format!(
            "duration={}",
            match self {
                SearchSettingsDuration::Short => String::from("short"),
                SearchSettingsDuration::Long => String::from("long"),
            }
        )
    }
}

impl SearchSettingsDuration {
    pub fn as_vec() -> Vec<String> {
        vec![
            String::from("No Filter"),
            String::from("Short"),
            String::from("Long"),
        ]
    }

    pub fn index_to_value(index: usize) -> Option<SearchSettingsDuration> {
        match index {
            0 => None,
            1 => Some(SearchSettingsDuration::Short),
            2 => Some(SearchSettingsDuration::Long),
            _ => unreachable!(),
        }
    }

    pub fn value_to_index(value: Option<SearchSettingsDuration>) -> usize {
        match value {
            None => 0,
            Some(SearchSettingsDuration::Short) => 1,
            Some(SearchSettingsDuration::Long) => 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SearchSettingsType {
    Video,
    Channel,
    Playlist,
    All,
}

impl AsUrlString for SearchSettingsType {
    fn as_url_string(&self) -> String {
        format!(
            "type={}",
            match self {
                SearchSettingsType::Video => String::from("video"),
                SearchSettingsType::Channel => String::from("channel"),
                SearchSettingsType::Playlist => String::from("playlist"),
                SearchSettingsType::All => String::from("all"),
            }
        )
    }
}

impl SearchSettingsType {
    pub fn as_vec() -> Vec<String> {
        vec![
            String::from("No Filter"),
            String::from("Video"),
            String::from("Channel"),
            String::from("Playlist"),
            String::from("All"),
        ]
    }

    pub fn index_to_value(index: usize) -> Option<SearchSettingsType> {
        match index {
            0 => None,
            1 => Some(SearchSettingsType::Video),
            2 => Some(SearchSettingsType::Channel),
            3 => Some(SearchSettingsType::Playlist),
            4 => Some(SearchSettingsType::All),
            _ => unreachable!(),
        }
    }

    pub fn value_to_index(value: Option<SearchSettingsType>) -> usize {
        match value {
            None => 0,
            Some(SearchSettingsType::Video) => 1,
            Some(SearchSettingsType::Channel) => 2,
            Some(SearchSettingsType::Playlist) => 3,
            Some(SearchSettingsType::All) => 4,
        }
    }
}
