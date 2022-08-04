use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, fs, io, sync::mpsc::channel};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use youtube_tui::{
    app::{app::App, config::Action},
    structs::{Row, RowItem, State, MessageText},
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
            if app.term_clear {
                terminal.clear()?;
                app.term_clear = false;
            }
            let hold = ui(&mut terminal, app);
            app = hold.0;
            hold.1?;
        }

        if app.load {
            app.message = MessageText::Text(app.page.message(&app.config));
            let mut watch_history = app.watch_history.clone();
            terminal.clear()?;
            let hold = ui(&mut terminal, app);
            app = hold.0;
            hold.1?;
            let mut new_state = Vec::new();
            for row in app.state.0.iter() {
                let mut row_vec = Vec::new();
                for row_item in row.items.iter() {
                    match row_item.item.load_item(&app, &mut watch_history) {
                        Ok(new) => {
                            row_vec.push(RowItem {
                                item: new,
                                ..*row_item
                            });
                        }
                        Err(e) => {
                            row_vec.push(RowItem {
                                item: row_item.item.clone(),
                                ..*row_item
                            });
                            app.message = MessageText::Text(e.to_string());
                        }
                    }
                }

                new_state.push(Row {
                    items: row_vec,
                    ..*row
                });
            }

            app.watch_history = watch_history;
            app.state = State(new_state);
            app.selectable = App::selectable(&app.state);

            app.load = false;
            terminal.clear()?;
            let hold = ui(&mut terminal, app);
            app = hold.0;

            let selected = app.page_default();

            app.selected = selected;
            app.hover = selected;
            app.render = true;
        } else {
            match event::read()? {
                event::Event::Key(key) => {
                    if app.selected.is_none() {
                        let action = match app.config.keybindings.0.get(&key) {
                            Some(action) => *action,
                            None => continue,
                        };
                        match action {
                            Action::Exit => {
                                return Ok(());
                            }
                            Action::Back => {
                                let holder = app.pop();
                                app = holder.0;
                                if holder.1 {
                                    terminal.clear()?;
                                }
                                app.render = true;
                                continue;
                            }
                            Action::ClearHistory => {
                                app.history = Vec::new();
                                continue;
                            }
                            Action::Home => {
                                terminal.clear()?;
                                app.home();
                                continue;
                            }
                            _ => {}
                        }
                    }

                    app = app.key_input(key);
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

fn ui<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> (App, Result<(), Box<dyn Error>>) {
    let (send, recv) = channel();
    let res = terminal.draw(|mut frame| {
        let app = app.render(&mut frame);
        send.send(app).unwrap();
    });

    app = recv.recv().unwrap();

    app.message = MessageText::None;
    app.render = false;

    match res {
        Ok(_) => (app, Ok(())),
        Err(e) => (app, Err(Box::new(e))),
    }
}

fn init() -> Result<(), Box<dyn Error>> {
    let home_dir = home::home_dir().expect("Cannot get your home directory");

    let mut dir = home_dir.clone();

    dir.push(".cache");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    dir.push("youtube-tui");
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }

    fs::create_dir(&dir)?;

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

    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }

    Ok(())
}
