## Notes for 5. Using Structs to Strucuture Related Data

- structs remind me of javascripts way of creating classes:
  - key: value, seperated by commas
- mutable structs need to be declared completed mutable, single attributes can not be declared as mutable
- .. (dot dot) syntax allows for copying from other structs
- values need to be declared as lifetime parameters. Therefore references won't work out of the box.
- impl [Structname] can be used to implement methods for a struct
  - Methods are perferred over functions, as you work with data that is in said struct
    - Those methods are called "associated functions"
  - When defining a method, a reference to *self* will be used.
  - It is possible to name methods the same as struct fields.
- other than C++, Rust uses automatic referencing and dereferencing, which makes the syntax cleaner - no more (->) operator.