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
    baz().await;
}

async fn baz() {
    info!("baz");
}
