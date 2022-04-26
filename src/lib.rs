use std::{error::Error, fmt};

pub struct App {
    pub search_bar: Vec<char>,
    pub state: State,
    pub status: Option<String>,
    pub nav_x: Option<usize>,
    pub nav_y: Option<usize>,

    main_menu_structure: Vec<Vec<MainMenuItem>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            search_bar: Vec::new(),
            state: State::MainMenu {
                selector: Selector::Trending,
                selected: None,
                navigation: None,
            },
            status: None,
            nav_x: None,
            nav_y: None,

            main_menu_structure: vec![
                vec![MainMenuItem::SearchBar],
                vec![
                    MainMenuItem::Trending,
                    MainMenuItem::Popular,
                    MainMenuItem::History,
                ],
                vec![MainMenuItem::Videos, MainMenuItem::VideoDetails],
                vec![MainMenuItem::StatusBar],
            ],
        }
    }

    pub fn up(&mut self) {
        match &mut self.state {
            State::MainMenu {
                selector: _,
                selected: _,
                navigation,
            } => {
                self.nav_y = match self.nav_y {
                    None => Some(0),
                    Some(0) => self.nav_y,
                    Some(y) => Some(y - 1),
                };

                if self.nav_x.is_none() {
                    self.nav_x = Some(0);
                }

                let nav_x = self.nav_x.unwrap();
                let nav_y = self.nav_y.unwrap();

                *navigation = Some(
                    self.main_menu_structure[nav_y][if nav_x
                        >= self.main_menu_structure[nav_y].len()
                    {
                        self.main_menu_structure[nav_y].len() - 1
                    } else {
                        nav_x
                    }],
                );
            }
        }
    }

    pub fn down(&mut self) {
        match &mut self.state {
            State::MainMenu {
                selector: _,
                selected: _,
                navigation,
            } => {
                self.nav_y = match self.nav_y {
                    None => Some(self.main_menu_structure.len() - 1),
                    Some(y) => {
                        if y < self.main_menu_structure.len() - 1 {
                            Some(y + 1)
                        } else {
                            Some(y)
                        }
                    }
                };

                if self.nav_x.is_none() {
                    self.nav_x = Some(0);
                }

                let nav_x = self.nav_x.unwrap();
                let nav_y = self.nav_y.unwrap();

                *navigation = Some(
                    self.main_menu_structure[nav_y][if nav_x
                        >= self.main_menu_structure[nav_y].len()
                    {
                        self.main_menu_structure[nav_y].len() - 1
                    } else {
                        nav_x
                    }],
                );
            }
        }
    }

    pub fn left(&mut self) {
        match &mut self.state {
            State::MainMenu {
                selector: _,
                selected: _,
                navigation,
            } => {
                if self.nav_y.is_none() {
                    self.nav_y = Some(0);
                }
                let nav_y = self.nav_y.unwrap();

                self.nav_x = match self.nav_x {
                    None => Some(0),
                    Some(0) => Some(0),
                    Some(x) => {
                        if x < self.main_menu_structure[nav_y].len() {
                            Some(x - 1)
                        } else if self.main_menu_structure[nav_y].len() == 1 {
                            Some(0)
                        } else {
                            Some(self.main_menu_structure[nav_y].len() - 2)
                        }
                    }
                };
                

                let nav_x = self.nav_x.unwrap();

                *navigation = Some(
                    self.main_menu_structure[nav_y][if nav_x
                        >= self.main_menu_structure[nav_y].len()
                    {
                        self.main_menu_structure[nav_y].len() - 1
                    } else {
                        nav_x
                    }],
                );
            }
        }
    }

    pub fn right(&mut self) {
        match &mut self.state {
            State::MainMenu {
                selector: _,
                selected: _,
                navigation,
            } => {
                if self.nav_y.is_none() {
                    self.nav_y = Some(0);
                }

                let nav_y = self.nav_y.unwrap();

                self.nav_x = match self.nav_x {
                    None => Some(0),
                    Some(x) => {
                        if x < self.main_menu_structure[nav_y].len() - 1 {
                            Some(x + 1)
                        } else {
                            Some(x)
                        }
                    }
                };

                let nav_x = self.nav_x.unwrap();

                *navigation = Some(
                    self.main_menu_structure[nav_y][if nav_x
                        >= self.main_menu_structure[nav_y].len()
                    {
                        self.main_menu_structure[nav_y].len() - 1
                    } else {
                        nav_x
                    }],
                );
            }
        }
    }

    pub fn enter(&mut self) {
        match &mut self.state {
            State::MainMenu {
                selector,
                selected,
                navigation,
            } => {
                *selected = *navigation;
                match selected.unwrap() {
                    MainMenuItem::Trending => *selector = Selector::Trending,
                    MainMenuItem::Popular => *selector = Selector::Popular,
                    MainMenuItem::History => *selector = Selector::History,
                    _ => {}
                };
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    MainMenu {
        selector: Selector,
        navigation: Option<MainMenuItem>,
        selected: Option<MainMenuItem>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainMenuItem {
    SearchBar,
    Status,
    Trending,
    Popular,
    History,
    Videos,
    VideoDetails,
    StatusBar,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Selector {
    Trending,
    Popular,
    History,
}
