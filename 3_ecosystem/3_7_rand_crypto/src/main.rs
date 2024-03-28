#![allow(dead_code, unused)]
use argon2::Config;
use base64ct::{Base64, Encoding};
use rand::distributions::{Alphanumeric, DistString};
use rand::random;
use sha3::{Digest, Sha3_256};
use std::path::Path;

fn main() {
    println!("{}", get_file_hash(Path::new("./Cargo.toml")));
}

// generates random password of given length and symbols set;
fn generate_password(n: usize, symbols: &str) -> String {
    let mut password = String::new();
    let chars: Vec<char> = symbols.chars().collect();
    for _ in 0..n {
        let idx = random::<usize>() % symbols.len();
        password.push(chars[idx]);
    }

    password
}

// retrieves random element from a given slice;
fn select_rand_val<T>(slice: &[T]) -> &T {
    let idx = random::<usize>() % slice.len();

    &slice[idx]
}

// generates unique cryptographically secure random value in a-zA-Z0-9 symbols set and has exactly 64 symbols.
fn new_access_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 64)
}

// returns SHA-3 hash of a file specified by its path.
fn get_file_hash(mut path: &Path) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(path.to_str().unwrap());

    let hash = hasher.finalize();

    Base64::encode_string(&hash)
}

// returns Argon2 password hash for a given password.
fn hash_password(password: &str) -> String {
    let salt = b"randomsalt";
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}
