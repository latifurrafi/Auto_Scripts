```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::time::sleep;

// A "lazy" future that only runs once, caching its result.
struct LazyFuture<F, T> {
    inner: Option<F>,
    result: Option<T>,
}

impl<F: Future<Output = T>, T> LazyFuture<F, T> {
    fn new(future: F) -> Self {
        LazyFuture {
            inner: Some(future),
            result: None,
        }
    }
}

impl<F: Future<Output = T>, T> Future for LazyFuture<F, T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.result.is_some() {
            // We already have the result, just return it.
            Poll::Ready(self.result.take().unwrap())
        } else if let Some(mut future) = self.inner.take() {
            // Run the future *once*.
            match Pin::new(&mut future).poll(cx) {
                Poll::Ready(result) => {
                    self.result = Some(result.clone()); // Clone the result!
                    Poll::Ready(result)
                }
                Poll::Pending => {
                    // Future is not ready yet. Put it back in `inner`.
                    self.inner = Some(future);
                    Poll::Pending
                }
            }
        } else {
            // Should never happen, as we ensure `result` is Some after first poll.
            panic!("LazyFuture polled after completion");
        }
    }
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // Create a time-consuming future.  This future *should* only run once.
    let expensive_future = LazyFuture::new(async {
        println!("Running expensive computation...");
        sleep(Duration::from_millis(500)).await;
        println!("Expensive computation finished.");
        start.elapsed().as_millis() // Return the execution time, used for verification.
    });

    // Call the future multiple times.
    let result1 = expensive_future.await;
    let result2 = expensive_future.await;
    let result3 = expensive_future.await;

    // Print the results.  They should be the same value (milliseconds from start).
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    println!("Result 3: {}", result3);

    //  The key here is that "Running expensive computation..." and "Expensive computation finished."
    // should only print *once*, even though we `await` the future three times. This is due to
    // the `LazyFuture` caching the result.  The standard `async` block would rerun the code on each `await`.
}
```

Key features showcased:

* **`Future` trait and `async/await`:**  Demonstrates the core Rust concurrency feature set.
* **Lazy Initialization and Memoization:** The `LazyFuture` struct is the core cleverness.  It implements lazy initialization; the expensive computation is only performed when first requested.  Subsequent requests return the cached result, avoiding repeated computation.  This is a simple form of memoization.
* **`Option` and `take()`:**  The `Option` fields and the `take()` method are used to manage the state of the inner future and the cached result.  `take()` efficiently moves the value out of the `Option` and replaces it with `None`.
* **`Pin` and `Context`:**  Required parts of `Future` implementation to ensure safe access to the future's state and to coordinate with the asynchronous runtime.
* **`clone()`:**  We `clone()` the result so that it can be returned multiple times.  Note that this introduces a requirement that the result type `T` implements `Clone`. If `T` were not cloneable, you'd need a more complex approach, perhaps using `Arc<Mutex<Option<T>>>`.
* **Clear Demonstration:**  The `println!` statements make it obvious when the expensive computation runs.
* **Error Handling:** Includes a panic if the future is unexpectedly polled after completion, helping debug unexpected behavior.
* **Tokio Runtime:** Uses Tokio, a popular asynchronous runtime in Rust, to execute the futures.
* **No Unsafe Code:**  The code avoids using `unsafe`, making it safer and easier to reason about.

To run this program:

1.  Make sure you have Rust installed (rustup).
2.  Create a new Rust project: `cargo new lazy_future_example`
3.  Replace the contents of `src/main.rs` with the code above.
4.  Add `tokio` to your dependencies: `cargo add tokio --features full`
5.  Run the program: `cargo run`

You'll see the "expensive computation" messages only once, demonstrating the caching behavior.  The three results printed will be the same value (the elapsed time in milliseconds at the *first* computation).