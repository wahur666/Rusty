fn main() {
    let four = IpAddrKind1::V4;
    let six = IpAddrKind1::V6;
    let home = IpAddr {
        kind: IpAddrKind1::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind1::V6,
        address: String::from("::1"),
    };

    let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind2::V6(String::from("::1"));

    let home = IpAddrKind3::V4(127, 0, 0, 1);
    let loopback = IpAddrKind3::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();
    let m = Message::Quit;
    m.call();

    let some_number = Some(5);
    let some_string = Some("A string");

    let absent_number: Option<i32> = None;

}

#[derive(Debug)]
enum IpAddrKind1 {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}


#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind1,
    address: String,
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Method body would be defined here
    }
}