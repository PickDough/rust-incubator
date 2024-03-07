use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::thread;

// Have no idea how to implement it safely.
#[derive(Debug, Default)]
struct OnlySync {
    data: PhantomData<Rc<i32>>,
}
unsafe impl Sync for OnlySync {}

#[derive(Debug, Default)]
struct OnlySend {
    data: PhantomData<Cell<i32>>,
}

#[derive(Debug, Default, Copy, Clone)]
struct SyncAndSend {
    data: PhantomData<i32>,
}

#[derive(Debug, Default)]
struct NotSyncNotSend {
    data: PhantomData<Rc<Cell<i32>>>,
}

fn main() {
    let only_sync = OnlySync::default();
    let only_send = OnlySend::default();
    let sync_and_send = SyncAndSend::default();
    let not_sync_not_send = NotSyncNotSend::default();

    thread::scope(|s| {
        // ** FAILS **
        // s.spawn(move || {
        //     println!("{:?}", only_sync);
        // });
        s.spawn(|| {
            println!("{:?}", only_sync);
        });
        // ** FAILS **
        // s.spawn(|| {
        //     println!("{:?}", only_send)
        // })
        s.spawn(move || {
            println!("{:?}", only_send);
        });
        s.spawn(|| {
            println!("{:?}", sync_and_send);
        });
        s.spawn(move || {
            println!("{:?}", sync_and_send);
        });
        // ** FAILS **
        // s.spawn(|| {
        //     println!("{:?}", not_sync_not_send);
        // });
        // ** FAILS **
        // s.spawn(move || {
        //     println!("{:?}", not_sync_not_send);
        // });
    });
}
