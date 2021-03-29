#![feature(test)]

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_max_511_2d_array(b: &mut Bencher) {
        const N: usize = 511;
        let data = vec![0; N * N];
        b.iter(|| {
            let mut max = 0;
            for i in 0..N {
                max = max.max(data[i * N + 0]);
            }
        });
    }

    #[bench]
    fn bench_max_512_2d_array(b: &mut Bencher) {
        const N: usize = 512;
        let data = vec![0; N * N];
        b.iter(|| {
            let mut max = 0;
            for i in 0..N {
                max = max.max(data[i * N + 0]);
            }
        });
    }

    #[bench]
    fn bench_max_513_2d_array(b: &mut Bencher) {
        const N: usize = 513;
        let data = vec![0; N * N];
        b.iter(|| {
            let mut max = 0;
            for i in 0..N {
                max = max.max(data[i * N + 0]);
            }
        });
    }
}
