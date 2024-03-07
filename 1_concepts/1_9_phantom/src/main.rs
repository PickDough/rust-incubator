use std::cell::UnsafeCell;
use std::marker::PhantomData;

struct Fact<T> {
    data: PhantomData<T>,
}

impl<T> Fact<T> {
    pub fn new() -> Self {
        Self { data: PhantomData }
    }
}

impl<T> Fact<Vec<T>> {
    fn fact(&self) -> &'static str {
        let facts = ["Vec is heap-allocated.", "Vec may re-allocate on growing."];

        facts[rand::random::<usize>() % facts.len()]
    }
}

impl Fact<usize> {
    fn fact(&self) -> &'static str {
        let facts = [
            "The size of usize is how many bytes it takes to reference any location in memory.",
            "26^4 − 1 on 64-bit targets is 18_446_744_073_709_551_615usize.",
        ];

        facts[rand::random::<usize>() % facts.len()]
    }
}

impl<T> Fact<UnsafeCell<T>> {
    fn fact(&self) -> &'static str {
        let facts = [
            "UnsafeCell.get() gives you a raw pointer *mut T to its contents. \
            It is up to you as the abstraction designer to use that raw pointer correctly.",
            "A &T reference can be released to safe code from UnsafeCell and there it can co-exist with other &T references, \
            but not with a &mut T.",
            "A &mut T reference may be released to safe code from UnsafeCell provided neither other &mut T nor &T co-exist with it. \
            A &mut T must always be unique."
        ];

        facts[rand::random::<usize>() % facts.len()]
    }
}

fn main() {
    let fact: Fact<Vec<i32>> = Fact::new();
    println!("Fact about Vec<T>: {}", fact.fact());

    let fact: Fact<usize> = Fact::new();
    println!("Fact about usize: {}", fact.fact());

    let fact: Fact<UnsafeCell<i32>> = Fact::new();
    println!("Fact about UnsafeCell<T>: {}", fact.fact());
}
