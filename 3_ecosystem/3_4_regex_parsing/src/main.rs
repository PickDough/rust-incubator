#![allow(dead_code, unused)]
use nom::AsBytes;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{alphanumeric1, digit1};
use nom::character::is_digit;
use nom::character::streaming::one_of;
use nom::combinator::{map, map_res, peek};
use nom::error::VerboseError;
use nom::sequence::{preceded, terminated, tuple};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let parsed = p_parse_precision(">+8.*");

    println!("{:?}", parsed);
}

fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    (
        p_parse_sign(input),
        p_parse_width(input),
        p_parse_precision(input),
    )
}

fn r_parse_sign(input: &str) -> Option<Sign> {
    static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[+-]").unwrap());
    REGEX
        .find(input)
        .map(|m| match m.as_str() {
            "+" => Some(Sign::Plus),
            "-" => Some(Sign::Minus),
            _ => unreachable!(),
        })
        .flatten()
}

fn p_parse_sign(input: &str) -> Option<Sign> {
    let parsed = tuple((
        (take_till(|c: char| c == '-' || c == '+')),
        one_of::<&str, &str, VerboseError<&str>>("-+"),
        peek(alt((digit1, tag(".")))),
    ))(input);

    match parsed {
        Ok((_, (_, sign, _))) => match sign {
            '+' => Some(Sign::Plus),
            '-' => Some(Sign::Minus),
            _ => unreachable!(),
        },
        Err(_) => None,
    }
}

fn take_out_last(s: &str) -> String {
    let b = s.as_bytes();
    std::str::from_utf8(
        b.iter()
            .take(b.len() - 1)
            .map(|b| b.clone())
            .collect::<Vec<_>>()
            .as_slice(),
    )
    .unwrap()
    .to_owned()
}

fn r_parse_width(input: &str) -> Option<usize> {
    static REGEX_INTEGER_OR_PARAMETER: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[0-9]+\$?").unwrap());

    REGEX_INTEGER_OR_PARAMETER
        .find(input)
        .map(|m| {
            let s = m.as_str();
            let mut res = s.parse::<usize>();
            if res.is_err() {
                res = take_out_last(s).parse::<usize>();
            }
            res.ok()
        })
        .flatten()
}

fn p_parse_width(input: &str) -> Option<usize> {
    let parsed = map_res(
        tuple((
            take_till(|c: char| is_digit(c as u8)),
            digit1::<&str, VerboseError<&str>>,
            peek(tag(".")),
        )),
        |(_, s, _): (&str, &str, &str)| s.parse(),
    )(input);

    match parsed {
        Ok((_, u)) => Some(u),
        Err(_) => None,
    }
}

// precision := (integer | identifier) '$' | integer | '*'
fn r_parse_precision(input: &str) -> Option<Precision> {
    static REGEX_INTEGER: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]").unwrap());
    static REGEX_ASTERISK: Lazy<Regex> = Lazy::new(|| Regex::new(r"\*").unwrap());
    static REGEX_PARAMETER: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(([a-zA-Z]|_)\w*|[0-9]+)\$").unwrap());

    REGEX_PARAMETER
        .find(input)
        .map(|m| Precision::Argument(m.as_str().to_owned()))
        .or_else(|| REGEX_ASTERISK.find(input).map(|_| Precision::Asterisk))
        .or_else(|| {
            REGEX_INTEGER
                .find(input)
                .map(|m| Precision::Integer(m.as_str().parse::<usize>().unwrap()))
        })
}

fn p_parse_precision(input: &str) -> Option<Precision> {
    let parsed = preceded(
        tuple((take_till(|c| c == '.'), tag("."))),
        map(
            alt((
                map(tag("*"), |s: &str| s.to_owned()),
                map(terminated(alt((tag("_"), alphanumeric1)), tag("$")), |s| format!("{}$", s)),
                map(digit1::<&str, VerboseError<&str>>, |s| s.to_owned()),
            )),
            |s| match s.as_str() {
                "*" => Precision::Asterisk,
                sl if s.ends_with("$")  => Precision::Argument(s),
                sl => Precision::Integer(s.parse::<usize>().unwrap()),
            },
        ),
    )(input);

    parsed.ok().map(|(_, p)| p)
}

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(String),
    Asterisk,
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in [
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in [
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse(input);
            assert_eq!(width, expected);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in [
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument("1$".to_owned()))),
            // Task is not clear. There are no requirements for precision to have a leading '.'
            // in order to be a valid precision. It's just an '*', an integer or a parameter.
            // By naively implement precision any of the following inputs will pass:
            // ["axz*", "32", "_format$", "2$"].
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse(input);
            assert_eq!(precision, expected);
        }
    }
}
