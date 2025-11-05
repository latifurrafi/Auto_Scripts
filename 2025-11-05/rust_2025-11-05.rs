```rust
fn main() {
    // A "type-level" linked list!  We use enums to define types that represent
    // the structure of the list, not the values stored in it.

    enum Nil {} // Empty list

    enum Cons<H, T> {
        Head(std::marker::PhantomData<H>, T), // "H" is the type of the head, T is the rest of the list
    }

    // Trait to calculate the length of the type-level list at compile time!
    trait Length {
        const LEN: usize;
    }

    impl Length for Nil {
        const LEN: usize = 0;
    }

    impl<H, T: Length> Length for Cons<H, T> {
        const LEN: usize = T::LEN + 1;
    }


    // Define some type-level lists:
    type List1 = Cons<i32, Nil>;
    type List2 = Cons<String, List1>;
    type List3 = Cons<bool, List2>;


    println!("Length of List1 (i32): {}", <List1>::LEN);
    println!("Length of List2 (String, i32): {}", <List2>::LEN);
    println!("Length of List3 (bool, String, i32): {}", <List3>::LEN);

    // This demonstrates type-level computation: the length is calculated
    // at compile time, not runtime. The actual values of the lists aren't
    // even stored! This is a highly unusual but powerful feature for
    // advanced type-level programming.  It uses zero-sized types (`PhantomData`) to avoid
    // ownership issues since we're just interested in the *structure* of the types.
}
```

Key features and explanation:

* **Type-Level Programming:**  This program performs calculations *at compile time* based on the *types* themselves, rather than based on runtime values.  This is a powerful (though niche) feature of Rust that allows for compile-time optimization and more robust type safety.
* **Type-Level Linked List:** We're creating a linked list structure *purely in the type system*.  The `Nil` and `Cons` enums represent the structure of the list (empty or head + tail), but they don't actually *store* any values.  Instead, the type parameters (`H` and `T`) determine the types that *would* be stored if we were building a regular list.
* **`Length` Trait and `const` Generics:** The `Length` trait defines a `LEN` constant associated with each list type.  The `impl` blocks calculate the length recursively. `const LEN: usize` allows to use it as a compile time constant.
* **`PhantomData`:**  The `std::marker::PhantomData` is crucial.  We're using type parameters (`H` in `Cons`) that we don't actually *own* in the data structure. `PhantomData` tells the compiler that `Cons` "logically" contains a value of type `H`, even though it doesn't actually hold it. This ensures that the compiler still checks for correct lifetime and ownership if the type `H` required it. Without `PhantomData`, the compiler would complain that we're not using the type parameter `H`.
* **Zero Runtime Cost:**  Since everything is done at compile time, there's effectively no runtime cost for calculating the lengths of these type-level lists.
* **Cleverness:** The program demonstrates a fairly advanced Rust concept (type-level programming) in a concise and understandable way.  It's unique in that it shows how to use enums, traits, associated constants, and `PhantomData` to build a data structure that exists purely at the type level.

This example is complex and requires some familiarity with Rust's type system to fully understand.  However, it clearly showcases Rust's ability to perform computations based on types, a feature that is less common in other languages.  This can be very useful in certain situations, such as writing highly optimized libraries or enforcing complex constraints at compile time.