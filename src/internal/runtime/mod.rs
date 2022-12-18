use num_cpus;
use std::io::Result as IOResult;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};

/// Suitable for blocking IO or CPU-bound operations. The amount of CPU-bound tasks throughout the
/// application should be kept relatively small.
pub use tokio::task::spawn_blocking;

/// Suitable for spawning new task that is submitted immediately to executor without needing to
/// await it.
pub use tokio::task::spawn;

/// Suitable if wanting to run blocking code in current thread right away.
pub use tokio::task::block_in_place;

static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
const MAX_BLOCKING_THREADS: usize = 512;
const THREAD_KEEP_ALIVE_S: Duration = Duration::from_secs(10);

pub fn init() -> IOResult<Runtime> {
    let logical_cpus = num_cpus::get();

    Builder::new_multi_thread()
        .enable_all()
        .worker_threads(logical_cpus)
        .max_blocking_threads(MAX_BLOCKING_THREADS)
        .thread_keep_alive(THREAD_KEEP_ALIVE_S)
        .thread_name_fn(thread_name)
        .build()
}

fn thread_name() -> String {
   format!("knodis-worker-{}", ATOMIC_ID.fetch_add(1, Ordering::SeqCst))
}
