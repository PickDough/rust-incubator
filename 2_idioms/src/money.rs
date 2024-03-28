use std::cmp::max;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

use crate::money::SealedCoin::{Cent, Dime, HalfDollar, Nickel, TwoCents, TwoDimes};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum SealedCoin {
    Cent(u64),
    TwoCents(u64),
    Nickel(u64),
    Dime(u64),
    TwoDimes(u64),
    HalfDollar(u64),
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Coin(SealedCoin);

impl Display for Coin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Cent(v) => {
                write!(f, "Cent({v})")
            }
            TwoCents(v) => {
                write!(f, "TwoCents({v})")
            }
            Nickel(v) => {
                write!(f, "Nickel({v})")
            }
            Dime(v) => {
                write!(f, "Dime({v})")
            }
            TwoDimes(v) => {
                write!(f, "TwoDimes({v})")
            }
            HalfDollar(v) => {
                write!(f, "HalfDollar({v})")
            }
        }
    }
}

impl Coin {
    pub fn value(&self) -> u64 {
        match &self.0 {
            Cent(v) => *v,
            TwoCents(v) => *v,
            Nickel(v) => *v,
            Dime(v) => *v,
            TwoDimes(v) => *v,
            HalfDollar(v) => *v,
        }
    }
}
/// 50
pub const HALF_DOLLAR: Coin = Coin(HalfDollar(50));
/// 20
pub const TWO_DIMES: Coin = Coin(TwoDimes(20));
/// 10
pub const DIME: Coin = Coin(Dime(10));
/// 5
pub const NICKEL: Coin = Coin(Nickel(5));
/// 2
pub const TWO_CENTS: Coin = Coin(TwoCents(2));
/// 1
pub const CENT: Coin = Coin(Cent(1));
const COINS: [Coin; 6] = [HALF_DOLLAR, TWO_DIMES, DIME, NICKEL, TWO_CENTS, CENT];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money(HashMap<Coin, u64>);

impl Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Money: ").unwrap();
        for (i, kv) in self.0.iter().enumerate() {
            if i == self.0.len() {
                for _ in 0..*kv.1 {
                    write!(f, "{}.", kv.0).unwrap();
                }
            } else {
                for _ in 0..*kv.1 {
                    write!(f, "{}, ", kv.0).unwrap();
                }
            }
        }

        write!(f, "")
    }
}

impl Add for Money {
    type Output = Money;

    fn add(mut self, rhs: Self) -> Self::Output {
        for kv in rhs.0 {
            self.increase(kv.0, kv.1);
        }

        self
    }
}

impl Sub for Money {
    type Output = Result<Self, ()>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for kv in rhs.0 {
            if self.decrease(&kv.0, kv.1).is_err() {
                return Err(());
            }
        }

        Ok(self)
    }
}

impl Money {
    pub fn new() -> Self {
        Self { 0: HashMap::new() }
    }

    pub fn increase(&mut self, coin: Coin, amount: u64) -> &mut Self {
        self.0
            .entry(coin)
            .and_modify(|e| *e = e.add(amount))
            .or_insert(amount);

        self
    }

    pub fn decrease(&mut self, coin: &Coin, amount: u64) -> Result<(), ()> {
        let val = self.0.get_mut(coin);

        if val.is_none() {
            Err(())
        } else {
            let val = val.unwrap();
            if *val < amount {
                return Err(());
            }

            *val -= amount;
            if *val == 0 {
                self.0.remove(coin);
            }

            Ok(())
        }
    }

    pub fn find_subset(&self, goal: u64) -> Option<Self> {
        let mut v = self
            .0
            .iter()
            .flat_map(|kv| vec![kv.0].repeat(*kv.1 as usize))
            .collect::<Vec<&Coin>>();
        v.sort_by_key(|k| k.value());
        v.reverse();

        let result = Self::find_subset_helper(vec![], v, goal);

        match result {
            Some(v) => {
                let mut coins: HashMap<Coin, u64> = HashMap::new();
                for c in v {
                    coins
                        .entry(c.clone())
                        .and_modify(|e| *e = e.add(1u64))
                        .or_insert(1u64);
                }

                Some(Self { 0: coins })
            }
            None => None,
        }
    }

    fn find_subset_helper<'a>(
        acc: Vec<&'a Coin>,
        mut v: Vec<&'a Coin>,
        goal: u64,
    ) -> Option<Vec<&'a Coin>> {
        if goal == 0 {
            return Some(acc);
        }

        v.retain(|c| c.value() <= goal);

        for i in 0..v.len() {
            let mut acc_clone = acc.clone();
            acc_clone.push(v[i]);
            let vv = v.iter().skip(i + 1).map(|f| *f).collect();

            let result = Self::find_subset_helper(acc_clone, vv, goal - v[i].value());
            match result {
                Some(r) => return Some(r),
                None => {}
            }
        }

        None
    }

    pub fn value(&self) -> u64 {
        self.0.iter().fold(0, |acc, kv| acc + kv.0.value() * kv.1)
    }

    fn max_coin(value: u64) -> Coin {
        *COINS
            .iter()
            .find(|&c| max(value, c.value()) == value)
            .unwrap()
    }

    fn construct(mut coins: HashMap<Coin, u64>, value: u64) -> HashMap<Coin, u64> {
        let max_coin = Self::max_coin(value);
        coins
            .entry(max_coin)
            .and_modify(|e| *e = e.add(1))
            .or_insert(1);

        let change = value.sub(max_coin.value());
        if change == 0 {
            coins
        } else {
            Self::construct(coins, change)
        }
    }
}

impl TryFrom<u64> for Coin {
    type Error = u64;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Coin(Cent(value))),
            2 => Ok(Coin(TwoCents(value))),
            5 => Ok(Coin(Nickel(value))),
            10 => Ok(Coin(Dime(value))),
            20 => Ok(Coin(TwoDimes(value))),
            50 => Ok(Coin(HalfDollar(value))),
            _ => Err(value),
        }
    }
}

impl From<u64> for Money {
    fn from(value: u64) -> Self {
        Money {
            0: Money::construct(HashMap::new(), value),
        }
    }
}
