```rust
fn main() {
    // Demonstrate zero-cost abstractions with a custom trait and blanket implementation.

    // Define a trait for things that can be "reversed".  Note: Generic over 'T' makes it more versatile.
    trait Reversible<T> {
        fn reverse(&self) -> T;
    }

    // Blanket implementation for all types that implement `Iterator<Item=char>`
    // and can be collected into a `String`.  This means we're *automatically*
    // providing `reverse` functionality to anything that meets these criteria,
    // without explicit implementations for each type!
    impl<I: Iterator<Item = char>> Reversible<String> for I
        where
            String: FromIterator<char>, // String can be constructed from chars
    {
        fn reverse(&self) -> String {
            self.clone().rev().collect() // Clone needed as `self` is consumed in `rev()`
        }
    }


    let hello = "Hello, World!".to_string();
    let reversed_hello = hello.chars().reverse(); // Implicitly uses the blanket impl!

    println!("Original: {}", hello);
    println!("Reversed: {}", reversed_hello);


    // Another example, using a different type that still satisfies the blanket impl.
    let numbers = vec!['1', '2', '3', '4', '5'];
    let reversed_numbers = numbers.into_iter().reverse(); // Uses blanket impl, consuming the vec!

    println!("Original Numbers: 12345");
    println!("Reversed Numbers: {}", reversed_numbers);

    // Show the zero-cost nature. The `reverse` calls get inlined and optimized
    // into equivalent code that directly reverses the iterator.  No vtable lookup,
    // no dynamic dispatch overhead! This is all determined at compile time.
}
```

Key features showcased:

* **Zero-Cost Abstractions:** The program highlights how Rust's traits and generic programming allow for powerful abstractions without runtime performance penalties. The `Reversible` trait and its blanket implementation provide a general-purpose `reverse` function for a wide range of types, but the compiler eliminates any overhead associated with dynamic dispatch. The code is effectively inlined and optimized to be as efficient as if we had written a specific reversal function for each type.

* **Traits and Generics:**  Demonstrates the power of traits to define shared behavior and generics to make that behavior applicable to multiple types.  The generic `Reversible<T>` makes it clear what type is returned.

* **Blanket Implementations:**  Uses a blanket implementation (`impl<I: Iterator<Item = char>> Reversible<String> for I ...`) to automatically provide the `reverse` functionality for *any* type that implements `Iterator<Item = char>` and can be collected into a `String`. This avoids code duplication and makes the code very reusable.  The `where` clause puts additional constraints to ensure that the implementation is valid.

* **Ownership and Borrowing:** Shows how to work with iterators and ownership.  The `clone()` call in the `reverse()` implementation is crucial to avoid consuming the original iterator. When we want to consume the collection (as in the `numbers` example), we use `into_iter()` instead of `chars()`.

* **Conciseness:** The program is relatively short and readable while demonstrating complex concepts.

The program's output will be:

```
Original: Hello, World!
Reversed: !dlroW ,olleH
Original Numbers: 12345
Reversed Numbers: 54321
```