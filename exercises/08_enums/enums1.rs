#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}

#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}

fn main() {
    // experiments
    let four = IpAddressKind::V4;

    fn route(ip: IpAddressKind) {
        println!("{:?}", ip);
    }

    route(IpAddressKind::V4);
    route(IpAddressKind::V6);
    route(four);

    //////

    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    // TODO: FInish reading about enum methods

    // Exercise tests
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
