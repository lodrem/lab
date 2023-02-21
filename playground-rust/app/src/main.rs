use tokio::runtime::Builder;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tracing::instrument]
fn foo() {
    info!("from foo");
    baz(42);
    fib(5);
}

#[tracing::instrument(fields(magic = 42), ret)]
fn bar() {}

#[tracing::instrument]
fn baz(n: usize) {
    inner::quo(n);
}

mod inner {
    use tracing::debug;

    #[tracing::instrument]
    pub fn quo(n: usize) {
        debug!("from inner::quo {n}");
    }
}

#[tracing::instrument(ret)]
fn fib(n: usize) -> usize {
    let mut seq = (1, 1);
    for _ in 1..n {
        seq = (seq.1, seq.0 + seq.1);
    }
    seq.0
}

fn main() {
    let rt = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .max_blocking_threads(4)
        .build()
        .expect("build threaded runtime");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "app=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    rt.block_on(async move {
        info!("hello, world");
        foo();
        bar();
    });
}
