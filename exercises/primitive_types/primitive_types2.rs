// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

use std::any::Any;

fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '1'; // Finish this line like the example! What's your favorite character?
                              // Try a letter, try a number, try a special character, try a character
                              // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let char_number = '1';
    if char_number.is_ascii_digit() {
        println!("'1' is ascii digit");
    }

    let char_special = '!';
    if char_special.is_ascii_punctuation() {
        println!("'!' is punctuation");
    }
}
