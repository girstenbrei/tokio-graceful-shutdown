use std::future::Future;
use tokio::task::JoinHandle;

#[cfg(not(all(tokio_unstable, feature = "tracing")))]
#[track_caller]
pub(crate) fn spawn<F: Future + Send + 'static>(f: F, _name: &str) -> JoinHandle<F::Output>
where
    <F as Future>::Output: Send + 'static,
{
    tokio::spawn(f)
}

#[cfg(all(tokio_unstable, feature = "tracing"))]
#[track_caller]
pub(crate) fn spawn<F: Future + Send + 'static>(f: F, name: &str) -> JoinHandle<F::Output>
where
    <F as Future>::Output: Send + 'static,
{
    tokio::task::Builder::new()
        .name(name)
        .spawn(f)
        .expect("a task should be spawned")
}
