#![feature(test)]

#[allow(dead_code)]
const KB: usize = 1024;

#[allow(dead_code)]
const MB: usize = KB * KB;

#[allow(dead_code)]
const N: usize = 2 * MB;

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_calc_with_15det_array(b: &mut Bencher) {
        let det = 15;
        let data = vec![0; N];
        b.iter(|| {
            let mut res = false;
            for i in 0..MB {
                res |= data[i] < data[i + det];
            }
            res
        });
    }

    #[bench]
    fn bench_calc_with_16det_array(b: &mut Bencher) {
        let det = 16;
        let data = vec![0; N];
        b.iter(|| {
            let mut res = false;
            for i in 0..MB {
                res |= data[i] < data[i + det];
            }
            res
        });
    }

    #[bench]
    fn bench_calc_with_17det_array(b: &mut Bencher) {
        let det = 17;
        let data = vec![0; N];
        b.iter(|| {
            let mut res = false;
            for i in 0..MB {
                res |= data[i] < data[i + det];
            }
            res
        });
    }
}
