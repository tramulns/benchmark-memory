#![feature(test)]

#[allow(dead_code)]
const KB: usize = 1024;

#[allow(dead_code)]
const MB: usize = KB * KB;

#[allow(dead_code)]
const N: usize = 16 * MB;

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use rand::prelude::*;
    use test::Bencher;

    #[bench]
    fn bench_index_sum(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut data = vec![0; N];
        for i in 0..N {
            data[i] = rng.gen_range(0..42);
        }
        let next = (0..N).collect::<Vec<_>>();
        let mut sum: u32 = 0;
        b.iter(|| {
            for &i in &next {
                let x = data[i];
                sum = sum.wrapping_add(x);
            }
            sum
        });
    }

    #[bench]
    fn bench_index_sum_unchecked(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut data = vec![0; N];
        for i in 0..N {
            data[i] = rng.gen_range(0..42);
        }
        let next = (0..N).collect::<Vec<_>>();
        let mut sum: u32 = 0;
        b.iter(|| {
            for &i in &next {
                let x = unsafe { *data.get_unchecked(i) };
                sum = sum.wrapping_add(x);
            }
            sum
        });
    }

    #[bench]
    fn bench_range_sum(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut data = vec![0; N];
        for i in 0..N {
            data[i] = rng.gen_range(0..42);
        }
        let mut sum: u32 = 0;
        b.iter(|| {
            for i in 0..N {
                let x = data[i];
                sum = sum.wrapping_add(x);
            }
            sum
        });
    }

    #[bench]
    fn bench_range_sum_unchecked(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut data = vec![0; N];
        for i in 0..N {
            data[i] = rng.gen_range(0..42);
        }
        let mut sum: u32 = 0;
        b.iter(|| {
            for i in 0..N {
                let x = unsafe { *data.get_unchecked(i) };
                sum = sum.wrapping_add(x);
            }
            sum
        });
    }

    #[bench]
    fn bench_iterator_sum(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut data = vec![0; N];
        for i in 0..N {
            data[i] = rng.gen_range(0..42);
        }
        let mut sum: u32 = 0;
        b.iter(|| {
            for &x in data[0..N].iter() {
                sum = sum.wrapping_add(x);
            }
            sum
        });
    }
}
