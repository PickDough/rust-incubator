use rand;
use std::{borrow::Borrow, ops::Deref};

use once_cell::sync::Lazy;
use regex::Regex;

struct EmailString(String);

impl TryFrom<&str> for EmailString {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
            )
            .unwrap()
        });

        if EMAIL_REGEX.is_match(&value) {
            Ok(EmailString(value.to_owned()))
        } else {
            Err("the string is not a valid email address")
        }
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for EmailString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

struct Random<T>(Box<[T]>);

impl<T> Random<T> {
    fn new(t1: T, t2: T, t3: T) -> Self {
        Self(Box::new([t1, t2, t3]))
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0[rand::random::<usize>() % 3]
    }
}

impl<T> AsRef<T> for Random<T>
{
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

fn main() {
    let email: EmailString = "sodapumpkin@icloud.com".try_into().unwrap();

    let s: &str = email.borrow();
    println!("{}", s);

    let r = Random::new(42, 69, 420);
    println!("{}", r.deref());
    println!("{}", r.deref());
    println!("{}", r.as_ref());
}
