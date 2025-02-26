fn main() {
    let s1 = "str 1";
    let s2 = "str 2";

    println!("Basic printing:");
    println!("{}", s1);

    println!("Named parameter:");
    println!("{s1}");

    println!("Multiple named parameters:");
    println!("{s2}, {s1}");

    println!("Debug printing:");
    println!("{:?}", s1);

    println!("Named debug printing:");
    println!("{s1:?}");

    // The above is possible because `&str` implements Display Trait
    // However it wouldn't work with e.g. Enum unless we implement Display ourselves
    // or we use debug printing by deriving it from `Debug`
    #[derive(Debug)]
    enum Mode {
        Dev,
    }
    // Wouldn't work
    // println!("{}", Mode::Dev);

    // This works
    println!("{:?}", Mode::Dev);

    // Other examples
    println!("Display: {}", 42); // uses Display
    println!("Debug: {:?}", vec![1, 2, 3]); // uses Debug

    #[derive(Debug)]
    struct Point(i32, i32);

    println!("{:?}", Point(1, 2)); // Debug: Point(1, 2)

    // Pretty printing makes a difference for complex structures:
    let complex = vec![Point(1, 2), Point(3, 4)];
    println!("{:#?}", complex); // Prints with nice indentation
}
