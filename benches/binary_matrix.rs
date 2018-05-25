#![feature(test)]

extern crate m4ri_rust;
extern crate test;

#[cfg(test)]
mod benchmarks {
    use m4ri_rust::friendly::BinMatrix;
    use test::Bencher;

    #[bench]
    fn as_vector_column(b: &mut Bencher) {
        let m1 = BinMatrix::random(1000, 1);
        b.iter(|| m1.as_vector())
    }

    #[bench]
    fn as_vector_column_transpose(b: &mut Bencher) {
        let m1 = BinMatrix::random(1000, 1);
        b.iter(|| m1.transpose().as_vector())
    }

    #[bench]
    fn as_vector_row(b: &mut Bencher) {
        let m1 = BinMatrix::random(1, 1000);
        b.iter(|| m1.as_vector())
    }

}
