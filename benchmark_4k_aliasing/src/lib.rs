#![feature(test)]

#[allow(dead_code)]
const KB: usize = 1024;

#[allow(dead_code)]
const MB: usize = KB * KB;

#[allow(dead_code)]
const N: usize = 32 * MB;

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use std::ptr;
    use test::Bencher;

    #[bench]
    fn bench_array_copy_with_stride_offset_neg33(b: &mut Bencher) {
        let mut src = vec![0; N];
        let align = 4 * KB;
        let length = 16 * KB;
        let stride_offset = (24 * KB as i64 - 33) as usize;
        let scr_raw_ptr = src.as_mut_ptr();
        let source_index;
        unsafe {
            source_index = scr_raw_ptr.add(align - (scr_raw_ptr as usize % align));
        }
        let destination_index;
        unsafe {
            destination_index =
                scr_raw_ptr.add(align - (scr_raw_ptr as usize % align) + stride_offset);
        }
        b.iter(|| unsafe {
            ptr::copy(source_index, destination_index, length);
        });
    }

    #[bench]
    fn bench_array_copy_with_stride_offset_neg1(b: &mut Bencher) {
        let mut src = vec![0; N];
        let align = 4 * KB;
        let length = 16 * KB;
        let stride_offset = (24 * KB as i64 - 1) as usize;
        let scr_raw_ptr = src.as_mut_ptr();
        let source_index;
        unsafe {
            source_index = scr_raw_ptr.add(align - (scr_raw_ptr as usize % align));
        }
        let destination_index;
        unsafe {
            destination_index =
                scr_raw_ptr.add(align - (scr_raw_ptr as usize % align) + stride_offset);
        }
        b.iter(|| unsafe {
            ptr::copy(source_index, destination_index, length);
        });
    }

    #[bench]
    fn bench_array_copy_with_stride_offset_0(b: &mut Bencher) {
        let mut src = vec![0; N];
        let align = 4 * KB;
        let length = 16 * KB;
        let stride_offset = 24 * KB;
        let scr_raw_ptr = src.as_mut_ptr();
        let source_index;
        unsafe {
            source_index = scr_raw_ptr.add(align - (scr_raw_ptr as usize % align));
        }
        let destination_index;
        unsafe {
            destination_index =
                scr_raw_ptr.add(align - (scr_raw_ptr as usize % align) + stride_offset);
        }
        b.iter(|| unsafe {
            ptr::copy(source_index, destination_index, length);
        });
    }

    #[bench]
    fn bench_array_copy_with_stride_offset_1(b: &mut Bencher) {
        let mut src = vec![0; N];
        let align = 4 * KB;
        let length = 16 * KB;
        let stride_offset = 24 * KB + 1;
        let scr_raw_ptr = src.as_mut_ptr();
        let source_index;
        unsafe {
            source_index = scr_raw_ptr.add(align - (scr_raw_ptr as usize % align));
        }
        let destination_index;
        unsafe {
            destination_index =
                scr_raw_ptr.add(align - (scr_raw_ptr as usize % align) + stride_offset);
        }
        b.iter(|| unsafe {
            ptr::copy(source_index, destination_index, length);
        });
    }

    #[bench]
    fn bench_array_copy_with_stride_offset_31(b: &mut Bencher) {
        let mut src = vec![0; N];
        let align = 4 * KB;
        let length = 16 * KB;
        let stride_offset = 24 * KB + 31;
        let scr_raw_ptr = src.as_mut_ptr();
        let source_index;
        unsafe {
            source_index = scr_raw_ptr.add(align - (scr_raw_ptr as usize % align));
        }
        let destination_index;
        unsafe {
            destination_index =
                scr_raw_ptr.add(align - (scr_raw_ptr as usize % align) + stride_offset);
        }
        b.iter(|| unsafe {
            ptr::copy(source_index, destination_index, length);
        });
    }
}
