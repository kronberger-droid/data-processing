use super::*;

fn mat(rows: usize, cols: usize, data: &[f64]) -> Matrix {
    assert_eq!(data.len(), rows * cols, "test matrix has the wrong length");
    Matrix {
        rows,
        cols,
        data: data.to_vec(),
    }
}

fn identity(n: usize) -> Matrix {
    let mut m = mat(n, n, &vec![0.0; n * n]);
    for i in 0..n {
        m[(i, i)] = 1.0;
    }
    m
}

fn assert_mat(m: &Matrix, rows: usize, cols: usize, data: &[f64]) {
    assert_eq!((m.rows, m.cols), (rows, cols), "shape");
    assert_eq!(m.data, data);
}

// Entries are small integers, so every sum is exact in f64 and results can be
// compared with `==` regardless of summation order.

#[test]
fn row_of_non_square() {
    let a = mat(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert_eq!(a.row(0), [1.0, 2.0, 3.0]);
    assert_eq!(a.row(1), [4.0, 5.0, 6.0]);
}

#[test]
fn row_mut_writes_the_right_row() {
    let mut a = mat(2, 3, &[0.0; 6]);
    a.row_mut(1).copy_from_slice(&[7.0, 8.0, 9.0]);
    assert_eq!(a.data, [0.0, 0.0, 0.0, 7.0, 8.0, 9.0]);
}

#[test]
fn square_2x2() {
    let a = mat(2, 2, &[1.0, 2.0, 3.0, 4.0]);
    let b = mat(2, 2, &[5.0, 6.0, 7.0, 8.0]);
    assert_mat(&a.matmul(&b).unwrap(), 2, 2, &[19.0, 22.0, 43.0, 50.0]);
}

#[test]
fn wide_times_tall() {
    let a = mat(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let b = mat(3, 2, &[7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
    assert_mat(&a.matmul(&b).unwrap(), 2, 2, &[58.0, 64.0, 139.0, 154.0]);
}

#[test]
fn tall_times_wide() {
    let a = mat(3, 2, &[7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
    let b = mat(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    #[rustfmt::skip]
    let expected = [
        39.0, 54.0, 69.0,
        49.0, 68.0, 87.0,
        59.0, 82.0, 105.0,
    ];
    assert_mat(&a.matmul(&b).unwrap(), 3, 3, &expected);
}

#[test]
fn row_vector_times_column_vector() {
    let a = mat(1, 3, &[1.0, 2.0, 3.0]);
    let b = mat(3, 1, &[4.0, 5.0, 6.0]);
    assert_mat(&a.matmul(&b).unwrap(), 1, 1, &[32.0]);
}

#[test]
fn identity_on_both_sides() {
    let a = mat(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert_mat(&a.matmul(&identity(3)).unwrap(), 2, 3, &a.data);
    assert_mat(&identity(2).matmul(&a).unwrap(), 2, 3, &a.data);
}

#[test]
fn associative() {
    let a = mat(2, 3, &[1.0, -2.0, 3.0, 0.0, 4.0, -1.0]);
    let b = mat(3, 4, &[2.0, 1.0, 0.0, -3.0, 1.0, 5.0, 2.0, 0.0, -1.0, 2.0, 1.0, 4.0]);
    let c = mat(4, 2, &[1.0, 0.0, 2.0, -1.0, 0.0, 3.0, -2.0, 1.0]);
    let left = a.matmul(&b).unwrap().matmul(&c).unwrap();
    let right = a.matmul(&b.matmul(&c).unwrap()).unwrap();
    assert!(left.data.iter().any(|&x| x != 0.0), "all zeros pass trivially");
    assert_mat(&left, 2, 2, &right.data);
}

#[test]
fn matches_textbook_formula() {
    // c_ij = sum_k a_ik b_kj, written out directly as an independent reference.
    let (n, m, p) = (7, 5, 6);
    let a = mat(n, m, &(0..n * m).map(|x| (x * 7 % 11) as f64 - 5.0).collect::<Vec<_>>());
    let b = mat(m, p, &(0..m * p).map(|x| (x * 3 % 13) as f64 - 6.0).collect::<Vec<_>>());

    let mut expected = vec![0.0; n * p];
    for i in 0..n {
        for j in 0..p {
            expected[i * p + j] = (0..m).map(|k| a.data[i * m + k] * b.data[k * p + j]).sum();
        }
    }

    assert_mat(&a.matmul(&b).unwrap(), n, p, &expected);
}

#[test]
fn mismatched_dimensions_are_an_error() {
    let a = mat(2, 3, &[0.0; 6]);
    let b = mat(2, 3, &[0.0; 6]);
    assert!(a.matmul(&b).is_err());
}

// Serialization: `write_to` and `read_from` move matrices through the worker
// pipes, so a round trip must give back the exact same bits.

/// Bit patterns, so `-0.0` vs `0.0` and the last rounding bit count too.
fn bits(m: &Matrix) -> Vec<u64> {
    m.data.iter().map(|x| x.to_bits()).collect()
}

fn round_trip(m: &Matrix) -> Matrix {
    let mut buf = Vec::new();
    m.write_to(&mut buf).unwrap();
    Matrix::read_from(&mut buf.as_slice()).unwrap()
}

#[test]
fn round_trip_keeps_shape_and_exact_bits() {
    #[rustfmt::skip]
    let a = mat(2, 3, &[
        0.1, -0.0, std::f64::consts::PI,
        f64::MAX, f64::MIN_POSITIVE, -1e-310, // the last one is subnormal
    ]);
    let b = round_trip(&a);
    assert_eq!((b.rows, b.cols), (2, 3), "shape");
    assert_eq!(bits(&b), bits(&a));
}

#[test]
fn round_trip_random_200x200() {
    let a = Matrix::random(200, 200, &mut Lcg::ansi_c(1));
    let b = round_trip(&a);
    assert_eq!((b.rows, b.cols), (200, 200), "shape");
    assert_eq!(bits(&b), bits(&a));
}

#[test]
fn round_trip_empty_matrix() {
    let b = round_trip(&mat(0, 3, &[]));
    assert_eq!((b.rows, b.cols), (0, 3), "shape");
    assert!(b.data.is_empty());
}

#[test]
fn byte_layout() {
    // rows and cols as u64 little endian, then the entries row major as f64
    // little endian. Drop this test if you pick a different format.
    let mut buf = Vec::new();
    mat(1, 2, &[1.5, -2.0]).write_to(&mut buf).unwrap();

    let mut expected = Vec::new();
    expected.extend(1u64.to_le_bytes());
    expected.extend(2u64.to_le_bytes());
    expected.extend(1.5f64.to_le_bytes());
    expected.extend((-2.0f64).to_le_bytes());
    assert_eq!(buf, expected);
}

#[test]
fn several_matrices_in_one_stream() {
    // A worker reads pair after pair from one pipe, so `read_from` must take
    // exactly one matrix and leave the rest of the stream alone.
    let ms = [
        mat(1, 2, &[1.0, 2.0]),
        mat(2, 1, &[3.0, 4.0]),
        mat(2, 2, &[5.0, 6.0, 7.0, 8.0]),
    ];
    let mut buf = Vec::new();
    for m in &ms {
        m.write_to(&mut buf).unwrap();
    }

    let mut reader = buf.as_slice();
    for m in &ms {
        let got = Matrix::read_from(&mut reader).unwrap();
        assert_mat(&got, m.rows, m.cols, &m.data);
    }
    assert!(reader.is_empty(), "{} bytes left over", reader.len());
}

#[test]
fn empty_input_is_an_error() {
    assert!(Matrix::read_from(&mut [].as_slice()).is_err());
}

#[test]
fn truncated_input_is_an_error() {
    let mut buf = Vec::new();
    mat(2, 2, &[1.0, 2.0, 3.0, 4.0]).write_to(&mut buf).unwrap();
    buf.pop();
    assert!(Matrix::read_from(&mut buf.as_slice()).is_err());
}
