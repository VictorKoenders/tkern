use core::ffi::c_void;
use core::ptr;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let source = [0u8; 1000];
    let mut dest = [0u8; 1000];
    bencher.iter(|| {
        let len = black_box(source.len() as isize);
        let source = black_box(source.as_ptr().cast());
        let dest = black_box(dest.as_mut_ptr().cast());
        unsafe {
            memcpy(dest, source, len);
        }
    })
}
unsafe fn memcpy(dest: *mut c_void, src: *const c_void, size: isize) -> *mut c_void {
    {
        let dest: *mut u8 = dest.cast();
        let src: *const u8 = src.cast();
        for i in 0..size {
            ptr::write(dest.offset(i), ptr::read(src.offset(i)));
        }
    }

    dest
}

#[bench]
fn bench_bulk(bencher: &mut Bencher) {
    let source = [0u8; 1000];
    let mut dest = [0u8; 1000];
    bencher.iter(|| {
        let len = black_box(source.len() as isize);
        let source = black_box(source.as_ptr().cast());
        let dest = black_box(dest.as_mut_ptr().cast());
        unsafe {
            bulk(dest, source, len);
        }
    })
}

unsafe fn bulk(dest: *mut c_void, src: *const c_void, size: isize) -> *mut c_void {
    let mut i = 0;
    macro_rules! impl_type {
        ($ty:ty) => {{
            const SIZE: isize = core::mem::size_of::<$ty>() as isize;
            while i + SIZE < size {
                let dest: *mut $ty = dest.cast();
                let src: *const $ty = src.cast();
                ptr::write(dest.offset(i / SIZE), ptr::read(src.offset(i / SIZE)));
                i += SIZE;
            }
        }};
    }

    // ARMv7 supports up to 64 bits of integers, so use u64 to do bulk copy
    impl_type!(u64);
    // No point in doing u32, as this can only get executed once
    // No point in doing u16, as this can only get executed up to 3 times
    // Go straight to u8 (up to 7 times)
    // This should reduce the amount of branching the CPU has to do
    impl_type!(u8);

    dest
}

#[bench]
fn bench_slice(bencher: &mut Bencher) {
    let source = [0u8; 1000];
    let mut dest = [0u8; 1000];
    bencher.iter(|| {
        let len = black_box(source.len());
        let source = black_box(source.as_ptr().cast());
        let dest = black_box(dest.as_mut_ptr().cast());
        unsafe {
            slice(dest, source, len);
        }
    })
}

unsafe fn slice(dest: *mut c_void, src: *const c_void, size: usize) -> *mut c_void {
    {
        let dest: &mut [u8] = core::slice::from_raw_parts_mut(dest.cast(), size);
        let src: &[u8] = core::slice::from_raw_parts(src.cast(), size);

        for i in 0..size {
            dest[i] = src[i];
        }
    }
    dest
}

#[bench]
fn bench_slice_rev(bencher: &mut Bencher) {
    let source = [0u8; 1000];
    let mut dest = [0u8; 1000];
    bencher.iter(|| {
        let len = black_box(source.len());
        let source = black_box(source.as_ptr().cast());
        let dest = black_box(dest.as_mut_ptr().cast());
        unsafe {
            slice_rev(dest, source, len);
        }
    })
}

unsafe fn slice_rev(dest: *mut c_void, src: *const c_void, size: usize) -> *mut c_void {
    {
        let dest: &mut [u8] = core::slice::from_raw_parts_mut(dest.cast(), size);
        let src: &[u8] = core::slice::from_raw_parts(src.cast(), size);

        for i in (0..size).rev() {
            dest[i] = src[i];
        }
    }
    dest
}
