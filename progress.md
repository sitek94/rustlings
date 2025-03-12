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

## Day 20

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
