// enum IpAddrKind {
//     V4,
//     V6,
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    // fn call(&self) {
        
    // }
}

enum WebEvents {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64}
}

enum Number {
    Zero,
    One,
}

enum Color {
    Red = 0xff0000,
    Blue = 0x0000ff
}



fn inspect(event: WebEvents) {
    match event {
        WebEvents::PageLoad => println!("Page loaded"),
        WebEvents::PageUnload => println!("Page Unloaded"),
        WebEvents::KeyPress(c) => println!("The key pressed is {}", c),
        WebEvents::Paste(text) => println!("Pasted \"{}\".", text),
        WebEvents::Click { x, y } => println!("Point on screen clicked is x={}; y={}", x, y)
    }
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let pressed = WebEvents::KeyPress('t');
    let pasted = WebEvents::Paste("decent text".to_owned());
    let clicked = WebEvents::Click { x: 20, y: 35 };
    let load = WebEvents::PageLoad;
    let unload = WebEvents::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(clicked);
    inspect(load);
    inspect(unload);

    use crate::Color::*;
    use crate::Number::{Zero, One};

    println!("Zero is {}", Zero as i32);
    println!("One is {}", One as i32);

    println!("roses are #{:06x}", Red as i32);
    println!("violets are #{:06x}", Blue as i32);



    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // }
}
