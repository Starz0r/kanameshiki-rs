#![no_std]

use core::alloc::{GlobalAlloc, Layout};

use kanameshiki_sys::{
    KanameShiki_Align, KanameShiki_Alloc, KanameShiki_Free, KanameShiki_ReAlloc,
};

pub struct KanameShiki;

unsafe impl GlobalAlloc for KanameShiki {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() == 1 {
            return KanameShiki_Alloc(layout.size().try_into().unwrap()) as _;
        }
        KanameShiki_Align(
            layout.align().try_into().unwrap(),
            layout.size().try_into().unwrap(),
        ) as _
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        KanameShiki_Free(ptr as _)
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unimplemented!("zeroed allocated blocks are not implemented")
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        KanameShiki_ReAlloc(ptr as _, new_size.try_into().unwrap()) as _
    }
}

#[cfg(test)]
mod tests {
    use core::{i128, mem::forget};

    use alloc::{borrow::Cow, boxed::Box, rc::Rc, string::String, sync::Arc, vec::Vec};

    extern crate alloc;

    use super::*;

    #[global_allocator]
    static GLOBAL: KanameShiki = KanameShiki;

    #[test]
    fn vec_alloc() {
        let mut v: Vec<u8> = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(5);
    }

    #[test]
    fn rc_alloc() {
        let mut rc: Rc<i128> = Rc::new(12345678910);
    }

    #[test]
    fn arc_alloc() {
        let mut arc: Arc<i128> = Arc::new(12345678910);
    }

    #[test]
    fn string_alloc() {
        String::from("The KanameShiki Memory Allocator is a fast, general purpose allocator with a lot of additional features.");
    }

    #[test]
    fn box_alloc() {
        let bbox: Box<i128> = Box::new(12345678910);
    }

    /*#[test]
    fn hashtable_alloc() {
    let dict: Has<u128, u128> = HashMap::new();
    dict.insert(4124651, -9187517915);
    }*/

    #[test]
    fn cow_alloc() {
        let mut cow = Cow::Borrowed("borrowed cow using KanameShiki");
        cow += "z";
    }
}
