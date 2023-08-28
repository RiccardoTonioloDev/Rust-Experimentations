enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

enum IpAddrEnumVersion {
    V4(String),
    V6(String)
}

enum IpAddrDiffTypes {
    V4(u8,u8,u8,u8),
    V6(String)
}

enum Message{
    Quit, //nessun dato associato
    Move {x:i32, y:i32}, //il dato associato è una struttura anonima
    Write(String), //il dato associato è una stringa
    ChangeColor(i32,i32,i32) //il dato associato è una tupla di tre valori interi
}
impl Message{
    fn call(&self){
        println!("this is a call to method on an enum!");
    }
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    California,
    //others
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let _four = IpAddrKind::V4; 
    let _six = IpAddrKind::V6;

    //stesso tipo, valori diversi
    route(_four);    
    route(_six);    

    let _home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let _loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let _home = IpAddrEnumVersion::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrEnumVersion::V6(String::from("::1"));

    route_enum(_home);
    route_enum(_loopback);

    let msg1 = Message::Move { x: 31, y: 52 };
    let msg2 = Message::Quit;
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(1, 2, 3);

    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();

    let some_number = Some(5);
    let a_number = 5;
    let some_string = Some(String::from("hello"));

    let absent_number: Option<i32> = None;

    //let sum_of_numbers = some_number + a_number;

    let pnn = Coin::Penny;
    let nckl = Coin::Nickel;
    let qrtr = Coin::Quarter(UsState::Alabama);

    println!("A penny is {} cents",value_in_cents(&pnn));
    println!("A nickel is {} cents",value_in_cents(&nckl));
    println!("A nickel is {} cents",value_in_cents(&qrtr));

    let _six = plus_one(Some(5));
    let _none = plus_one(None);

    trying_default(qrtr);
    trying_default(pnn);

    let some_u8_value = Some(3);

    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn route(_ip_kind: IpAddrKind){
}

fn route_enum(_ip_kind: IpAddrEnumVersion){
}

fn value_in_cents(coin: &Coin) -> i32{
    match coin{
        Coin::Penny => 1,
        Coin::Dime => 5,
        Coin::Nickel => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!",state);
            25
        }
    }
}

fn plus_one(num: Option<i32>) -> Option<i32>{
    match num {
        None => None,
        Some(n) => Some(n + 1)
    }
}

fn trying_default(cn: Coin){
    match cn {
        Coin::Penny => println!("It's a Penny"),
        Coin::Dime => println!("It's a Dime"),
        _ => println!("Other coins!")
    };
}
