```rust
// This program demonstrates the power of move semantics and lifetimes
// combined with a clever use of closures.

fn main() {
    let mut data = String::from("Initial Value");

    // `updater` is a closure that takes a mutable reference to a String,
    // appends to it, and returns a new String that *borrows* from the original.
    // This borrowing is crucial to demonstrate lifetimes.
    let updater = |input: &mut String| -> &str {
        input.push_str(" - Updated!");
        input.as_str() // Returns a slice (&str) that borrows from `input`
    };

    // The `process_data` function takes ownership of the String.
    fn process_data(mut s: String, update_func: impl Fn(&mut String) -> &str) {
        // `updated_data` borrows from `s` and its lifetime is tied to `s`.
        let updated_data = update_func(&mut s);

        println!("Processed data: {}", s);
        println!("Borrowed data: {}", updated_data);
    }

    // `data` is moved into `process_data`.  It's no longer valid after this line.
    process_data(data, updater);

    // This line would cause a compile-time error because `data` has been moved.
    // println!("Data after move: {}", data); // Error: use of moved value 'data'
}
```

Key features demonstrated:

* **Move Semantics:** The `data` String is moved into the `process_data` function. After the call, `data` is no longer valid in `main`.  This prevents double frees and data races.  Trying to use `data` after the move would result in a compile-time error.
* **Borrowing and Lifetimes:** The closure `updater` takes a mutable reference to the `String` and returns a borrowed slice (`&str`). This illustrates how Rust's borrow checker ensures that the borrowed slice does not outlive the String it refers to. The lifetime of `updated_data` within `process_data` is tied to the lifetime of the `s` String.
* **Closures:**  The `updater` is a closure that captures the current environment. The closure's return type being a borrow is important.  If it returned an owned `String`, there wouldn't be a lifetime dependency on the input.
* **Compile-Time Safety:** The code purposefully includes a commented-out line that *would* cause a compile-time error. This highlights how Rust's compiler proactively prevents common memory management errors.

The program elegantly showcases how Rust's ownership, borrowing, and lifetimes system work together to ensure memory safety and prevent common programming errors at compile time.  The use of a closure to manipulate a String and return a borrowed slice is a clean and illustrative example.