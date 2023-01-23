## Notes for 2. Programming a Guessing Game

- `mut` enables coders to change the value of a variable
- `const` variables can't have the `mut` code at all as they are constants. They can't used for results that are processed in runtime.
- Shadowing creates a new variable with the same name as the old variable.
  - The variable can have a new type, for example - old: String ; new: u32
  - Old variable value will become inaccessible when shadowing a new variable
  - Has the ability to stop an objects mutability by shadowing a variable without the `mut` keyword
- When using certain functions, Rust needs the data type of one variable (e.g `42.parse()`)
- numerical operations are the same compared to other languages
- tuples can become very handy in the future I suppose (returning multiple values with different types)
- out of bound errors can still happen, but rust will panic and crash the program if it happens, compared to C
- statements always end with a semicolon (;), while expressions do not
- statements never return something while expressions do
- expressions are used as implicit return values for functions in rust
  - ternary operator in rust is basically a one-line if condition because of this
- you can label loops with `'name: loop {...` code and access them with `'name` later on
- At the end of the day, loops are very much the same compared to other languages