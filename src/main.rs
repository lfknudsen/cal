use chrono;
use chrono::Datelike;

fn main() {
    println!("Calendar");
    println!("   Mo Tu We Th Fr Sa Su");
    let now = chrono::offset::Local::now();
    let first = now.date_naive().with_day(1).unwrap();
    let first_day = first.weekday().number_from_monday();
    let week_number = first.iso_week().week();
    //let w = now.date_naive().day();
    print!("{: >2} ", week_number);
    for _ in 0..(first_day - 1) {
        print!("   ");
    }
    for i in (first_day - 1)..7 {
        if now.day() == i as u32 {
            print!("\033[94m{: >2}\033[0m ", first_day - i)
        } else {
            print!("{: >2} ", first_day - i);
        }
    }
    println!()

}
