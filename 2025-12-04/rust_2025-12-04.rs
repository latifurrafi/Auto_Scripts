```rust
fn main() {
    // Let's use const generics to create arrays of different sizes at compile time.

    const fn fibonacci<const N: usize>() -> [usize; N] {
        let mut result = [0; N];
        if N > 0 {
            result[0] = 1;
        }
        if N > 1 {
            result[1] = 1;
        }
        for i in 2..N {
            result[i] = result[i - 1] + result[i - 2];
        }
        result
    }

    // Define a macro to neatly print the Fibonacci sequence.
    macro_rules! print_fib {
        ($size:literal) => {
            {
                let fib_seq = fibonacci::<$size>();
                println!("Fibonacci sequence of length {}: {:?}", $size, fib_seq);
            }
        };
    }

    //  Call the macro to print different lengths of the sequence.
    print_fib!(5);
    print_fib!(10);
    print_fib!(15);
}
```

**Explanation:**

1. **`const fn fibonacci<const N: usize>()`:**  This defines a `const fn` (constant function) that calculates the Fibonacci sequence. The key here is `const N: usize`, which is a *const generic*.  This means `N` is a generic type parameter, but its *value* is known at compile time. This allows us to create arrays of different sizes based on the compile-time constant `N`.  `const fn` means the function can be evaluated at compile time if its arguments are known at compile time, allowing for zero-cost abstractions.

2. **`[usize; N]`:** Inside the function, we declare an array `result` of type `[usize; N]`.  Because `N` is a const generic, the size of the array is determined at compile time.

3. **Fibonacci Calculation:** The function then calculates the Fibonacci sequence and stores it in the `result` array.

4. **`macro_rules! print_fib`:** This is a macro that takes a literal integer as input (representing the size of the Fibonacci sequence we want to print). It uses the `fibonacci` function (with the given size) to generate the sequence and then prints it in a user-friendly format.

5. **Macro Calls:**  The `print_fib!` macro is called with different sizes (5, 10, and 15). This demonstrates that the `fibonacci` function is being instantiated with different compile-time constant values for `N`, resulting in arrays of different sizes being created and populated.

**Why this is interesting:**

* **Compile-Time Computation:**  The Fibonacci sequence is calculated at compile time thanks to `const fn`. This leads to faster runtime execution, as the results are precomputed.
* **Const Generics for Static Sizes:**  Const generics are a powerful feature that allows you to parameterize types and functions by constant values.  This is especially useful for creating statically-sized data structures like arrays.
* **Zero-Cost Abstraction:**  The combination of `const fn` and const generics allows for zero-cost abstractions. We can write generic code that performs calculations at compile time, without incurring any runtime overhead.
* **Conciseness:** The macro helps make the code more readable and easier to use, demonstrating a neat abstraction layer on top of the core functionality.

This example highlights a modern and powerful feature of Rust, demonstrating how it allows for efficient, type-safe, and expressive code.  The use of compile-time computation and const generics is a prime example of Rust's commitment to performance and zero-cost abstractions.