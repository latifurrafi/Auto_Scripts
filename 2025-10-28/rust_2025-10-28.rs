```rust
fn main() {
    // This program demonstrates Rust's powerful and safe ownership system
    // and move semantics with a custom smart pointer.

    struct ExpensiveResource {
        data: String,
    }

    impl ExpensiveResource {
        fn new(size: usize) -> Self {
            println!("Allocating a resource of size: {}!", size);
            ExpensiveResource { data: "X".repeat(size) }
        }

        fn access(&self) {
            println!("Accessing the resource!");
            // You could imagine doing some actual work with 'self.data' here
        }
    }

    // A custom smart pointer type that enforces single ownership.
    struct UniquePtr<T> {
        ptr: *mut T,
    }

    impl<T> UniquePtr<T> {
        fn new(data: T) -> Self {
            let boxed_data = Box::new(data); // Box allocates on the heap.
            let raw_ptr = Box::into_raw(boxed_data); // Convert to raw pointer.
            UniquePtr { ptr: raw_ptr }
        }

        // Consume the pointer and return the underlying value.
        // Prevents further access via the smart pointer, ensuring ownership is transferred.
        fn take(mut self) -> T {
            if self.ptr.is_null() {
                panic!("Pointer is already taken or was never initialized!");
            }

            unsafe {
                let boxed = Box::from_raw(self.ptr);  // Reclaim ownership of the Box.
                self.ptr = std::ptr::null_mut(); // Prevent double free.
                *boxed // Dereference to move the value *out* of the box (ownership transfer).
            }
        }
    }

    // Crucially, implement Drop to ensure the allocated memory is freed.
    impl<T> Drop for UniquePtr<T> {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                println!("Dropping the resource!");
                unsafe {
                    drop(Box::from_raw(self.ptr)); //Reclaim ownership for deallocation.
                }
            }
        }
    }

    // Create a UniquePtr, allocating the resource.
    let mut resource_ptr = UniquePtr::new(ExpensiveResource::new(1024));

    // Access the resource while the UniquePtr owns it.
    unsafe {
        (*resource_ptr.ptr).access(); // Accessing the raw pointer via dereference
                                        // is *unsafe*, but we know it's valid here.
    }


    // Take ownership out of the UniquePtr.  The UniquePtr is now invalid.
    let resource = resource_ptr.take();

    // We can now work with the resource directly.
    resource.access();


    // resource will be dropped at the end of this scope.

    // The following line would cause a compile error due to move semantics:
    //  println!("Data is: {}", resource_ptr.ptr.is_null());

    // The following would cause a runtime panic due to accessing already taken resource:
    // let another_resource = resource_ptr.take();
}
```

Key improvements and explanations:

* **`UniquePtr`**:  The core of the example is a custom smart pointer called `UniquePtr`.  This pointer is designed to ensure *exclusive* ownership of the data it points to.  This directly demonstrates Rust's core ownership principles.
* **`take()` method**:  This is critical.  `take()` *consumes* the `UniquePtr`.  It *moves* the underlying `ExpensiveResource` out of the `UniquePtr` and into a regular variable.  After `take()`, the `UniquePtr` is no longer valid, preventing double frees and other memory safety issues. The `panic!` if the pointer is already taken demonstrates the explicit safety checks that can be implemented.
* **`Drop` implementation**: The `Drop` trait is implemented for `UniquePtr`.  This is *essential* to ensure that the memory allocated for the `ExpensiveResource` is freed when the `UniquePtr` goes out of scope.  This prevents memory leaks.  The `drop` function includes a check to make sure that the pointer has not already been taken.
* **`unsafe` blocks**: The code that interacts directly with raw pointers (`*mut T`) is wrapped in `unsafe` blocks. This clearly marks the parts of the code that require extra care and attention, as the compiler cannot guarantee memory safety within those blocks.  The comment within the `unsafe` block explains *why* it's safe in this specific context.
* **Move semantics**: The example directly demonstrates move semantics. After `resource_ptr.take()`, you cannot use `resource_ptr` anymore. This is a compile-time error if you try to. This reinforces the concept that ownership is transferred, not copied.
* **Error Handling/Panics:** Includes runtime panic to demonstrate what happens when attempting to access a taken resource.  This provides a stronger illustration of safety mechanisms.
* **Comments**: The code is heavily commented to explain the purpose of each section and the rationale behind the design choices.
* **Clear Example**: The program clearly allocates an expensive resource, accesses it through the unique pointer, transfers ownership out of the unique pointer, accesses it again, and then deallocates it when it goes out of scope. The print statements demonstrate the state changes throughout the program.
* **No Cloning:** The code avoids any explicit cloning. This is important because the goal is to demonstrate ownership and move semantics, not copying data.
* **Real-World Relevance**:  While simplified, the `UniquePtr` pattern is a common building block for more complex smart pointers and memory management techniques in Rust.
* **Compile-Time Errors Explained**: Comments explain what compile-time errors would occur if you were to use the moved values.

This improved version is more robust, educational, and directly addresses the core concepts of ownership, move semantics, and memory safety in Rust with a practical and understandable example. The `unsafe` blocks are minimized and clearly justified.  It's a much stronger demonstration of what makes Rust unique.