use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, fs, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use youtube_tui::{
    app::{
        app::App,
        pages::{item_info::ItemInfo, main_menu::MainMenu},
    },
    structs::{Item, Page, Row, RowItem},
    traits::LoadItem,
};

fn main() -> Result<(), Box<dyn Error>> {
    init()?;

    let app = App::default();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app);

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;

    res?;
    exit()?;

    Ok(())
}

fn run_app<B: Backend>(mut terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    loop {
        if app.render {
            ui(&mut terminal, &mut app)?;
        }

        if app.load {
            *app.message.lock().unwrap() = Some(match app.page {
                Page::MainMenu(_) => MainMenu::message(),
                Page::ItemDisplay(_) => ItemInfo::message(),
            });
            terminal.clear()?;
            ui(&mut terminal, &mut app)?;
            let mut new_state = Vec::new();
            for row in app.state.iter() {
                let mut row_vec = Vec::new();
                for row_item in row.items.iter() {
                    match &row_item.item {
                        Item::MainMenu(item) => match item.load_item(&app) {
                            Ok(new) => {
                                row_vec.push(RowItem {
                                    item: Item::MainMenu(*new),
                                    ..*row_item
                                });
                            }
                            Err(e) => {
                                row_vec.push(RowItem {
                                    item: Item::MainMenu(item.clone()),
                                    ..*row_item
                                });
                                *app.message.lock().unwrap() = Some(e.to_string());
                            }
                        },
                        Item::ItemInfo(item) => match item.load_item(&app) {
                            Ok(new) => {
                                row_vec.push(RowItem {
                                    item: Item::ItemInfo(*new),
                                    ..*row_item
                                });
                            }
                            Err(e) => {
                                row_vec.push(RowItem {
                                    item: Item::ItemInfo(item.clone()),
                                    ..*row_item
                                });
                                *app.message.lock().unwrap() = Some(e.to_string());
                            }
                        },
                        _ => {
                            row_vec.push(RowItem {
                                item: row_item.item.clone(),
                                ..*row_item
                            });
                        }
                    }
                }

                new_state.push(Row {
                    items: row_vec,
                    ..*row
                });
            }

            app.state = new_state;

            app.load = false;
            terminal.clear()?;
            ui(&mut terminal, &mut app)?;
        } else {
            match event::read()? {
                event::Event::Key(key) => {
                    if app.selected.is_none() {
                        match key.code {
                            KeyCode::Char('q') => {
                                return Ok(());
                            }
                            KeyCode::Backspace => {
                                let holder = app.pop();
                                app = holder.0;
                                if holder.1 {
                                    terminal.clear()?;
                                }
                                app.render = true;
                                continue;
                            }
                            KeyCode::End => {
                                app.history = Vec::new();
                                continue;
                            }
                            KeyCode::Home => {
                                terminal.clear()?;
                                app.home();
                                continue;
                            }
                            _ => {}
                        }
                    }

                    app = app.key_input(key.code);
                    app.render = true;
                }
                event::Event::Resize(_, _) => {
                    app.render = true;
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    terminal.draw(|mut frame| {
        app.render(&mut frame);
    })?;

    *app.message.lock().unwrap() = None;
    app.render = false;

    Ok(())
}

fn init() -> Result<(), Box<dyn Error>> {
    let home_dir = home::home_dir().expect("Cannot get your home directory");

    let mut dir = home_dir.clone();

    dir.push(".cache");
    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir.push("youtube-tui");
    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir.push("thumbnails");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir = home_dir.clone();

    dir.push(".config");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir.push("youtube-tui");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir = home_dir;

    dir.push(".local");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir.push("share");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir.push("youtube-tui");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir.push("watch_history");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir.push("thumbnails");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir.pop();

    dir.push("info");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }


    Ok(())
}

fn exit() -> Result<(), Box<dyn Error>> {
    let mut dir = home::home_dir().expect("Cannot get your home directory");

    dir.push(".cache");
    dir.push("youtube-tui");
    dir.push("cache");

    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }

    Ok(())
}
