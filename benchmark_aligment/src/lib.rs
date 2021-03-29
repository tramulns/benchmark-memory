#![feature(test)]

#[allow(dead_code)]
const N: usize = 256;

#[allow(dead_code)]
#[derive(Default, Copy, Clone)]
struct Struct3 {
    x1: u8,
    x2: u8,
    x3: u8,
}

#[allow(dead_code)]
#[derive(Default, Copy, Clone)]
struct Struct8 {
    x1: u8,
    x2: u8,
    x3: u8,
    x4: u8,
    x5: u8,
    x6: u8,
    x7: u8,
    x8: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_s3_size() {
        assert_eq!(3, std::mem::size_of::<Struct3>());
    }

    #[test]
    fn test_s8_size() {
        assert_eq!(8, std::mem::size_of::<Struct8>());
    }

    #[bench]
    fn bench_calc_s3(b: &mut Bencher) {
        let data = [Struct3::default(); N];
        b.iter(|| {
            let mut result = 0;
            for item in data.iter() {
                result += item.x1 + item.x2;
            }
            result
        });
    }

    #[bench]
    fn bench_calc_s8(b: &mut Bencher) {
        let data = [Struct8::default(); N];
        b.iter(|| {
            let mut result = 0;
            for item in data.iter() {
                result += item.x1 + item.x2;
            }
            result
        });
    }
}
