```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

// A future that only completes after a certain duration has passed.
struct Delay {
    deadline: Instant,
}

impl Delay {
    fn new(duration: Duration) -> Self {
        Self { deadline: Instant::now() + duration }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let now = Instant::now();
        if now >= self.deadline {
            Poll::Ready(())
        } else {
            // Important part: Wakes the waker when the deadline is near.
            let waker = cx.waker().clone();
            let deadline = self.deadline;
            thread::spawn(move || {
                let time_to_wake = deadline - Instant::now();
                thread::sleep(time_to_wake);
                waker.wake(); // Signal the future to retry polling
            });
            Poll::Pending
        }
    }
}


#[tokio::main]
async fn main() {
    println!("Starting the delay...");

    // Create a delay future that waits for 2 seconds.
    let delay = Delay::new(Duration::from_secs(2));

    // Await the delay future.  The clever part is how this works without
    // requiring a proper executor.
    delay.await;

    println!("Delay finished!");
}
```

**Explanation and Cleverness:**

* **`Delay` Future:**  This custom `Future` is the heart of the example.  It represents an asynchronous operation that completes after a specified duration.

* **Manual Waking:** The key to the "cleverness" is how `Delay` manages its `Poll` state *without* relying on a traditional asynchronous executor like `tokio` or `async-std` for the core sleeping logic.  When `poll` is called and the delay hasn't elapsed, it does the following:
    * Spawns a new thread.
    * The thread calculates the remaining time until the deadline.
    * The thread sleeps for the calculated duration.
    * *Crucially*: When the sleep is complete, the thread calls `waker.wake()`.  This `wake()` call is the mechanism that signals the `tokio` runtime to re-poll the `Delay` future.  The `waker` is obtained from the `Context` parameter passed to `poll`.

* **Executor-less Async (Sort Of):**  While we're *using* `tokio::main` for convenience, the *core* `Delay` future doesn't directly rely on tokio timers. The sleep is performed in a separate thread, and the wake is done by hand.

* **Why is this Clever?**
    * **Minimal Dependencies:**  The code demonstrates a rudimentary asynchronous mechanism with minimal dependencies. It shows how to implement a basic form of asynchronous I/O or timing without needing a complex executor.
    * **Under the Hood Understanding:** It illustrates how the `Future` trait and the `Waker` API work at a low level, providing insight into how asynchronous execution is orchestrated behind the scenes.
    * **Educational:**  It serves as a good educational tool for understanding the building blocks of asynchronous programming in Rust.

**How to Run:**

1.  Make sure you have Rust installed.
2.  Save the code as `src/main.rs` (or any other `.rs` file).
3.  Create a `Cargo.toml` file with the following content:

    ```toml
    [package]
    name = "delay_example"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    tokio = { version = "1", features = ["full"] }  # Needed for tokio::main
    ```

4.  Run the program using `cargo run`.

You'll see the "Starting the delay..." message, followed by a pause of approximately 2 seconds, and then "Delay finished!".