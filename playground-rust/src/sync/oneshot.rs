use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::Arc;
use std::thread;
use std::thread::Thread;

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
    receiving_thread: Thread,
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe { (*self.inner.value.get()).write(message) };
        self.inner.ready.store(true, Release);
        self.receiving_thread.unpark();
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn receive(self) -> T {
        while !self.inner.ready.swap(false, Acquire) {
            thread::park();
        }
        unsafe { (*self.inner.value.get()).assume_init_read() }
    }
}

struct Inner<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
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
    });

    std::sync::mpsc::channel()(
        Sender {
            inner: inner.clone(),
            receiving_thread: thread::current(),
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
        });
        s.spawn(|| {
            assert_eq!(rx.receive(), 42);
        });
    })
}
