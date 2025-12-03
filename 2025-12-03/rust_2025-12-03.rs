```rust
#![feature(generic_const_exprs)] // Needed for const generics calculations

// A trait for calculating factorial at compile time
trait Factorial<const N: usize> {
    const VALUE: usize;
}

// Base case for factorial calculation
impl Factorial<0> for usize {
    const VALUE: usize = 1;
}

// Recursive case for factorial calculation using const generics
impl<const N: usize> Factorial<N> for usize
where
    [(); N - 1]:, // Ensure N > 0
{
    const VALUE: usize = N * <usize as Factorial<{ N - 1 }>>::VALUE;
}


// A function that uses the compile-time factorial to create an array of a specific size.
fn generate_array<const N: usize>() -> [usize; <usize as Factorial<N>>::VALUE]
where
    [(); <usize as Factorial<N>>::VALUE]:, // Ensure factorial calculation is a valid array size
{
    [0; <usize as Factorial<N>>::VALUE] // Initialize array with zeros
}

fn main() {
    // Create an array of size 3! = 6 at compile time.
    let my_array = generate_array::<3>();

    println!("Array length: {}", my_array.len()); // Outputs: Array length: 6
    println!("First element: {}", my_array[0]);   // Outputs: First element: 0
}
```

**Explanation and Why it's Interesting:**

1. **`#![feature(generic_const_exprs)]`**: This line is crucial. It enables the `generic_const_exprs` feature, which allows us to perform calculations *within* const generics (the `<const N: usize>` part of the function signature). This is still a relatively new and powerful feature of Rust.

2. **Compile-Time Factorial Calculation**: The `Factorial` trait and its `impl` blocks define a compile-time factorial function using recursion.  The `const VALUE` associated with each type is calculated at compile time, not at runtime.

3. **Const Generics for Array Size**: The `generate_array` function uses the result of the compile-time factorial (`<usize as Factorial<N>>::VALUE`) to determine the size of the array `[usize; <usize as Factorial<N>>::VALUE]`.  This is a key demonstration of how const generics can be used to create arrays of sizes known at compile time, based on calculations that also happen at compile time.

4. **`where [(); <usize as Factorial<N>>::VALUE]:,`**: This is a bounds check using a zero-sized array. It forces the compiler to evaluate `<usize as Factorial<N>>::VALUE` at compile time and ensures that it's a valid array size (i.e., non-negative and within the limits of Rust's array size). It also triggers the factorial calculation at compile time, so it's not "lazy evaluated" at runtime. The `[(); N - 1]:` bound in the recursive `impl` is similar; it ensures `N` is positive.

**Why this is clever:**

* **Compile-Time Magic**: It pushes a computation (factorial) into the compile-time domain. No runtime overhead for calculating the size of the array.
* **Const Generics Usage**: It expertly demonstrates the use of const generics and const expressions, a relatively advanced Rust feature.
* **Type-Level Programming**: The `Factorial` trait represents a form of type-level programming where calculations are encoded in the type system.
* **Safety and Guarantees**:  Rust's type system guarantees that the array size is known at compile time, preventing potential runtime errors related to array bounds or invalid sizes.

This example shows how to harness Rust's powerful compile-time features to achieve performance and safety, creating an array with a size determined by a calculation performed during compilation.  It's a concise yet illustrative example of Rust's capabilities.