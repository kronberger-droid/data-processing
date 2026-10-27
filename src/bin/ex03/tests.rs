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
