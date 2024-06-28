#![allow(unused)]

use backtrace::{Backtrace, BacktraceFmt, PrintFmt};
use rustc_demangle::demangle;
use std::{
    env,
    path::{Path, PathBuf},
};
use tracing::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    foo().await;
}

async fn foo() {
    bar().await;
}

async fn bar() {
    caller();
}

struct Frame {
    name: String,
    path: String,
    line: Option<u32>,
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

// where we will capture the backtrace, e.g. Command::new
fn caller() {
    let bt = Backtrace::new();
    let frames = bt.frames();
    if let Some(frame) = bt
        .frames()
        .into_iter()
        .map(Frame::from)
        .filter(|f| !f.name.starts_with("backtrace::"))
        .skip(1)
        .next()
    {
        let Frame { name, path, line } = frame;
        let line = line.map(|u| u.to_string()).unwrap_or_else(|| String::from("??"));
        info!("{path}:{line} - {name}");
    }
}

fn short_path(p: impl AsRef<Path>) -> String {
    let mut p = p.as_ref();
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
