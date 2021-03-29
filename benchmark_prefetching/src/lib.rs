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
    use rand::prelude::*;
    use test::Bencher;

    fn is_nice(x: i32) -> bool {
        x % 13 == 0
            || x % 17 == 0
            || x % 19 == 0
            || x % 43 == 0
            || x % 71 == 0
            || x % 101 == 0
            || x % 233 == 0
            || x % 383 == 0
            || x % 479 == 0
            || x % 541 == 0
    }

    #[bench]
    fn bench_simple(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut data = vec![0; N];
        for i in 0..N {
            data[i] = rng.gen_range(0..42);
        }
        let mut next = vec![0; N];
        for i in 0..N {
            next[i] = rng.gen_range(0..N);
        }

        b.iter(|| {
            let mut result = 0;
            for i in 0..2000 {
                let index = next[i];
                let x = data[index];

                result += is_nice(x) as i32;
            }
            result
        });
    }

    #[bench]
    fn bench_prefetch(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut data = vec![0; N];
        for i in 0..N {
            data[i] = rng.gen_range(0..42);
        }
        let mut next = vec![0; N];
        for i in 0..N {
            next[i] = rng.gen_range(0..N);
        }

        b.iter(|| {
            let mut result = 0;
            let mut index = next[0];
            let mut y = data[index];
            for _ in 0..2000 {
                let x = y;
                index = next[index];
                y = data[index];

                result += is_nice(x) as i32;
            }
            result
        });
    }
}
