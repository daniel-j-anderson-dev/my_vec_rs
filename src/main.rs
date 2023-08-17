use std::ptr::NonNull;
use std::mem;

pub struct MyVec<T> {
    pointer: NonNull<T>,
    capacity: usize, // number of elements that can be held before allocating more memory
    length: usize, // number of elements currently held
}

unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    pub fn new() -> MyVec<T> {
        assert!(mem::size_of::<T>() != 0, "Zero sized types are not implemented yet");
        
        let pointer: NonNull<T> = NonNull::dangling();
        let capacity: usize = 0;
        let length: usize = 0;
        
        let new_vec: MyVec<T> = MyVec { pointer, capacity, length, };
        return new_vec;
    }
}


fn main() {
    let my_vec: MyVec<i32> = MyVec::<i32>new();
}
