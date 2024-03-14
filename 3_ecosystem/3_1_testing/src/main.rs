use std::{cmp::Ordering, env, io::{self, BufRead, Read}};

fn main() {
    println!("Guess the number!");

    let secret_number = get_secret_number();

    loop {
        println!("Please input your guess.");

        let guess = match get_guess_number() {
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
        .ok()
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
    use std::io;

    #[test]
    fn test() {
        
    }
}