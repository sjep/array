use std::alloc::{alloc, dealloc, Layout};
use std::ops::{Index, IndexMut};

pub struct Array<T>
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

    pub fn iter<'a>(&'a self) -> ArrayIter<'a, T> {
        ArrayIter{
            arr: &self,
            iter: 0usize
        }
    }

    pub fn len(&self) -> usize {
        self.size
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

impl<'a, T> IntoIterator for &'a Array<T>
  where T: Default + Copy {
    type Item = &'a T;
    type IntoIter = ArrayIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct ArrayIter<'a, T>
  where T: Default + Copy {
    arr: &'a Array<T>,
    iter: usize
}

impl<'a, T> Iterator for ArrayIter<'a, T>
  where T: Default + Copy {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter < self.len() {
            true => {
                self.iter += 1;
                Some(&self.arr[self.iter - 1])
            },
            false => None
        }
    }
}

impl<'a, T> ExactSizeIterator for ArrayIter<'a, T>
  where T: Default + Copy {

    fn len(&self) -> usize {
        self.arr.size - self.iter
    }

}
