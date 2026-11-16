use chrono;
use chrono::Datelike;

fn days(num: u32) -> chrono::Days {
    chrono::Days::new(num as u64)
}

fn main() {
    let now = chrono::offset::Local::now();
    let first = now.with_day(1).unwrap();
    let first_day = first.weekday().number_from_monday();
    let week_number = first.iso_week().week();
    let prev_month_days = first.checked_sub_days(days(1)) // get prev month
                               .unwrap()
                               .num_days_in_month() as u32;

    // Print headers
    println!("{: ^23}", now.format("%B %Y"));
    println!("   Mo Tu We Th Fr Sa Su");

    // Print first line
    print!("{: >2} ", week_number);
    for i in 0..(first_day - 1) {
        print!("\x1b[2m{: >2}\x1b[0m ", prev_month_days - (first_day - 2 - i));
    }
    for i in (first_day - 1)..7 {
        if now.day() == i {
            print!("\x1b[94m{: >2}\x1b[0m ", first_day - i)
        } else {
            print!("{: >2} ", first_day - i);
        }
    }
    println!();

    // Print remaining lines
    let next_day = 7 - first_day;
    let mut next = first.checked_add_days(days(next_day)).unwrap();
    let one_week = days(7);
    while next.month() == now.month() {

        print!("{: >2} ", next.iso_week().week());
        let num_days_in_month = next.num_days_in_month() as u32;

        for i in 1..8 {
            let today = next.day() + i;
            if today > num_days_in_month {
                print!("\x1b[2m{: >2}\x1b[0m ", today - num_days_in_month);
            } else if now.day() == today {
                print!("\x1b[94m{: >2}\x1b[0m ", today)
            } else {
                print!("{: >2} ", today);
            }
        }
        println!();
        next = next.checked_add_days(one_week).unwrap();
    }
}
