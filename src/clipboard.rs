use std::string::ToString;
use std::thread;

use arboard::Clipboard;
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use color_eyre::eyre::{Context, Result};

use crate::{debug_log, log};
use crate::log_file::save_clipboard;

/// Start listening for any clipboard changes, saving the new content into the file
pub fn start_clipboard_watcher() {
    debug_log!("Hooking clipboard watcher");
    thread::spawn(|| {
        let watcher = Master::new(Handler::new()).run()
            .context("Could not start Clipboard Watcher");

        if let Err(report) = watcher {
            log!("{:#?}", report);
        }
    });
    debug_log!("Hooked clipboard watcher");
}

fn get_clipboard() -> Result<Clipboard> {
    Clipboard::new().context("Could not get an instance of Clipboard".to_string())
}

fn get_clipboard_content() -> Result<Option<String>> {
    let clipboard = get_clipboard();
    clipboard.map(|mut c| c.get_text().ok())
}

struct Handler {
    previous_clipboard_content: String
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            previous_clipboard_content: String::default(),
        }
    }
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let previous_content = String::from(&self.previous_clipboard_content);
        let content = get_clipboard_content()
            .map(|e| e.unwrap_or_default())
            .unwrap_or_default();

        if previous_content != content {
            let clipboard_log = format!("Copied Text: '{}'", content);
            debug_log!("{}", clipboard_log);
            save_clipboard(&clipboard_log);
            self.previous_clipboard_content = content;
        }
        CallbackResult::Next
    }
}