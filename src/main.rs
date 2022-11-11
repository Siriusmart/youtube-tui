use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};
use tui_additions::framework::{Framework, State};
use youtube_tui::{exit, init, run};

// stuff happening:
//  1. setup the terminal
//  2. run()
//  3. restore the terminal
//  4. unwrap errors
fn main() -> Result<(), Box<dyn Error>> {
    let state = State(Vec::new());
    let mut framework = Framework::new(state);
    init(&mut framework)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run(&mut terminal, &mut framework);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    let exit_res = exit(&framework);

    res?;
    exit_res?;

    Ok(())
}
