use std::{
    mem,
    alloc::{ self, Layout },
    ptr::{ self, NonNull },
};

#[derive(Debug)]
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

    fn grow(&mut self) {
        let capacity_is_zero: bool = self.capacity == 0;

        let new_capacity: usize = if capacity_is_zero { 1 } else { 2 * self.capacity };
        let new_layout: Layout = Layout::array::<T>(new_capacity).unwrap();

        assert!(new_layout.size() < isize::MAX as usize, "Allocation to large in MyVec::grow");

        let new_pointer: *mut u8 = if capacity_is_zero {
            // new allocation
            unsafe {
                alloc::alloc(new_layout)
            }
        } else {
            // realoction
            let old_layout: Layout = Layout::array::<T>(self.capacity).unwrap();
            let old_pointer: *mut u8 = self.pointer.as_ptr() as *mut u8; // convert the T pointer to a byte pointer

            unsafe { 
                alloc::realloc(old_pointer, old_layout, new_layout.size())
            }
        };

        self.pointer = match NonNull::new(new_pointer as *mut T) {
            Some(new_non_null) => new_non_null,
            None => alloc::handle_alloc_error(new_layout), // check if new_pointer failed to allocate and is null
        };

        self.capacity = new_capacity;
    }

    pub fn push(&mut self, element: T) {
        // if full grow
        if self.capacity == self.length {
            self.grow();
        }

        // write the element to the first empty slot
        unsafe {
            // the first empty slot is the self.length after self.pointer
            let first_free_slot: *mut T = self.pointer.as_ptr().add(self.length);
            ptr::write(first_free_slot, element);
        }

        // account for the new element
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            // if self is empty there is nothing to pop
            return None;
        } else {
            // account for the pop
            self.length -= 1;
            unsafe {
                return Some(ptr::read(self.pointer.as_ptr().add(self.length)));
            }
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut v: MyVec<i32> = MyVec::new();

    for i in 0..50 {
        v.push(i);
    }    

    for _ in 0..=v.length {
        match v.pop() {
            Some(element) => print!("{element}, "),
            None => println!("End"),
        }
    }

    return Ok(());
}
