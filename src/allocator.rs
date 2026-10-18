pub(crate) use self::inner::{vec, Allocator, Global, Vec};

#[cfg(feature = "nightly")]
mod inner {
    pub(crate) use alloc::{
        alloc::{Allocator, Global},
        vec,
        vec::Vec,
    };
}

#[cfg(all(not(feature = "nightly"), feature = "allocator-api2"))]
mod inner {
    pub(crate) use allocator_api2::{
        alloc::{Allocator, Global},
        vec,
        vec::Vec,
    };
}

#[cfg(not(any(feature = "nightly", feature = "allocator-api2")))]
mod inner {
    use alloc::{
        alloc::{alloc, dealloc, Layout},
        vec,
        vec::Vec,
    };
    use core::ptr::NonNull;

    #[expect(clippy::missing_safety_doc)] // not exposed outside of this crate
    pub unsafe trait Allocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, ()>;
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
    }

    #[derive(Copy, Clone)]
    pub struct Global;

    unsafe impl Allocator for Global {
        #[inline]
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, ()> {
            match unsafe { NonNull::new(alloc(layout)) } {
                Some(data) => {
                    // SAFETY: this is NonNull::slice_from_raw_parts.
                    Ok(unsafe {
                        NonNull::new_unchecked(core::ptr::slice_from_raw_parts_mut(
                            data.as_ptr(),
                            layout.size(),
                        ))
                    })
                }
                None => Err(()),
            }
        }
        #[inline]
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            unsafe {
                dealloc(ptr.as_ptr(), layout);
            }
        }
    }

    impl Default for Global {
        #[inline]
        fn default() -> Self {
            Global
        }
    }
}
