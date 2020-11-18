use regex::Regex;
use std::io;

// fn validate_date_format(date: &str) {
// // check if date is of struct Date if not throw an error
// let re = Regex::new(r"^\d\d/\d\d/\d\d\d\d$").unwrap();
// if re.is_match(date) {
// println!(
// "the date given is not properly formatted,\nplease try again with the correct fomat."
// );
// }
// }

// fn validate_date_greater_than_today(date: &str) {
// // check if date is greater than Date::now if not throw an error
// }

// fn calculate_time_left(_date: String) -> i32 {
// // date - now
// // let now = Utc::now;
// let days_left: i32 = 0;
//
// return days_left;
// }

fn main() {
    println!("give us the date and we'll tell you how much time is left for it.");
    println!("please follow this format: mm/dd/yy.");

    let mut date = String::new();

    loop {
        io::stdin()
            .read_line(&mut date)
            .expect("Failed to read line");
        let date = date.trim();
        let re = Regex::new(r"^\d\d/\d\d/\d\d\d\d$").unwrap();
        if !re.is_match(date) {
            println!("the date given is not properly formatted,\nplease try again with the correct fomat.");
        } else {
            break;
        }
    }

    // let time_left = calculate_time_left(date);
    println!("{}, is how much time left in days for it.", date);
}
