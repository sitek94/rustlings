// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    // `.into()` converts a type into expected type
    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.

    // It will be a `&str` containing first one byte of "abc" which in this case will be "a"
    string_slice(&String::from("abc")[0..1]);

    // trim() doesn't need to mutate the string, can return a slice with omitted empty parts
    // at both ends
    string_slice("  hello there ".trim());

    // It mutates the original string so needs to clone it -> String
    string("Happy Monday!".replace("Mon", "Tues"));

    // Same as above for "Happy Monday!"
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
