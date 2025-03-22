# Rust

## Variables

- `mut` is used to make a variable mutable
- `const` declare a constant, must be annotated with a type
- shadowing allows you to redeclare a variable with the same name, useful for changing the type of a
  variable
  ```rust
  let spaces = "   ";
  let spaces = spaces.len();
  ```

## Primitive Types

- Booleans are one byte in size
  - Each `bool` uses 1 byte (8 bits) because modern hardware can only address memory at the byte
    level.
  - Even in arrays, each `bool` still takes 1 byte; Rust does not automatically optimize this by
    bit-packing.
    - For efficient storage of many booleans, you can use crates like
      [`bitvec`](https://github.com/ferrilab/bitvec) to store them bit by bit.
- Characters
  - We specify `char` with single quotes, e.g. `let c = 'z';`
  - Can be used for emojis as well, however, they must be a single Unicode scalar value, e.g.
    - `'😀'` is valid as it is a single Unicode scalar value
    - `'👍🏼'` is invalid as it is a compound Unicode scalar value (thumbs up + skin tone modifier)

## Vectors

- Stored on the heap and can grow or shrink at runtime (as opposed to arrays which are stored on the
  stack and must have a fixed size)

## Ownership

- All programs must manage their memory one way or another
  - Garbage collection regularly looks for no-longer-used memory as the program runs (e.g.
    JavaScript)
  - in other languages, the programmer must explicitly allocate and free the memory (e.g. C's famous
    `malloc`)
- **Rust** uses different approach called Ownership
  - memory is managed through a system of ownership with a set of rules that the compiler checks
- Ownership rules
  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.
  - https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy

## The Stack and the Heap

- The stack
  - stores values in the order it gets them and removes the values in the opposite order (last in,
    first out)
  - Adding data is called pushing onto the stack, and removing data is called popping off the stack.
  - All data stored on the stack must have a known, fixed size.
- The heap
  - less organized
  - when you put data on the heap, you request a certain amount of space. The memory allocator finds
    an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer,
    which is the address of that location. This process is called allocating on the heap and is
    sometimes abbreviated as just allocating
  - Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack,
    but when you want the actual data, you must follow the pointer.
  - Accessing data in the heap is slower than accessing data on the stack because you have to follow
    a pointer to get there.

## Stack-Only Data: Copy

Some of the types that implement Copy:

- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating-point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example, (`i32`, `i32`)
  implements `Copy`, but (`i32`, `String`) does not.

## Structs

- [ ] [Unit-Like Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields) -
      I'm not sure in what situations these would be useful, let's revisit it after some time
- Methods
  - I was wondering why methods are not defined in the struct definition, but rather in a separate
    impl (implementation) block and apparently, there are many reasons for this, some of them are:
    - separation of concerns - struct defines the data, impl block defines the behavior
    - flexibility and organization - you can have multiple impl blocks for the same type, each
      grouping some behaviors, and they can be live in separate modules or files
    - [ ] consistency - the same syntax is used for Traits, I haven't read about Traits, so I'll
          revisit it later

## Printing

- Two main ways to print values:
  - `Display` formatting with `{}` - for user-friendly output
  - `Debug` formatting with `{:?}` or pretty-printed with `{:#?}` - for debugging
- Types that implement `Display` by default:
  - All primitive types (`i32`, `f64`, `bool`, `char`, etc.)
  - `&str` and `String`
  - References to types that implement `Display`
- Many types don't implement `Display` by default (like structs, enums)
- For custom types, you need to:
  - Add `#[derive(Debug)]` to enable debug printing
  - Implement `Display` trait manually for custom display formatting

## References

- https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references
- [ ] [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
