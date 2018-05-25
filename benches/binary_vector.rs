#![feature(test)]

extern crate m4ri_rust;
extern crate test;

#[cfg(test)]
mod benchmarks {
    use m4ri_rust::friendly::BinVector;
    use test::Bencher;

    #[bench]
    fn vector_as_column_matrix(b: &mut Bencher) {
        let v1 = BinVector::random(1000);
        b.iter(|| v1.as_column_matrix())
    }

    #[bench]
    fn vector_as_matrix_transposed(b: &mut Bencher) {
        let v1 = BinVector::random(1000);
        b.iter(|| v1.as_matrix().transpose());
    }

    #[bench]
    fn vector_as_matrix(b: &mut Bencher) {
        let v1 = BinVector::random(1000);
        b.iter(|| v1.as_matrix())
    }
}
