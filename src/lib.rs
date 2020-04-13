//! arr is a simple array crate designed to allow huge array allocation without stack overflows.
//!
//! Even in rustc 1.44 allocating arrays too large to fit on the stack (say 8MB in size) will
//! stack overflow even if the array is Boxed.
//!
//! Basic usage:
//! ```
//! use arr::Array;
//!
//! let big_array: Array<u8> = Array::new(1 << 25);
//! ```
//!
//! Try to do this with a traditional array:
//! ```
//! let big_array: Box<[u8; 1 << 25]> = Box::new([0u8; 1 << 25]);
//! ```
//!
//! Currently the array supports two modes of allocation, via new (requires types to have Default +
//! Copy) and new_from_template, only requiring clone. As long as this type fits on the stack and
//! the array fits in memory this should be allocatable.
//!
use std::alloc::{alloc, dealloc, Layout};
use std::ops::{Index, IndexMut};

pub struct Array<T> {
    size: usize,
    ptr: *mut T
}

impl<T> Array<T> {
    /// Create an immutable iterator over elements in Array.
    pub fn iter<'a>(&'a self) -> ArrayIter<'a, T> {
        ArrayIter{
            arr: &self,
            iter: 0usize
        }
    }

    /// The length of the array (number of elements T)
    pub fn len(&self) -> usize {
        self.size
    }
}

impl<T> Array<T>
  where T: Default + Copy {
    /// Easy initialization if all you want is your T's default instantiation
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

impl<T> Array<T>
  where T: Clone {
    /// More generic initialization instantiating all elements as copies of some template
    pub fn new_from_template(size: usize, template: &T) -> Self {
        let objsize = std::mem::size_of::<T>();
        let layout = Layout::from_size_align(size * objsize, 8).unwrap();
        let ptr = unsafe {
            alloc(layout) as *mut T
        };
        for i in 0..size {
            unsafe {
                (*(ptr.wrapping_offset(i as isize))) = template.clone();
            }
        }
        Self{size, ptr}
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {

        unsafe {
            self.ptr.wrapping_offset(idx as isize).as_ref()
        }.unwrap()
    }
}

impl<T> IndexMut<usize> for Array<T> {

    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {

        unsafe {
            self.ptr.wrapping_offset(idx as isize).as_mut()
        }.unwrap()
    }

}

impl<T> Drop for Array<T> {

    fn drop(&mut self) {
        let objsize = std::mem::size_of::<T>();
        let layout = Layout::from_size_align(self.size * objsize, 8).unwrap();
        unsafe {
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl<'a, T> IntoIterator for &'a Array<T> {
    type Item = &'a T;
    type IntoIter = ArrayIter<'a, T>;
    /// For now, you can only for loop iterate directly over the
    /// reference:
    /// ```
    /// for i in &arr {
    ///     println!("{}", i);
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct ArrayIter<'a, T> {
    arr: &'a Array<T>,
    iter: usize
}

impl<'a, T> Iterator for ArrayIter<'a, T> {
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

impl<'a, T> ExactSizeIterator for ArrayIter<'a, T> {

    fn len(&self) -> usize {
        self.arr.size - self.iter
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        let arr: Array<f32> = Array::new(4 << 20); // Uses 16 MB - much to large for a stack
        assert_eq!(arr[4 << 20 - 1], 0.0);
    }

    #[test]
    fn test_template() {
        // template: 256 * 8 = 2^11
        let template: [u64; 256] = [65u64; 256];
        // num: 4096 => total size: 2^23, too large for the stack
        let arr: Array<[u64; 256]> = Array::new_from_template(4096, &template);
        assert_eq!(arr[4095][255], 65);
    }

    struct Unaligned {
        a: u64,
        b: u16
    }

    #[test]
    fn test_unaligned() {
        let unaligned_template = Unaligned{a: 15, b:32};
        let arr: Array<Unaligned> = Array::new_from_template(5, &unaligned_template);
        assert_eq!(arr[3].a, 15);
        assert_eq!(arr[3].b, 32);
        assert_eq!(arr[4].a, 15);
        assert_eq!(arr[4].b, 32);
    }
}
