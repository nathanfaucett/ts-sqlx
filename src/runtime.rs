use std::future::Future;

use once_cell::sync::Lazy;
use tokio::runtime::{self, Runtime};

pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to start Tokio runtime")
});

pub fn block_on<F>(f: F) -> F::Output
where
    F: Future,
{
    RUNTIME.block_on(f)
}
