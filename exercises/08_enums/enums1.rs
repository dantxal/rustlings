// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    // like tuple structs,
    Echo(char),
    Move(String),
    // or c-like structures.
    ChangeColor { red: i32, green: i32, blue: i32 },
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo('c'));
    println!("{:?}", Message::Move("Somewhere".to_string()));
    println!(
        "{:?}",
        Message::ChangeColor {
            red: 0,
            green: 244,
            blue: 100
        }
    );
}
