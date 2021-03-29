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
    fn bench_calc_1kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 1];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_2kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 2];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_4kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 4];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_8kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 8];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_16kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 16];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_32kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 32];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_64kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 64];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_128kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 128];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_256kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 256];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_512kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 512];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_1024kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 1024];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

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

    #[bench]
    fn bench_calc_8192kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 8192];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_16384kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 16384];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }

    #[bench]
    fn bench_calc_32768kb_array(b: &mut Bencher) {
        let mut data = vec![0_u8; KB * 32768];
        let mask = data.len() - 1;
        b.iter(|| {
            for i in 0..N {
                data[(i * 64) & mask] += 1;
            }
        });
    }
}
