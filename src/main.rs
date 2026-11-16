use std::time::{SystemTime};

fn main() {
    println!("Calendar");
    let now = SystemTime::now();
    println!("{:#?}", now)
}
