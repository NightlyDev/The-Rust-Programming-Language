## Notes for 2. Programming a Guessing Game

- `mut` enables coders to change the value of a variable
- `const` variables can't have the `mut` code at all as they are constants. They can't used for results that are processed in runtime.
- Shadowing creates a new variable with the same name as the old variable.
  - The variable can have a new type, for example - old: String ; new: u32
  - Old variable value will become inaccessible when shadowing a new variable
  - Has the ability to stop an objects mutability by shadowing a variable without the `mut` keyword
- When using certain functions, Rust needs the data type of one variable (e.g `42.parse()`)
- 