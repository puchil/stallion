//! Synchronization primitives.
//!
//! # Resources
//!
//!   - <https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html>
//!   - <https://stackoverflow.com/questions/59428096/understanding-the-send-trait>
//!   - <https://doc.rust-lang.org/std/cell/index.html>

use core::cell::UnsafeCell;

pub mod interface {
    pub trait Mutex {
        type Data;
        fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
    }
}

pub struct NullLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

unsafe impl<T> Send for NullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for NullLock<T> where T: ?Sized + Send {}

impl<T> NullLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

//OS Interface
impl<T> interface::Mutex for NullLock<T> {
    type Data = T;
    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        let data = unsafe{ &mut *self.data.get() };
        f(data)
    }
}