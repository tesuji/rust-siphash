#![feature(test)]

extern crate test;

#[link(name = "ciphash")]
extern "C" {
    fn siphash24(src: *const u8, src_sz: usize, k0: u64, k1: u64) -> u64;
}

macro_rules! bench_for_size {
    ($f:ident, $size:expr) => {
        #[bench]
        fn $f(b: &mut test::Bencher) {
            use std::iter::repeat;

            let chunk = repeat(b'.').take($size).collect::<Vec<_>>();

            b.bytes = $size;
            b.iter(|| unsafe { siphash24(chunk.as_ptr(), chunk.len(), 0, 0) });
        }
    };
}

bench_for_size!(bench_for_size_00000, 0);
bench_for_size!(bench_for_size_00001, 1);
bench_for_size!(bench_for_size_00002, 2);
bench_for_size!(bench_for_size_00004, 4);
bench_for_size!(bench_for_size_00008, 8);
bench_for_size!(bench_for_size_00016, 16);
bench_for_size!(bench_for_size_00032, 32);
bench_for_size!(bench_for_size_00064, 64);
bench_for_size!(bench_for_size_00128, 128);
bench_for_size!(bench_for_size_00256, 256);
bench_for_size!(bench_for_size_00512, 512);
bench_for_size!(bench_for_size_01024, 1024);
bench_for_size!(bench_for_size_02048, 2048);
bench_for_size!(bench_for_size_04096, 4096);
bench_for_size!(bench_for_size_65536, 65536);
