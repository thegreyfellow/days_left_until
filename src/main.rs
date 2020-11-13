use std::io;

// fn validate_date(_date: &String) -> bool {
// // check if date is of struct Date if not throw an error
// // check if date greater than now if not throw an error
// return true;
// }
//
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

    let mut date = String::new().to_string();
    io::stdin()
        .read_line(&mut date)
        .expect("Failed to read line");

    let date = date.trim();

    // validate_date(&date);

    // let time_left = calculate_time_left(date);
    println!("{}, is how much time left in days for it.", date);
}
