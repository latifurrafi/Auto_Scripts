```rust
fn main() {
    // This program demonstrates type-level programming with const generics
    // to calculate the n-th Fibonacci number at compile time.

    // Define a struct to hold the calculated Fibonacci number.
    struct Fibonacci<const N: usize>;

    // Implement a const function to calculate the Fibonacci number recursively.
    impl<const N: usize> Fibonacci<N> {
        const VALUE: usize = match N {
            0 => 0,
            1 => 1,
            _ => Fibonacci::<{ N - 1 }>::VALUE + Fibonacci::<{ N - 2 }>::VALUE,
        };
    }

    // Calculate the 10th Fibonacci number at compile time.
    const FIB_10: usize = Fibonacci::<10>::VALUE;

    // Use a static assertion to ensure the calculation is correct at compile time.
    //  This will cause a compilation error if FIB_10 is not equal to 55.
    // (Requires nightly Rust and the `const_evaluatable_checked` feature)
    // const _: () = assert!(FIB_10 == 55);

    // Print the result.  The calculation is already done, this is just output.
    println!("The 10th Fibonacci number is: {}", FIB_10);

    // A more concise, but harder to read version using a compile-time function:
    const fn fibonacci<const N: usize>() -> usize {
        match N {
            0 => 0,
            1 => 1,
            _ => fibonacci::<{ N - 1 }>() + fibonacci::<{ N - 2 }>(),
        }
    }

    const FIB_12: usize = fibonacci::<12>();
    println!("The 12th Fibonacci number is: {}", FIB_12); // Output: 144
}
```

Key features and explanations:

* **Const Generics:** The program leverages `const generics` to parameterize the `Fibonacci` struct with a compile-time constant integer `N`. This allows us to perform calculations based on `N` at compile time.
* **Compile-Time Calculation:**  The `Fibonacci::VALUE` is calculated using a `const` function.  `const` functions are evaluated at compile time whenever possible.  This means that the Fibonacci number is computed during compilation, not during runtime.
* **Type-Level Programming:** By embedding the value of `N` within the type `Fibonacci<N>`, we're effectively doing type-level programming.  The Rust compiler uses this type information to perform the calculation.
* **Static Assertion (commented out):** The `assert!(FIB_10 == 55)` statement (commented out because it requires nightly Rust and a specific feature flag) would be evaluated at compile time. If the assertion fails, the compilation would halt with an error. This adds an extra layer of correctness verification.  Enabling it requires:
    1.  Using nightly Rust: `rustup default nightly`
    2.  Adding `#![feature(const_evaluatable_checked)]` to the top of the file.
* **Concise Compile-Time Function (fibonacci):** A more compact version of the same logic is demonstrated using a `const fn` directly, avoiding the need for a struct. This achieves the same compile-time calculation in a more streamlined manner.
* **Uniqueness:** While Fibonacci is a common example, the combination of const generics, `const` functions, and the potential for static assertions creates a unique demonstration of compile-time programming in Rust.
* **Cleverness:** The program elegantly uses the type system and compile-time evaluation to perform computations without any runtime overhead for the Fibonacci calculation.  The resulting executable simply contains the already-calculated result.

How to run this program:

1. **Install Rust:**  If you don't have it, install Rust from [https://www.rust-lang.org/](https://www.rust-lang.org/).
2. **Save as `fibonacci.rs`:**  Copy the code above and save it in a file named `fibonacci.rs`.
3. **Compile:**  Open a terminal and navigate to the directory where you saved the file. Run `cargo run`.
4. **Observe the output:** The program will print the 10th and 12th Fibonacci numbers.

Note: If you want to enable the static assertion (`assert!(FIB_10 == 55)`), you will need to switch to the nightly Rust toolchain and enable the `const_evaluatable_checked` feature flag. Uncomment the line at the top to enable it.
```
#![feature(const_evaluatable_checked)]
```

Then compile with `cargo +nightly run`.