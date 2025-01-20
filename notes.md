# Rust

## Variables

- `mut` is used to make a variable mutable
- `const` declare a constant, must be annotated with a type
- shadowing allows you to redeclare a variable with the same name, useful for changing the type of a variable
  ```rust
  let spaces = "   ";
  let spaces = spaces.len();
  ```
