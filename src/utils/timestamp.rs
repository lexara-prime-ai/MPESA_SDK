use chrono::{Datelike, Local, Timelike};

fn main() {
    // Use [Local] instead of [Utc] depending on your region.
    let current_date = Local::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();
    let hour = current_date.hour();
    let minute = current_date.minute();
    let second = current_date.second();

    let formatted_date_time = format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}",
        year, month, day, hour, minute, second
    );

    println!("{}", formatted_date_time);
}
