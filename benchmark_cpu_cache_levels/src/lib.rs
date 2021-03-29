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
    use test::Bencher;

    #[bench]
    fn bench_calc_2048kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 2048];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_4096kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 4096];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_6144kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 6144];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }
}
