use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use youtube_tui::{
    app::app::{App, Item, Row, RowItem},
    traits::LoadItem,
};

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

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

    res
}

fn run_app<B: Backend>(mut terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    let mut render = true;
    app.message = Some(String::from("Loading videos..."));
    loop {
        if render {
            ui(&mut terminal, &mut app)?;
            app.message = None;
            render = false;
        }

        if app.load {
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
                            _ => {
                                row_vec.push(RowItem {
                                    item: Item::MainMenu(item.clone()),
                                    ..*row_item
                                });
                                app.message =
                                    Some(String::from("An error occurred while loading videos"));
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
            render = true;
        } else {
            match event::read()? {
                event::Event::Key(key) => {
                    if KeyCode::Char('q') == key.code && app.selected.is_none() {
                        return Ok(());
                    } else {
                        app = app.key_input(key.code);
                        render = true;
                    }
                }
                event::Event::Resize(_, _) => {
                    render = true;
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

    Ok(())
}
