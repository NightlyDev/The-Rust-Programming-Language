## Notes for 2. Programming a Guessing Game

- `mut` (mutable) before variable enables to change the variable later on
- `&` reference of variable - similar to cpp
- `match` expression is used for error handling and comparisons
  - `Ok` -> return when method without an error
  - `Err` -> return when method has an error
  - `Ordering::...` for easy comparisons
- `expect` for simple error handling which will crash tthe program with a message (for now?)
- `rustc --explain E0308` explains to you what the *type did not match* error is with examples. Very cool!
- Shadowing of variables is possible, which prevents silly variable names!
- Cargo.toml dependencies can be specified like `^0.8.5` - similar to Pythons PIP requirements.txt file