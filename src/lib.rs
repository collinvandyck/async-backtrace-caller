use backtrace::Backtrace;
use rustc_demangle::demangle;
use std::path::{Path, PathBuf};

pub struct Frame {
    pub name: String,
    pub path: String,
    pub line: Option<u32>,
}

impl Frame {
    pub fn capture() -> Option<Self> {
        let bt = Backtrace::new();
        bt.frames()
            .into_iter()
            .map(Frame::from)
            .filter(|f| !f.name.starts_with("backtrace::"))
            .skip(2)
            .next()
    }
}

impl From<&backtrace::BacktraceFrame> for Frame {
    fn from(frame: &backtrace::BacktraceFrame) -> Self {
        let mut name = None;
        let mut file = None;
        let mut line = None;
        for symbol in frame.symbols() {
            if name.is_none() {
                name =
                    symbol.name().and_then(|s| s.as_str()).map(|s| format!("{:#?}", demangle(s)));
            }
            if file.is_none() {
                file = symbol.filename();
            }
            if line.is_none() {
                line = symbol.lineno();
            }
        }
        let name = name.unwrap_or_default();
        let file = file.map(|f| short_path(&f)).unwrap_or_default();
        Self { name, path: file, line }
    }
}

fn short_path(p: impl AsRef<Path>) -> String {
    let p = p.as_ref();
    let parts: Vec<_> = p.components().collect();
    parts
        .into_iter()
        .rev()
        .take(3)
        .rev()
        .map(|p| p.as_os_str())
        .fold(PathBuf::from(""), |acc, p| acc.join(p))
        .display()
        .to_string()
}
