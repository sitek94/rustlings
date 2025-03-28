// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // First solution, without checking the official one
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // 1? Was not sure if creating mutable Vec like this and then pushing
        // onto it is the correct way to do it
        let mut result: Vec<String> = Vec::new();

        // 2? Would it be possible to "destructure" and rename 0 and 1 from tuple
        // like in JS?
        for tuple in input {
            let string = tuple.0;
            let command = tuple.1;
            match command {
                Command::Uppercase => {
                    println!("uppercase");

                    // `to_uppercase` returns a new String because some characters might
                    // expand when converted to uppercase
                    result.push(string.to_uppercase());
                }
                Command::Trim => {
                    println!("trim");

                    // `trim` returns a str slice so we need to convert it to String
                    result.push(string.trim().to_string());
                }
                Command::Append(n) => {
                    println!("append x{n}");

                    // 3? Wasn't sure what's the best way to do it here. At the begging was
                    // trying to use sth like `concat, join or append` on string, but didn't
                    // really now how.
                    result.push(format!("{string}{}", "bar".repeat(n)))
                }
            }
        }

        result
    }

    pub fn transformer_take_2(input: Vec<(String, Command)>) -> Vec<String> {
        // 1. That's correct, it was actually exactly how they've done it in the solutions
        let mut output: Vec<String> = Vec::new();

        // 2. Yup, it's possible to destructure tuples:
        for (string, command) in input {
            // Extra: In solutions I've found this pattern, where we can assign the result
            // of `match` to a new variable. It's much simpler now, because I don't need
            // to push in each arm of match.
            let new_string = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),

                // 3. Ah, of course, this is another way how you can do it, just using `+`
                // But, `format!` macro was correct as well
                // More info in this chapter:
                // https://doc.rust-lang.org/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro
                Command::Append(n) => string + &"bar".repeat(n),
            };

            output.push(new_string);
        }

        output
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::my_module::transformer;
    use super::my_module::transformer_take_2;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }

    #[test]
    fn take_2_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer_take_2(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
