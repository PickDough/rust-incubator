use std::{
    cmp::Ordering,
    env,
    io::{self, BufRead},
};

fn main() {
    println!("Guess the number!");

    let secret_number = get_secret_number(env::args());

    loop {
        println!("Please input your guess.");

        let guess = match get_guess_number(&mut io::stdin().lock()) {
            Some(n) => n,
            _ => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_secret_number<I: Iterator<Item = String>>(config: I) -> u32 {
    let secret_number = config
        .skip(1)
        .take(1)
        .last()
        .expect("No secret number is specified");
    secret_number
        .trim()
        .parse()
        .expect("Secret number is not a number")
}

fn get_guess_number<R: BufRead>(read_buff: &mut R) -> Option<u32> {
    let mut guess = String::new();
    read_buff
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use test_case::test_case;

    use crate::{get_guess_number, get_secret_number};

    #[test]
    #[should_panic(expected = "No secret number is specified")]
    fn test_no_secret_number() {
        get_secret_number(vec!["".to_owned()].into_iter());
    }

    #[test]
    #[should_panic(expected = "Secret number is not a number")]
    fn test_secret_number_nan() {
        get_secret_number(vec!["number".to_owned(), "64i32".to_owned()].into_iter());
    }

    #[test]
    fn test_secret_number() {
        assert_eq!(
            64,
            get_secret_number(vec!["number".to_owned(), "64".to_owned()].into_iter())
        );
    }

    #[test_case("sixty two")]
    #[test_case("6_3")]
    #[test_case("6.4")]
    fn test_get_guess_number_none(str: &str) {
        assert_eq!(
            None,
            get_guess_number(
                &mut (str
                    .as_bytes()
                    .iter()
                    .map(Clone::clone)
                    .collect::<VecDeque<u8>>()),
            )
        );
    }

    #[test]
    fn test_get_guess_number_some() {
        assert_eq!(
            Some(64),
            get_guess_number(
                &mut ("64"
                    .as_bytes()
                    .iter()
                    .map(Clone::clone)
                    .collect::<VecDeque<u8>>()),
            )
        );
    }
}
