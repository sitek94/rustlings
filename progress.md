# Progress

## Day 17

Finished reading about `Option` enum, finished 6.1 chapter at the same time.
https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values

Next: https://doc.rust-lang.org/book/ch06-02-match.html

## Day 18

Seems like `match` is very similar to `switch` in JS

Next: https://doc.rust-lang.org/book/ch06-02-match.html#patterns-that-bind-to-values

## Day 19

Interesting, practical use-case for enums to hold data inside it:
https://doc.rust-lang.org/book/ch06-02-match.html#patterns-that-bind-to-values

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Historically only quarters have different designs for
                      // each of the 50 states
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

So basically, Rust's pattern matching is like JS switch on steroids.

## Day 20 - `match`

- Matching with Option enum
- Matches are exhaustive
- Catch all

  ```rust
  let dice_roll = 9;
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),

    // 1. When you want to do something with the value
    other => move_player(other),

    // 2. Alternatively use `_` as placeholder when you aren't going to use
    // the bound value
    _ => reroll(),

    // 3. Lastly, you can just do nothing using the unit value `()`
    _ => (),
  }

  fn add_fancy_hat() {}
  fn remove_fancy_hat() {}
  fn move_player(num_spaces: u8) {}
  fn reroll() {}
  ```

Next: https://doc.rust-lang.org/book/ch06-03-if-let.html

## Day 21 - `if let`

- `if let`

  ```rust
  // Using `match` expression:
  let mut count = 0;
  match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
  }

  // Or `if let` and `else` expression:
  let mut count = 0;
  if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
  } else {
    count += 1;
  }
  ```

- `let`-`else` example:

  ```rust
  fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
  }
  ```

  The usefulness of it is especially visible when trying to do something similar in JS:

  ```js
  function describe_state_quarter(coin: Coin) {
    // In Rust all of this happens in one step: matching + extraction + early
    // return. No need for extra type guard helper.
    if (!is_quarter(coin)) {
      // In Rust compiler enforces that the else branch must exit the current
      // scope (return, break, etc.)
      return null;
    }
    const state = coin.state;

    // ...
  }
  ```

Next: start `09_strings`

## Day 22 - Strings

Next:
https://doc.rust-lang.org/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro

## Day 23

### Concatenation with the + Operator

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
// When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]
// `add` does not take ownership of `s2`
// However, it takes ownership of `s1`
let s3 = s1 + &s2;
```

Concatenating multiple strings:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

// We can do it like this but it's a bit verbose:
let s = s1 + "-" + &s2 + "-" + &s3;

// Instead, we can use `format!` macro:
let s = format!("{s1}-{s2}-{s3}");
// format! macro uses references so that this call doesn’t take ownership of
// any of its parameters
```

### Indexing into Strings

https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings

```rust
let s1 = String::from("hello");
let h = s1[0]; // ❌ This will not compile
```

TLDR; Rust strings don't support indexing.

Next, find out why: https://doc.rust-lang.org/book/ch08-02-strings.html#internal-representation

## Day 24 - Strings and UTF-8

```rust
let hello = "Здравствуйте";
let answer = &hello[0];
```

Rust won't compile this because the first Cyrillic letter `З` takes up 2 bytes (208 151 in UTF-8). If Rust let us grab index 0, we'd get just 208, which is only half the character - that's pretty useless and could cause bugs.

This is also why the string length is 24 bytes even though it's only 12 characters - each Cyrillic letter needs 2 bytes.

Next: https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my
