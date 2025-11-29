```rust
fn main() {
    // This program showcases const generics and compile-time array initialization.

    const COUNT: usize = 5;

    // `ArrayGenerator` is a function that takes a const generic parameter N.
    // It uses it to create an array of size N at compile time.
    const fn array_generator<const N: usize>() -> [i32; N] {
        //  This loop executes at compile time to initialize the array.
        let mut arr = [0i32; N];
        let mut i = 0;
        while i < N {
            arr[i] = (i as i32) * (i as i32); // Each element is the square of its index.
            i += 1;
        }
        arr
    }

    // Initialize a static array at compile time using our function.
    static SQUARES: [i32; COUNT] = array_generator::<COUNT>();

    println!("Squares up to {}: {:?}", COUNT -1, SQUARES);

    // Demonstrating that the array is truly initialized at compile time.
    // Accessing it is just like accessing a normal static array.
    for &x in SQUARES.iter() {
        println!("{}", x);
    }
}
```

**Explanation:**

1. **Const Generics:**  `array_generator::<const N: usize>()` uses a `const generic` parameter `N`.  This means `N` is a constant value determined at compile time, not a runtime value.  This allows us to specify array sizes at compile time, based on constants.

2. **`const fn`:** The `array_generator` is declared as `const fn`.  This tells the compiler that this function *can* be evaluated at compile time.  This is crucial because we want to initialize the array *before* the program even starts running.

3. **Compile-Time Initialization:** The `while` loop inside `array_generator` executes at compile time.  Because we are using a `const fn` and `COUNT` is a constant, the compiler can determine the values to populate the array during compilation.

4. **`static` Array:** `static SQUARES: [i32; COUNT] = array_generator::<COUNT>();` declares a static array named `SQUARES`. `static` variables exist for the lifetime of the program, and in this case, its value is calculated by the `array_generator` function *at compile time*.

**Why this is interesting:**

* **Performance:**  The array `SQUARES` is completely initialized *before* the program starts.  There's no runtime cost of initialization.  This can be a significant performance boost, especially for large arrays.
* **Safety:** Compile-time evaluation helps catch errors earlier in the development process.  If `array_generator` had an error, it would be detected during compilation, not at runtime.
* **Expressiveness:**  Const generics enable more flexible and powerful APIs, allowing us to write functions that operate on arrays of different sizes without resorting to dynamic allocation or runtime size checks.
* **Compile-Time Computation:** Rust allows for more complex computations at compile time than many other languages, which can lead to more optimized code and reduced runtime overhead.

This example, while simple, demonstrates a powerful and potentially useful feature of Rust that can lead to more efficient and reliable code.  The combination of `const generics`, `const fn`, and `static` allows us to perform significant computations at compile time, resulting in zero-cost abstractions and improved performance.