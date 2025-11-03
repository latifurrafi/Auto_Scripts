```rust
fn main() {
    // Illustrating the power of const generics with array creation
    const ARRAY_SIZE: usize = 5;

    // A function that creates an array of a specified size, filled with the same value.
    fn create_filled_array<const N: usize, T: Copy>(value: T) -> [T; N] {
        //  Demonstrates array initialization using `[value; N]` with a const generic N.
        [value; N] 
    }

    let numbers: [i32; ARRAY_SIZE] = create_filled_array(42);
    let booleans: [bool; 3] = create_filled_array(true);

    println!("Numbers: {:?}", numbers); // Output: Numbers: [42, 42, 42, 42, 42]
    println!("Booleans: {:?}", booleans); // Output: Booleans: [true, true, true]

    // Using a calculated const generic
    const DOUBLE_ARRAY_SIZE: usize = ARRAY_SIZE * 2;
    let doubles: [f64; DOUBLE_ARRAY_SIZE] = create_filled_array(3.14);
    println!("Doubles: {:?}", doubles); // Output: Doubles: [3.14, 3.14, 3.14, 3.14, 3.14, 3.14, 3.14, 3.14, 3.14, 3.14]

    // This will cause a compile-time error, showing type safety:
    // let strings: [String; 2] = create_filled_array("hello".to_string()); // Compile Error!  String is not `Copy`
}
```

**Explanation and Why it's Unique/Clever:**

* **Const Generics:**  The core feature showcased is *const generics*. Rust allows you to parameterize functions and types not just by types (like `Vec<T>`), but also by constant values (like integers or booleans). Here, `create_filled_array` is generic over the size of the array (`N`), which is known at compile time.

* **Array Creation with `[value; N]`:** The function `create_filled_array` uses the concise array initialization syntax `[value; N]`, which creates an array of size `N` filled with copies of `value`.  This syntax only works if `N` is a compile-time constant.

* **Compile-Time Safety:** The commented-out line highlights the important point that the `Copy` trait is *required* for this kind of array initialization.  If you try to use a type that isn't `Copy` (like `String`), the compiler will give you an error. This demonstrates Rust's strong emphasis on memory safety and preventing potentially problematic implicit cloning of resources.

* **Calculated Const Generics:** The `DOUBLE_ARRAY_SIZE` constant demonstrates that const generics can be calculated based on other constants, offering further flexibility.

* **Conciseness and Clarity:**  The code is short and directly illustrates the concept without unnecessary complexity.

**Why it's better than simpler examples:**

* **More Useful Than `fn foo<const N: usize>(arr: [i32; N])`:** A very basic const generic example would be to accept an array of size `N` as input. While technically showing the feature, it doesn't demonstrate a *practical* use case.  This program *creates* an array dynamically based on a compile-time size, which is a more valuable application of const generics.
* **Demonstrates `Copy` Requirement:**  By attempting to use `String` and receiving a compile error, it illustrates a critical constraint of the `[value; N]` initialization, which is often overlooked in introductory examples.
* **Shows Calculation:**  Demonstrates that you aren't limited to hard-coded constants.
* **Less Verbose Than Using Macros:**  While macros *could* achieve similar results, using const generics directly is cleaner and more idiomatic Rust.

This program elegantly captures the benefits of const generics: allowing compile-time size parameters for arrays, enabling more powerful and efficient code generation, and enforcing type safety.  It's a small but potent demonstration of a key feature in modern Rust.