```rust
#![feature(generic_const_exprs)] // Required for const generics based on calculations

// This program demonstrates compile-time calculation with const generics,
// effectively generating a Fibonacci sequence at compile time within an array.

const fn fibonacci<const N: usize>() -> [usize; N]
where
    [usize; N]:, // Necessary bound
{
    let mut result = [0; N];
    if N > 0 {
        result[0] = 0;
    }
    if N > 1 {
        result[1] = 1;
    }

    let mut i = 2;
    while i < N {
        result[i] = result[i - 1] + result[i - 2];
        i += 1;
    }
    result
}

const FIB_SEQUENCE: [usize; 10] = fibonacci::<10>(); // Calculate Fibonacci sequence up to 10 terms at compile time.

fn main() {
    println!("Fibonacci sequence (first 10 terms): {:?}", FIB_SEQUENCE);

    // Prove it's truly compile-time by accessing elements as constants:
    const THIRD_FIB: usize = FIB_SEQUENCE[2];
    println!("The 3rd Fibonacci number (0-indexed): {}", THIRD_FIB);

    // Attempting to modify would cause a compile error, as FIB_SEQUENCE is const.
    // FIB_SEQUENCE[0] = 5; // Error: cannot assign to `FIB_SEQUENCE[_]` which is behind a `&` reference

    // Show that we can still manipulate the array at runtime if copied.
    let mut runtime_fib = FIB_SEQUENCE;
    runtime_fib[0] = 42;
    println!("Modified copy of the sequence: {:?}", runtime_fib);
}
```

Key features and explanation:

* **`#![feature(generic_const_exprs)]`:**  This enables the use of `const` generics in more complex scenarios, particularly where the size of a type depends on a calculation. This is still a nightly feature (as of writing). Without this, const generics are much more limited.

* **Compile-Time Calculation:** The `fibonacci<const N: usize>()` function is a `const fn`, meaning it can be evaluated at compile time if all its inputs are known at compile time. The generic parameter `N` allows us to specify the desired length of the Fibonacci sequence as a `const`.

* **`where [usize; N]:,`**: This is a necessary bound for using generic const expressions involving arrays. It tells the compiler that the array type `[usize; N]` must be well-formed.

* **`FIB_SEQUENCE: [usize; 10] = fibonacci::<10>();`:** This line is where the magic happens.  We are declaring `FIB_SEQUENCE` as a `const` array of size 10, and initializing it with the result of `fibonacci::<10>()`. Because `N` is known to the compiler (it's 10), the `fibonacci` function is executed during compilation, generating the Fibonacci sequence.

* **Proof of Compile Time:** The program then accesses elements of `FIB_SEQUENCE` via `FIB_SEQUENCE[2]` to initialize another `const`. This demonstrates that the Fibonacci sequence is fully calculated and stored at compile time. The value of `THIRD_FIB` is embedded directly into the compiled binary.

* **Immutability and Copying:**  The program demonstrates that `FIB_SEQUENCE` is truly immutable (you can't change it directly), because it's declared as `const`.  However, you can create a *copy* of it (as done with `let mut runtime_fib = FIB_SEQUENCE;`) and modify the copy at runtime.

* **Uniqueness:**  The combination of `const fn`, generic const expressions, compile-time array generation, and demonstration of its compile-time nature makes this program stand out. While `const fn` is common, using it with arrays whose sizes are controlled by compile-time calculations adds an extra layer of sophistication.
This program illustrates the power and flexibility of Rust's compile-time capabilities, enabling performant and predictable code. The Fibonacci sequence itself is just an example; the same principle can be applied to more complex calculations and data structures.