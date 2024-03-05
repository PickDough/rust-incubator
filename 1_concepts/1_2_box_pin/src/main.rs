use std::{borrow::BorrowMut, fmt, future::Future, mem, pin::{pin, Pin}, rc::Rc, task::{Context, Poll}};
trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T: fmt::Debug> SayHi for Box<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T: fmt::Debug> SayHi for Rc<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T: fmt::Debug> SayHi for Vec<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl SayHi for String {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl SayHi for &[u8] {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

impl<T: Default> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        mem::take(self.get_mut().as_mut());
    }
}

impl<T: Default> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        mem::take(self.get_mut().borrow_mut());
    }
}

impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        self.get_mut().clear();
    }
}

impl<T> MutMeSomehow for Vec<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        unsafe {
            mem::take(self.get_unchecked_mut());
        }
    }
}

impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        unsafe {
            mem::take(self.get_unchecked_mut());
        }
    }
}

struct MeasurableFuture<Fut> {
    inner_future: Fut,
    started_at: Option<std::time::Instant>,
}

impl<Fut> MeasurableFuture<Fut> {
    fn get_started_at(self: Pin<&mut Self>) -> &mut Option<std::time::Instant> {
        unsafe {
           &mut self.get_unchecked_mut().started_at
        }
    }

    fn get_inner_future(self: Pin<&mut Self>) -> Pin<&mut Fut> {
        unsafe {
            self.map_unchecked_mut(|s| &mut s.inner_future)
        }
    }
}

impl<Fut> Future for MeasurableFuture<Fut> where Fut: Future<Output = ()> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.started_at.is_none() {
            *self.as_mut().get_started_at() = Some(std::time::Instant::now());
        } 
        match self.as_mut().get_inner_future().poll(cx) {
            Poll::Ready(()) => {
                println!("Elapsed time: {:?}", self.as_mut().get_started_at().unwrap().elapsed());
                Poll::Ready(())
            },
            Poll::Pending => Poll::Pending
        }
    }
}


#[tokio::main]
async fn main() {
    let mut five = Box::new(5);
    let mut pinned_five = pin!(five);
    pinned_five.as_ref().say_hi();
    pinned_five.as_mut().mut_me_somehow();
    pinned_five.as_ref().say_hi();

    let mut five = Rc::new(5);
    let mut pinned_five = pin!(five);
    pinned_five.as_ref().say_hi();
    pinned_five.as_mut().mut_me_somehow();
    pinned_five.as_ref().say_hi();

    let measurable_future = Box::pin(MeasurableFuture {
        inner_future: async { 
            std::thread::sleep(std::time::Duration::from_secs(2));
        },
        started_at: None,
    });
    measurable_future.await;
}

