#![feature(test)]

extern crate m4ri_rust;
extern crate test;

#[cfg(test)]
mod binary_matrix {
    use m4ri_rust::friendly::*;
    use test::Bencher;

    #[bench]
    fn as_vector_column(b: &mut Bencher) {
        let m1 = BinMatrix::random(1000, 1);
        b.iter(|| m1.as_vector())
    }

    #[bench]
    fn as_vector_column_transpose(b: &mut Bencher) {
        let m1 = BinMatrix::random(1000, 1);
        b.iter(|| m1.transposed().as_vector())
    }

    #[bench]
    fn as_vector_row(b: &mut Bencher) {
        let m1 = BinMatrix::random(1, 1000);
        b.iter(|| m1.as_vector());
    }

    #[bench]
    fn vector_matrix(b: &mut Bencher) {
        let v = BinVector::random(1000);
        let m = BinMatrix::random(1000, 64);
        b.iter(|| &v * &m);
    }

    #[bench]
    fn matrix_vector(b: &mut Bencher) {
        let v = BinVector::random(1000);
        let m = BinMatrix::random(64, 1000);
        b.iter(|| &m * &v);
    }

    macro_rules! multiply {
        ($id:ident, $a:tt, $b:tt, $d:tt) => {
            #[bench]
            fn $id(b: &mut Bencher) {
                let m1 = BinMatrix::random($a, $b);
                let m2 = BinMatrix::random($b, $d);
                b.iter(|| &m1 * &m2);
            }
        };
    }

    multiply!(matrix_multiply_10x10_10x10, 10, 10, 10);
    multiply!(matrix_multiply_100x10_10x10, 100, 10, 10);
    multiply!(matrix_multiply_100x10_10x100, 100, 10, 100);
    multiply!(matrix_multiply_1000x64_64x1000, 1000, 64, 1000);
    multiply!(matrix_multiply_1000x10_10x1000, 1000, 10, 1000);
    multiply!(matrix_multiply_1000x64_64x1, 1000, 64, 1);
    multiply!(matrix_multiply_1x64_64x100, 1, 64, 1000);
    multiply!(matrix_multiply_10x1000_1000x10, 10, 1000, 10);

}
