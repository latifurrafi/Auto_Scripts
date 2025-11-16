```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Use `drain_filter` to efficiently remove even numbers *in-place*,
    // and sum them at the same time.
    let sum_of_evens: i32 = numbers.drain_filter(|x| x % 2 == 0).sum();

    println!("Original vector after removing even numbers: {:?}", numbers);
    println!("Sum of even numbers removed: {}", sum_of_evens);

    // `drain_filter` returns an iterator of the removed elements,
    // which can be consumed in various ways, not just summation.
    // For instance, we could have collected them into another vector.
    // The in-place modification is particularly efficient.
}
```

**Explanation of the "Interesting Feature": `drain_filter`**

This program highlights the `drain_filter` method on the `Vec` type.  It's a more efficient and expressive way to remove elements from a vector based on a condition, while simultaneously collecting the removed elements.

*   **In-place Modification:**  `drain_filter` modifies the original vector *directly*, removing elements that satisfy the given predicate.  This avoids unnecessary allocations of new vectors that would be required if you used `filter` and then reassigned the vector.

*   **Iterator Consumption:** `drain_filter` returns an *iterator* over the removed elements. This is crucial because Rust's iterators are lazy and zero-cost abstractions.  You only pay for the operations you actually perform on the removed elements.  In this example, we immediately sum them, but you could equally collect them into a new `Vec`, print them, or perform any other iterator-based operation.

*   **Efficiency:** `drain_filter` is generally more efficient than a naive approach of iterating, filtering, and then creating a new vector. It avoids unnecessary copying of elements.

*   **Ownership and Borrowing:**  The closure passed to `drain_filter` borrows each element immutably (`|x|`).  If you needed to modify the elements as you remove them, you'd likely need to use a different approach.

**Why this is unique and clever:**

*   **Focus on `drain_filter`:**  It isolates and showcases a relatively less commonly used, but powerful, method on `Vec`.
*   **Conciseness:** The code is short and to the point, making it easy to understand.
*   **Efficiency:**  It utilizes a method optimized for in-place modification and iterator-based processing.
*   **Clarity:**  The comments explain the purpose of `drain_filter` and its advantages.
*   **Illustrative Consumption:**  It shows how to consume the iterator returned by `drain_filter` immediately.

This combination of in-place modification, iterator-based processing, and concise syntax makes `drain_filter` a particularly interesting and powerful feature of Rust's standard library.  The program demonstrates how Rust allows you to perform complex operations on data structures efficiently and elegantly.