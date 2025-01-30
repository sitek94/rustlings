# Rust

## Variables

- `mut` is used to make a variable mutable
- `const` declare a constant, must be annotated with a type
- shadowing allows you to redeclare a variable with the same name, useful for changing the type of a variable
  ```rust
  let spaces = "   ";
  let spaces = spaces.len();
  ```

## Primitive Types

- Booleans are one byte in size
  - Each `bool` uses 1 byte (8 bits) because modern hardware can only address memory at the byte level.
  - Even in arrays, each `bool` still takes 1 byte; Rust does not automatically optimize this by bit-packing.
    - For efficient storage of many booleans, you can use crates like [`bitvec`](https://github.com/ferrilab/bitvec) to
      store them bit by bit.
- Characters
  - We specify `char` with single quotes, e.g. `let c = 'z';`
  - Can be used for emojis as well, however, they must be a single Unicode scalar value, e.g.
    - `'😀'` is valid as it is a single Unicode scalar value
    - `'👍🏼'` is invalid as it is a compound Unicode scalar value (thumbs up + skin tone modifier)

## Vectors

- Stored on the heap and can grow or shrink at runtime (as opposed to arrays which are stored on the stack and must have
  a fixed size)
