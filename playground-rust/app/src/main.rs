use tokio::runtime::Builder;
use tracing::{info, span, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tracing::instrument]
fn foo() {
    info!("from foo");
    baz(42);
    fib(5);
}

#[tracing::instrument(level = "trace", fields(magic = 42), ret)]
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

thread_local! {
    static CONTEXT: u64 = {
        println!("Hello, world");
        panic!("fuck");
        42
    }
}

fn main() {
    let rt = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .max_blocking_threads(4)
        .build()
        .expect("build threaded runtime");

    {
        let filter = EnvFilter::from_default_env()
            .add_directive("app=trace".parse().expect("valid directive"));
        let format = tracing_subscriber::fmt::layer().pretty();

        tracing_subscriber::registry()
            .with(filter)
            .with(format)
            .init();
    }

    rt.block_on(async move {
        let span = span!(Level::INFO, "runtime main");
        let _enter = span.enter();

        info!("hello, world");
        foo();
        bar();
    });
}
