//! # Run:
//! A module containing the functions to start and stop the main App run loop. The exposed "Run"
//! functions allows starting the app based on a root layout.
use std::io::{stdout, Write};
use crossterm::{ExecutableCommand, execute, Result, QueueableCommand,
                cursor::{Hide, Show},
                event::{DisableMouseCapture, EnableMouseCapture},
                terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};


/// Set initial state of the terminal
pub fn initialize_terminal() -> Result<()> {

    enable_raw_mode()?;
    execute!(stdout(), EnableMouseCapture)?;
    stdout().execute(Hide)?;
    stdout().execute(Clear(ClearType::All))?;
    Ok(())
}


/// Set terminal to initial state before exit
pub fn shutdown_terminal() -> Result<()>{

    stdout().queue(DisableMouseCapture)?.queue(Show)?.flush()?;
    stdout().execute(Clear(ClearType::All))?;
    disable_raw_mode()?;
    Ok(())
}