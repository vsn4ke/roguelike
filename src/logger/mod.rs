use bracket_lib::terminal::{TextBuilder, RGB};
use lazy_static::lazy_static;
use std::sync::Mutex;

pub mod builder;

pub struct LogFragment {
    pub color: RGB,
    pub text: String,
}

lazy_static! {
    static ref LOG: Mutex<Vec<Vec<LogFragment>>> = Mutex::new(Vec::new());
}

pub fn append_fragment(fragment: LogFragment) {
    LOG.lock().unwrap().push(vec![fragment]);
}

pub fn append_entry(fragments: Vec<LogFragment>) {
    LOG.lock().unwrap().push(fragments);
}

pub fn log_display() -> TextBuilder {
    let mut buffer = TextBuilder::empty();

    LOG.lock().unwrap().iter().rev().take(12).for_each(|log| {
        log.iter().for_each(|frag| {
            buffer.fg(frag.color);
            buffer.line_wrap(&frag.text);
        });
        buffer.ln();
    });

    buffer
}
