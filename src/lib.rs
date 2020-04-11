use std::alloc::{alloc, dealloc, Layout};
use std::ops::{Index, IndexMut};

struct Array<T>
  where T: Default + Copy {
    size: usize,
    ptr: *mut T
}

impl<T> Array<T>
  where T: Default + Copy {

    pub fn new(size: usize) -> Self {
        let objsize = std::mem::size_of::<T>();
        let layout = Layout::from_size_align(size * objsize, 8).unwrap();
        let ptr = unsafe {
            alloc(layout) as *mut T
        };
        let default: T = Default::default();
        for i in 0..size {
            unsafe {
                (*(ptr.wrapping_offset(i as isize))) = default;
            }
        }
        Self{size, ptr}
    }
}

impl<T> Index<usize> for Array<T>
  where T: Default + Copy {
    type Output = T;

    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {

        unsafe {
            self.ptr.wrapping_offset(idx as isize).as_ref()
        }.unwrap()
    }
}

impl<T> IndexMut<usize> for Array<T>
  where T: Default + Copy {

    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {

        unsafe {
            self.ptr.wrapping_offset(idx as isize).as_mut()
        }.unwrap()
    }

}

impl<T> Drop for Array<T>
  where T: Default + Copy {

    fn drop(&mut self) {
        let objsize = std::mem::size_of::<T>();
        let layout = Layout::from_size_align(self.size * objsize, 8).unwrap();
        unsafe {
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}
