## Notes for 3. Understanding Ownership

Ownership Rules
- Each value in Rustt has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

Heap allocated objects:
- When assigning a variable to a previously defined variable with an heap allocated object, the new assigned variable will
  copy the pointer, length and capacity of the defined variable. But it will not allocate new memory on the heap.
  - pointer = pointer to the memory that holds the contents
  - length = currently allocated memory in bytes
  - capacity = total amount of memory in bytes that a variable has received from the allocator
- copies from stack allocated variables can be copies just fine, heap allocated need to be "cloned".

Ownership:
- Ownership will be passed to a function when it you are not using a reference. Therefore a variable can changes scopes into function and then be dropped after the function has concluded.
- if returned a value from inside the function, ownership will be given back to the scope that used the function.

Borrowing/References:
- You can use "borrowing" to change values from in-function passed variables with `&`
  Similar to other languages, `&` is creating a reference, which is like a pointer that points toward an object.
- to really change a value, you have to make a reference mutable `&mut string`. The example here must be declared as mutable before hand
- You can only have one mutable reference for an object at a time.
- You can have multiple immutable references.
- You can't mix immutable and mutable references in a value, but you can use a mutable reference after the last usage of a immutable one.
- dangling references are not possible in Rust, as references are borrowed values.

Slices:
- Is a reference tto a contiguous sequence of elements in a collection rather than a whole collection.
- Because it is a reference, it does not have ownership
- Can be used to "sync up" a collection and a value that has been calculated from it (via a function).
  - if the collection somehow changes before the slice will be used the last time, the slice will error because of the princple of  
    borrowing.
- (reminds me somewhat of pythons syntax)