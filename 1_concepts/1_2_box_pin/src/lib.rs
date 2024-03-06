use std::{fmt, mem, pin::Pin};

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T: fmt::Debug> SayHi for T {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

impl<T: Default> MutMeSomehow for T {
    fn mut_me_somehow(self: Pin<&mut Self>) {
       unsafe {
          mem::take(self.get_unchecked_mut());
       }
    }
}