```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

// Showcase:  Interior Mutability with `AtomicUsize` in a read-only context!

static COUNTER: AtomicUsize = AtomicUsize::new(0); // Globally accessible counter

fn increment_counter() {
    COUNTER.fetch_add(1, Ordering::SeqCst); // Increment atomically
}

fn read_counter() -> usize {
    COUNTER.load(Ordering::SeqCst) // Read atomically
}


fn main() {
    let num_threads = 10;
    let mut threads = vec![];

    println!("Starting threads.  Counter initially: {}", read_counter());

    for i in 0..num_threads {
        threads.push(thread::spawn(move || {
            println!("Thread {} running...", i);
            for _ in 0..1000 {
                increment_counter();
            }
            println!("Thread {} finished.", i);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("All threads finished.  Final counter value: {}", read_counter());
}
```

**Explanation and Cleverness:**

1. **Interior Mutability via `AtomicUsize`:** The core of the "cleverness" lies in the `COUNTER` being a `static AtomicUsize`.  `AtomicUsize` provides *interior mutability*.  This means that even though `COUNTER` is declared as `static`, which generally implies immutability, we can *still modify its internal value* using the atomic operations like `fetch_add` and `load`.

2. **No Explicit `mut`:** Notice that nowhere do we declare `COUNTER` as `mut`. It's a `static` variable, but because it *contains* the mutability within the `AtomicUsize` type, we don't need a `mut` declaration for the variable itself.  This is a subtle but powerful feature of Rust.

3. **Global State in a Safe Way:** The `static` allows us to have global, shared state.  Without `AtomicUsize`, managing mutable global state across threads would be rife with data races and undefined behavior.  `AtomicUsize` guarantees safe, concurrent access to this shared counter, using atomic operations that prevent race conditions.  `Ordering::SeqCst` provides a strong memory ordering guarantee.

4. **No Locks or Mutexes:**  The program achieves thread-safe concurrent incrementing without explicit locks (like `Mutex`).  Atomic operations are often faster than acquiring and releasing locks, especially in scenarios where contention is low.

5. **Demonstrates Thread Safety:**  The program creates multiple threads that increment the counter.  If you run it, you'll see that the final counter value is (approximately) `num_threads * 1000`, which demonstrates that the increments are happening safely and concurrently.

**Why it's interesting:**

* **Highlights a key aspect of Rust's safety:**  Interior mutability allows for safe and controlled mutation even in read-only contexts.  This is crucial for many patterns in concurrent programming.
* **Demonstrates a practical use case of atomics:** Atomic types are essential for building concurrent data structures and algorithms without the overhead and potential deadlocks associated with traditional locking mechanisms.
* **Shows a subtle but significant detail about Rust's type system:** The mutability is contained within the `AtomicUsize` type, rather than requiring a `mut` declaration on the variable itself.  This gives Rust its fine-grained control over mutability.