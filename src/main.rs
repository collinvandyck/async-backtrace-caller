use async_backtrace_caller::Frame;
use tracing::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    caller(true);
    foo().await;
}

async fn foo() {
    bar().await;
}

async fn bar() {
    caller(true);
}

// where we will capture the backtrace, e.g. Command::new
fn caller(print: bool) {
    if let Some(Frame { name, path, line }) = Frame::capture() {
        let line = line.map(|u| u.to_string()).unwrap_or_else(|| String::from("??"));
        if print {
            info!("{path}:{line} - {name}");
        }
    }
}
