use time::{Date, Month};

fn main() {
    let start = Date::from_calendar_date(2023, Month::January, 1).unwrap();
    let end = Date::from_calendar_date(2023, Month::April, 13).unwrap();

    let diff = end - start;

    println!("{} days", diff.whole_days());
}
