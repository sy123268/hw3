use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ptr::NonNull;

struct MyRc<T> {
    data: NonNull<T>,
    count: UnsafeCell<usize>,
}

impl<T> MyRc<T> {
    fn new(data: T) -> MyRc<T> {
        let boxed = Box::new(data);
        let data = NonNull::new(Box::leak(boxed)).expect("Box::leak should never return null");
        let count = UnsafeCell::new(1);
        MyRc { data, count }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}


impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            *self.count.get() -= 1;
            if *self.count.get() == 0 {
                Box::from_raw(self.data.as_ptr());
            }
        }
    }
}