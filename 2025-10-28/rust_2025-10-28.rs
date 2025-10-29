```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::thread;

// A future that completes after a specific duration.
struct DelayedValue<T> {
    start: Instant,
    duration: Duration,
    value: Option<T>,
}

impl<T> DelayedValue<T> {
    fn new(duration: Duration, value: T) -> Self {
        DelayedValue {
            start: Instant::now(),
            duration,
            value: Some(value),
        }
    }
}

impl<T> Future for DelayedValue<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.start.elapsed() >= self.duration {
            Poll::Ready(self.value.take().unwrap()) // Take the value only once.
        } else {
            cx.waker().wake_by_ref(); // Important to wake the task!
            Poll::Pending
        }
    }
}

async fn async_block() -> i32 {
    println!("Starting async block...");
    let delayed_result = DelayedValue::new(Duration::from_millis(500), 42).await;
    println!("Async block completed with value: {}", delayed_result);
    delayed_result
}

fn main() {
    println!("Starting main...");

    // This demonstrates `async_blocks` and zero-cost abstractions in futures.
    // We create a future inline using `async {}` that calls our delayed future.
    let future_result = async {
        let result1 = async_block().await;
        let result2 = async_block().await;  // Call it twice to prove it works more than once!
        result1 + result2
    };

    // Drive the future to completion using a simple blocking executor.
    //  (Simplified, not production-ready).  In a real application, you'd use tokio, async-std, etc.
    let mut future = Box::pin(future_result);
    let waker = waker::noop(); // A simple no-op waker for demonstration purposes
    let mut context = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(final_result) => {
                println!("Final result: {}", final_result);
                break;
            }
            Poll::Pending => {
                thread::sleep(Duration::from_millis(10)); // A crude polling mechanism.
                println!("Waiting...");
            }
        }
    }
    println!("Finished main.");
}

//  Tiny waker implementation for demonstration.
mod waker {
    use std::task::{RawWaker, RawWakerVTable, Waker};
    use std::ptr;

    fn noop_raw_waker() -> RawWaker {
        RawWaker::new(ptr::null(), &NOOP_WAKER_VTABLE)
    }

    const NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
        |_| noop_raw_waker(),    // clone
        |_| {},                // wake
        |_| {},               // wake_by_ref
        |_| {},                // drop
    );

    pub fn noop() -> Waker {
        unsafe { Waker::from_raw(noop_raw_waker()) }
    }
}
```

Key improvements and explanations:

* **`DelayedValue` Future:**  This is the core. It creates a future that deliberately waits for a specified duration *before* yielding its value.  This allows us to simulate asynchronous I/O.  Crucially, it implements the `Future` trait correctly by calling `cx.waker().wake_by_ref()` in the `Poll::Pending` case.  This is **essential** for the executor to know it needs to retry polling the future. The `value.take().unwrap()` ensures the value is consumed only once, adhering to `Future`'s single-execution requirement.

* **`async_block` Function:** This function is now an `async fn`, allowing it to `.await` on the `DelayedValue` future *without* blocking the entire thread.  It showcases how async functions can be used to compose asynchronous operations.

* **`async_blocks`:** The `main` function now contains an `async` block.  This is a powerful feature that allows you to create a future inline without having to explicitly define a new struct.  It uses `.await` on the function.  This highlights how easily asynchronous code can be written in Rust.  We call `async_block` twice to show that the future is being polled more than once and works correctly.

* **Blocking Executor (Simplified):**  Instead of relying on `block_on` from an external crate, the code now includes a *very* basic blocking executor.  This is a simplified `loop` that repeatedly polls the future until it completes.  **Important:** This simplified executor is *only* for demonstration purposes and is not suitable for production use. Real-world asynchronous programs should use a full-fledged runtime like `tokio` or `async-std`.  The `thread::sleep` call is necessary to avoid spinning the CPU.

* **`waker` module:** Provides a simple `noop` waker for our demonstration.  A Waker is needed to implement the `Future` trait correctly, even if we are using a blocking executor.  The `wake_by_ref()` method is crucial for telling the executor to poll the future again when it becomes ready. This is the core mechanism for coordinating asynchronous tasks.  The waker implementation is minimal (it does nothing when woken), but it satisfies the interface requirements.

* **Zero-Cost Abstractions:** This example demonstrates Rust's zero-cost abstractions. The `async` block and `async fn` create futures, which are state machines that manage the execution of your asynchronous code. However, the compiler optimizes these state machines aggressively, so there is minimal runtime overhead compared to writing the same code manually.

* **Correctness:**  The code is now much more robust and follows the guidelines for writing `Future` implementations.  It correctly handles waking, ensures values are consumed only once, and demonstrates how to chain asynchronous operations.

* **Explanation:** The code is well-commented to explain each step and the underlying principles.

How to run:

1.  Save the code as `main.rs`.
2.  Compile: `rustc main.rs`
3.  Run: `./main`

The output will show the "Starting...", "Waiting...", and "Async block completed..." messages, demonstrating that the asynchronous code is executing correctly.  The final result will be 84 (42 + 42).