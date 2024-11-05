#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    ChangeColor,
    Echo,
    Move,
    Quit,
    Resize,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
