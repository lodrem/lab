use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::Arc;
use std::thread;
use std::thread::Thread;

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe { (*self.inner.value.get()).write(message) };
        self.inner.ready.store(true, Release);

        if self.inner.receiving.load(Acquire) {
            let receiving_thread =
                unsafe { (*self.inner.receiving_thread.get()).assume_init_read() };
            receiving_thread.unpark();
            println!("send to {:?}", receiving_thread.id());
        }
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn receive(self) -> T {
        println!("recv from {:?}", thread::current().id());
        unsafe { (*self.inner.receiving_thread.get()).write(thread::current()) };
        self.inner.receiving.store(true, Release);
        while !self.inner.ready.swap(false, Acquire) {
            thread::park();
        }
        unsafe { (*self.inner.value.get()).assume_init_read() }
    }
}

struct Inner<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    receiving: AtomicBool,
    receiving_thread: UnsafeCell<MaybeUninit<Thread>>,
}

unsafe impl<T> Sync for Inner<T> where T: Send {}

impl<T> Drop for Inner<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.value.get_mut().assume_init_drop() }
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Arc::new(Inner {
        value: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
        receiving: AtomicBool::new(false),
        receiving_thread: UnsafeCell::new(MaybeUninit::uninit()),
    });

    (
        Sender {
            inner: inner.clone(),
        },
        Receiver { inner },
    )
}

#[test]
fn test() {
    let (tx, rx) = channel();
    thread::scope(|s| {
        s.spawn(|| {
            tx.send(42);
            println!("sent");
        });
        s.spawn(|| {
            assert_eq!(rx.receive(), 42);
            println!("received");
        });
    });
}
