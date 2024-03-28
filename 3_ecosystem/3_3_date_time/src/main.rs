#![allow(dead_code, unused)]
use chrono::{format::strftime, Local, NaiveDate};
fn main() {
    println!("Implement me!");
}

const NOW: &str = "2019-06-26";

struct User {
    birthday: NaiveDate,
}

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        Self {
            birthday: NaiveDate::from_ymd_opt(year, month, day).unwrap(),
        }
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        NaiveDate::parse_from_str(NOW, "%Y-%m-%d")
            .unwrap()
            .years_since(self.birthday.clone())
            .unwrap_or(0) as u16
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in [
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in [
            ((2032, 6, 25), 0),
            ((2020, 6, 27), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }
}
