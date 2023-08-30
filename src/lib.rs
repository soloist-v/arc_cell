use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::Arc;

#[repr(transparent)]
pub struct ArcCell<T: ?Sized> {
    data: Arc<UnsafeCell<T>>,
}

unsafe impl<T: ?Sized + Sync + Send> Send for ArcCell<T> {}

unsafe impl<T: ?Sized + Sync + Send> Sync for ArcCell<T> {}

impl<T> Deref for ArcCell<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UnsafeCell::get(&self.data) }
    }
}

impl<T> Clone for ArcCell<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T> ArcCell<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            data: Arc::new(UnsafeCell::new(value)),
        }
    }
    #[inline(always)]
    pub unsafe fn get_mut_uncheck(arc_cell: &Self) -> &mut T {
        &mut *UnsafeCell::get(&arc_cell.data)
    }
}

#[test]
fn test() {
    let a = ArcCell::new(10);
    let ref_mut_a = unsafe { ArcCell::get_mut_uncheck(&a) };
    *ref_mut_a = 11;
    assert_eq!(*a, 11);
}