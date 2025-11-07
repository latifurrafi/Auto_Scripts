```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    // Atomically track how many threads think they are the "winner".
    static WINNERS: AtomicUsize = AtomicUsize::new(0);

    let mut threads = vec![];
    for i in 0..10 {
        threads.push(thread::spawn(move || {
            // Each thread tries to be the "winner" by atomically incrementing the counter.
            // But only the FIRST one to hit a certain number (in this case, 1) truly wins.
            // This demonstrates how atomic operations can be used to create a simple race condition
            // where only one thread is intended to succeed, even if multiple try.

            if WINNERS.fetch_add(1, Ordering::Relaxed) == 0 {
                println!("Thread {} claims to be the Winner!", i);
                WINNERS.store(1, Ordering::Relaxed); // Ensure the winning thread sets it to 1
            } else {
                println!("Thread {} lost the race.", i);
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Final winner count: {}", WINNERS.load(Ordering::Relaxed));
    // The final winner count will be 1, even though multiple threads *thought* they won briefly.
}
```

**Key Features Showcased:**

*   **Atomic Operations (`AtomicUsize` and `Ordering::Relaxed`):** The program uses `AtomicUsize` to maintain a counter (`WINNERS`) that can be safely accessed and modified by multiple threads concurrently.  `Ordering::Relaxed` is used for simplicity. While it might not be appropriate for complex multi-threaded scenarios where stricter ordering is crucial, it's sufficient to illustrate the basic concept of atomic access.
*   **Race Condition (Controlled):** The program intentionally creates a controlled race condition. Multiple threads simultaneously try to increment `WINNERS` and claim to be the "winner."  However, due to the atomic nature of the operation, only one thread's increment effectively sets the value to 1 *first* (or appears to do so), becoming the "true" winner.
*   **Threads (`std::thread`):** The program utilizes multiple threads to simulate concurrent access to the shared atomic counter.
*   **`fetch_add` vs `store`**: The program demonstrates the difference between `fetch_add` (get and increment in one atomic operation) and `store` (directly setting a value). The `fetch_add` enables the threads to try and get the `WINNERS` counter before anyone else while the winning thread uses `store` to set it to `1` at the end.

**Why this is unique/clever:**

*   **Simple and Illustrative:** The program is short and easy to understand, yet it effectively demonstrates the core concept of atomic operations and race conditions in a multi-threaded environment.
*   **Highlights a Subtle Point:**  It showcases that even with atomic operations, you can create race conditions where multiple threads might momentarily *believe* they have succeeded, even if only one ultimately does.
*   **Real-World Relevance:** This pattern (atomically claiming a "resource" or "lock") is a common pattern in concurrent programming, making the example practical and relatable.

**To Run the Program:**

1.  Save the code as `main.rs`.
2.  Compile: `rustc main.rs`
3.  Run: `./main`

You'll see that only one thread prints the "Winner!" message, and the final winner count is 1, even though multiple threads were racing to increment the counter.