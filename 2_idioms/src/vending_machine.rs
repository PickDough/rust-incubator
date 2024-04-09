#![allow(dead_code, unused)]
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use crate::money::Money;
use crate::vending_machine::VendingError::{NoChange, NoProduct, NotEnoughMoney};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Product(String);

impl Product {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self { 0: s.into() }
    }
}

#[derive(Debug, Clone)]
pub struct ProductMap {
    name: Product,
    price: Money,
}

impl PartialEq for ProductMap {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for ProductMap {}

impl Hash for ProductMap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Display for ProductMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, {}", self.name.0, self.price)
    }
}

impl ProductMap {
    pub fn new<S: Into<String>, P: Into<Money>>(name: S, price: P) -> Self {
        Self {
            name: Product::new(name),
            price: price.into(),
        }
    }
}

#[derive(Debug)]
pub struct VendingMachine {
    products: HashMap<ProductMap, u64>,
    money: Money,
}

impl Display for VendingMachine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for kv in &self.products {
            write!(f, "{} - {} item(s).\n", kv.0, kv.1).unwrap();
        }
        write!(f, "Money in the machine: {}", self.money)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VendingError {
    NoProduct(Product),
    NoChange { change: Money, money: Money },
    NotEnoughMoney { money: Money, product: ProductMap },
}

impl Display for VendingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VendingError::NoProduct(p) => {
                write!(f, "Product [{}] ran out.", p.0)
            }
            VendingError::NoChange { change, money } => {
                write!(f, "Couldn't give change: [{change}], having: [{money}]")
            }
            VendingError::NotEnoughMoney { money, product } => {
                write!(f, "Not enough money: [{money}], for : [{product}]")
            }
        }
    }
}

impl Error for VendingError {}

impl VendingMachine {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
            money: Money::new(),
        }
    }

    pub fn fill<P: Into<Money>>(&mut self, products: &[ProductMap], money: P) {
        for p in products {
            self.products
                .entry(p.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        self.money = self.money.clone() + money.into()
    }

    pub fn purchase<M: Into<Money>>(
        &mut self,
        p: &Product,
        money: M,
    ) -> Result<(ProductMap, Money), VendingError> {
        let count = self
            .products
            .get_mut(&ProductMap {
                name: p.clone(),
                price: Money::new(),
            })
            .ok_or(NoProduct(p.clone()));
        if count.is_err() {
            return Err(count.err().unwrap());
        }
        let mut count = *count.unwrap();
        let p_map = self
            .products
            .get_key_value(&ProductMap {
                name: p.clone(),
                price: Money::new(),
            })
            .unwrap()
            .0
            .clone();
        let money = money.into();

        if money.value() < p_map.price.value() {
            return Err(NotEnoughMoney {
                product: p_map.clone(),
                money: money.clone(),
            });
        }

        let change = money.value() - p_map.price.value();
        self.money = self.money.clone() + money.clone();
        let can_give_change = self.money.find_subset(change);

        match can_give_change {
            Some(c) => {
                self.money = (self.money.clone() - c.clone()).unwrap();
                count -= 1;
                if count == 0 {
                    self.products.remove(&p_map);
                }

                Ok((p_map.clone(), c))
            }
            None => {
                self.money = (self.money.clone() - money).unwrap();

                Err(NoChange {
                    change: change.into(),
                    money: self.money.clone(),
                })
            }
        }
    }
}
