use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(v: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(v),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }
}

#[test]
fn test() {
    use std::thread;
    let lock = SpinLock::new(vec![]);
    thread::scope(|s| {
        s.spawn(|| lock.lock().push(42));
        s.spawn(|| {
            let mut v = lock.lock();
            v.push(1);
            v.push(2);
        });
        s.spawn(|| {
            let mut v = lock.lock();
            v.push(3);
            v.push(4);
        });
    });

    let v = lock.lock();
    let v = v.as_slice();
    assert!(
        v == [1, 2, 3, 4, 42]
            || v == [1, 2, 42, 3, 4]
            || v == [3, 4, 42, 1, 2]
            || v == [3, 4, 1, 2, 42]
            || v == [42, 3, 4, 1, 2]
            || v == [42, 1, 2, 3, 4]
    );
}
