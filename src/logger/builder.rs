use bracket_lib::terminal::RGB;

use super::{super::colors::*, append_entry, LogFragment, LOG};

#[derive(Default)]
pub struct Log {
    current_color: RGB,
    fragments: Vec<LogFragment>,
}

impl Log {
    pub fn new() -> Self {
        Self {
            current_color: c(WHITE),
            fragments: Vec::new(),
        }
    }

    pub fn color(mut self, color: &str) -> Self {
        self.current_color = c(color);
        self
    }

    pub fn append<S: ToString>(mut self, text: S) -> Self {
        self.fragments.push(LogFragment {
            color: self.current_color,
            text: text.to_string(),
        });

        self
    }

    pub fn build(self) {
        append_entry(self.fragments)
    }

    pub fn clear() {
        LOG.lock().unwrap().clear();
    }

    pub fn npc<S: ToString>(mut self, text: &S) -> Self {
        self.fragments.push(LogFragment {
            color: c(YELLOW1),
            text: text.to_string(),
        });
        self
    }

    pub fn item<S: ToString>(mut self, text: &S) -> Self {
        self.fragments.push(LogFragment {
            color: c(SHALLOWWATERS5),
            text: text.to_string(),
        });
        self
    }

    pub fn good<S: ToString>(mut self, text: &S) -> Self {
        self.fragments.push(LogFragment {
            color: c(GREEN6),
            text: text.to_string(),
        });
        self
    }

    pub fn bad<S: ToString>(mut self, text: &S) -> Self {
        self.fragments.push(LogFragment {
            color: c(RED3),
            text: text.to_string(),
        });
        self
    }
    pub fn roll<S: ToString>(mut self, text: &S) -> Self {
        self.fragments.push(LogFragment {
            color: c(BLUE1),
            text: text.to_string(),
        });
        self
    }
}
