fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    //  ^^^ added

    vec.push(88);

    vec
}

fn main() {
    let s = String::from("hello"); // s comes into scope

    let s = takes_and_gives_back_ownership(s); // s's value moves into the function...
                                               // ... and so is no longer valid here
    println!("{}", s);

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("{}", x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_and_gives_back_ownership(some_string: String) -> String {
    // some_string comes into scope
    println!("{some_string}");

    some_string
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // `vec0` can't be accessed anymore because it is moved to `fill_vec`.
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
