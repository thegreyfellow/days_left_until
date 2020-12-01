use regex::Regex;
use std::io;

fn date_greater_than_tomorrow(_date: &str) -> bool {
    // check if date is greater than Date::now if not throw an error
    true
}

fn calculate_days_left(_date: &str) -> i32 {
    // date - now
    // let now = Utc::now;
    let days_left: i32 = 0;

    return days_left;
}

fn main() {
    println!("give us the date and we'll tell you how much time is left for it.");
    println!("please follow this format: mm-dd-yyyy.");

    let mut date = String::new();

    let date = loop {
        io::stdin()
            .read_line(&mut date)
            .expect("Failed to read line");
        let date = date.trim();
        let date_regex = Regex::new(r"^\d{2}-\d{2}-\d{4}$").unwrap();
        if !date_regex.is_match(date) {
            println!("the date given is not properly formatted,\nplease try again with the correct fomat.");
        } else {
            break date;
        }
    };

    if !date_greater_than_tomorrow(&date) {
        println!("the date given is old");
        println!("try with a date from the future :)");
    }

    let days_left = calculate_days_left(date);
    println!("{}, days left.", days_left);
}
