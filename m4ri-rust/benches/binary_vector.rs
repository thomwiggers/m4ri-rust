#![feature(test)]

extern crate m4ri_rust;
extern crate test;

#[cfg(test)]
mod binary_vector {
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
        b.iter(|| v1.as_matrix().transposed());
    }

    #[bench]
    fn vector_as_matrix(b: &mut Bencher) {
        let v1 = BinVector::random(1000);
        b.iter(|| v1.as_matrix())
    }

    #[bench]
    fn dot_product(b: &mut Bencher) {
        let v1 = BinVector::random(1000);
        let v2 = BinVector::random(1000);

        b.iter(|| &v1 * &v2);
    }

    #[bench]
    fn count_ones(b: &mut Bencher) {
        let v1 = BinVector::random(1000);
        b.iter(|| v1.count_ones());
    }

    #[bench]
    fn use_iter_set_bit_for_count(b: &mut Bencher) {
        let v1 = BinVector::random(1000);
        b.iter(|| { v1.iter_set_bits(..).count() })
    }
}
