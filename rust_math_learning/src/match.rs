//fn main(){
//    let number = 3;
//    match number {
//        1 => println!("One"),
//        2 => println!("Two"),
//        3 => println!("Three"),
//        _ => println!("Other"),   // _ pattern is a wildcard that catches any unmatched value.
//    }
//}  

// MATCHING ranges
//fn main(){
//    let score = 85;
//    match score {
//        90..=100 => println!("A"),
//        80..=89 => println!("B"),
//        70..=79 => println!("C"),
//        _ =>println!("F")
//    }
//}
//


// MATCHING ENUMS
//enum Coin{
//    Penny,
//    Nickel,
//    Dime,
//    Quarter,
//}

//fn value_in_cents(coin:Coin) -> u8 {
//    match coin {
//        Coin::Penny =>1,
//        Coin::Nickel => 5,
//        Coin::Dime => 10,
//        Coin::Quarter => 25,
//    }
//}




//fn main(){
//    let coin = Coin::Dime;
//    println!("Value: {} cents", value_in_cents(coin));
//}

//---------------------------------------------------------
//
// Vector Match
fn main() {
    let vec: Vec<i32> = (1..=5).collect();
    for i in vec {
        match i {
            1 => print!("One "),
            2..=4 => print!("Mid Range "),
            _ => print!("Other "),
        }
    }
    println!();
}
