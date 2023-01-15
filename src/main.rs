use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};
use tui_additions::framework::{Framework, State};
use youtube_tui::{exit, global::functions::text_command, init, run};

// stuff happening:
//  1. setup the terminal
//  2. run()
//  3. restore the terminal
//  4. unwrap errors
fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>().join(" ");

    if let Some(s) = text_command(&args) {
        println!("{s}");
        return Ok(());
    }

    let state = State(Vec::new());
    let mut framework = Framework::new(state);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = (|| -> Result<(), Box<dyn Error>> {
        init(
            &mut framework,
            &mut terminal,
            if args.is_empty() { None } else { Some(&args) },
        )?;
        run(&mut terminal, &mut framework)?;
        Ok(())
    })();

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
