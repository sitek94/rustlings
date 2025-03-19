// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    // `to_string` creates a `String` from string literal
    "blue".to_string()

    // We could alternatively do:
    // String::from("blue")
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");

    // Experiments
    let mut s1 = String::from("foo");
    // let s2 = "bar";
    let s2 = "bar";
    s1.push_str(s2);
    println!("We can still use s2: {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3: {s3}");

    // Strings and UTF-8
    let ze = 'З';
    println!("\n\nCyrillic letter Ze");
    println!("As bytes: ");
    for byte in ze.to_string().as_bytes() {
        print!("{} ", byte); // Will print 208 151
    }

    println!("\nAs hex: ");
    for byte in ze.to_string().as_bytes() {
        print!("{:02X} ", byte); // Will print D0 97
    }
}
