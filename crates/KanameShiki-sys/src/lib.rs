#![no_std]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use core::ptr::null_mut;

    use super::*;

    #[test]
    fn alloc() {
        let buf = unsafe { KanameShiki_Alloc(32) };
        assert_ne!(buf, null_mut());
    }

    #[test]
    fn free() {
        let buf = unsafe { KanameShiki_Alloc(32) };
        assert_ne!(buf, null_mut());

        unsafe {
            KanameShiki_Free(buf);
        }
    }

    #[test]
    fn aligned() {
        let buf = unsafe { KanameShiki_Align(4, 128) };
        assert_ne!(buf, null_mut());
    }

    #[test]
    fn realloc() {
        let buf = unsafe { KanameShiki_Alloc(32) };
        assert_ne!(buf, null_mut());

        let new_buf = unsafe { KanameShiki_ReAlloc(buf, 64) };
        assert_ne!(new_buf, null_mut());
    }

    #[test]
    fn realloc_free() {
        let buf = unsafe { KanameShiki_Alloc(32) };
        assert_ne!(buf, null_mut());

        let new_buf = unsafe { KanameShiki_ReAlloc(buf, 128) };
        assert_ne!(new_buf, null_mut());

        unsafe {
            KanameShiki_Free(new_buf);
        }
    }
}
