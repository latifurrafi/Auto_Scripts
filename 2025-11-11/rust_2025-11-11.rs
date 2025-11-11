```rust
fn main() {
    // Leverage compile-time evaluation and const generics for a prime number sieve.
    const SIEVE_SIZE: usize = 50;

    const PRIMES: [bool; SIEVE_SIZE] = {
        let mut primes = [true; SIEVE_SIZE];
        primes[0] = false; // 0 is not prime
        primes[1] = false; // 1 is not prime

        let mut i = 2;
        while i * i < SIEVE_SIZE {
            if primes[i] {
                let mut j = i * i;
                while j < SIEVE_SIZE {
                    primes[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
        primes
    };

    println!("Primes up to {}:", SIEVE_SIZE);
    for (index, is_prime) in PRIMES.iter().enumerate() {
        if *is_prime {
            println!("{}", index);
        }
    }
}
```

**Explanation:**

1. **Compile-Time Prime Sieve:**  The core of the program is a prime number sieve algorithm implemented within a `const` context.  This means the entire prime number calculation happens *at compile time*.

2. **`const SIEVE_SIZE: usize = 50;`:**  Defines the size of the sieve. This is a `const` value, allowing it to be used in array sizes and other compile-time expressions.

3. **`const PRIMES: [bool; SIEVE_SIZE] = { ... };`:**  This is the magic.  It declares a constant array `PRIMES` of booleans. The crucial part is the `{ ... }` block following the equals sign. This is a **`const` block**. Code within this block is evaluated during compilation.

4. **Sieve Algorithm:**  The code inside the `const` block implements the Sieve of Eratosthenes algorithm to find prime numbers up to `SIEVE_SIZE`.  It efficiently marks non-prime numbers by iterating through multiples of each prime.

5. **Compile-Time Computation:** Because the `PRIMES` array is calculated at compile time, the final executable doesn't need to perform the sieve calculation at runtime. The `PRIMES` array is directly embedded in the executable's data section, pre-computed.

6. **Printing the Results:**  The `main` function simply iterates through the `PRIMES` array (which already contains the results) and prints the indices where the value is `true` (indicating a prime number).

**Why this is interesting:**

* **Performance:**  Eliminating runtime computation can lead to significant performance improvements, especially for frequently used values.  This program trades compile time (which only happens once) for faster execution every time it runs.
* **Const Generics (Technically Implied):**  Although not explicitly using a `generic<const N: usize>`, the `SIEVE_SIZE` being a `const` makes the array type `[bool; SIEVE_SIZE]` depend on a compile-time constant. This effectively demonstrates how compile-time constants are foundational to const generics.  If you wanted a more explicit const generic version, you could rewrite it to take the sieve size as a type-level parameter.
* **Readability and Expressiveness:**  The code is concise and demonstrates how Rust enables writing performant and compile-time logic in a relatively readable way.  The `const` block clearly demarcates the code that will be executed at compile time.
* **Safety:** Because the prime array is determined at compile time, it is guaranteed to be immutable and safe to access.

This program showcases a powerful combination of Rust's features: compile-time evaluation, constant values, and safe memory management.  It demonstrates how to move computation from runtime to compile time for improved performance and correctness.