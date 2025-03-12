#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize { width: u32, height: u32 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("some_number: {some_number:?}");
    println!("some_char: {some_char:?}");
    println!("absent_number: {absent_number:?}");
    ///////////

    println!("Match Option enum: {:?}", match_option_enum());

    ///////////
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

// Matching with Option enum
fn match_option_enum() -> Option<i8> {
    let x: i8 = 5;
    let y = Some(5);

    match y {
        // Matches are exhaustive
        None => None,

        // Binding value
        Some(sth) => {
            let sum = sth + x;
            println!("{}", sum);
            Some(sum)
        }
    }
}
