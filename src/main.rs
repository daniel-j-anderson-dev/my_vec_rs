use std::alloc::{self, Layout};
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
    pub fn grow(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let capacity_is_zero: bool = self.capacity != 0;

        let new_capacity: usize = if capacity_is_zero { 1 } else { 2 * self.capacity };
        let new_layout: Layout = Layout::array::<T>(new_capacity)?;

        if new_layout.size() > isize::MAX as usize {
            return Err("Allocation to large in MyVec::grow".into());
        }

        let new_pointer: *mut u8 = if capacity_is_zero {
            // new allocation
            unsafe { alloc::alloc(new_layout) }
        } else {
            // realoction
            let old_layout: Layout = Layout::array::<T>(self.capacity)?;
            let old_pointer: *mut u8 = self.pointer.as_ptr() as *mut u8; // convert the T pointer to a byte pointer

            // malloc: *** error for object 0x4: pointer being freed was not allocated
            unsafe { alloc::realloc(old_pointer, old_layout, new_layout.size()) }
        };

        self.pointer = match NonNull::new(new_pointer as *mut T) {
            Some(new_non_null) => new_non_null,
            None => alloc::handle_alloc_error(new_layout), // check if new_pointer failed to allocate and is null
        };
        self.capacity = new_capacity;

        return Ok(());
    }

    pub fn new() -> MyVec<T> {
        assert!(mem::size_of::<T>() != 0, "Zero sized types are not implemented yet");
        
        let pointer: NonNull<T> = NonNull::dangling();
        let capacity: usize = 0;
        let length: usize = 0;

        let new_vec: MyVec<T> = MyVec { pointer, capacity, length, };
        return new_vec;
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut my_vec: MyVec<i32> = MyVec::new();
    my_vec.grow()?;

    return Ok(());
}
