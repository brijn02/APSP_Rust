// File with all the similar function for the FWB parallel and sequential algorithm
// Because of similarity this is done in a separated file
// 30-01-2024

// Import crates
use num::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::fmt::Debug;

macro_rules! floyd_warshall_base {
    ($left: expr, $right: expr, $out: expr, $b: expr) => {
        // let b = $b;
        let k_range = $b.min($left[0].len()).min($right.len());
        let i_range = $b.min($left.len()).min($out.len());
        let j_range = $b.min($right[0].len()).min($out[0].len());

        for k in 0..k_range {
            for i in 0..i_range {
                for j in 0..j_range {
                    if let (Some(aik), Some(bkj)) = ($left[i][k], $right[k][j]) {
                        let sum = aik + bkj;
                        if let Some(cij) = $out[i][j] {
                            if cij > sum {
                                $out[i][j] = Some(sum);
                            }
                        } else {
                            $out[i][j] = Some(sum);
                        }
                    }
                }
            }
        }
        return $out;
    };
}

// FW for one block where all matrices are the same (original FW algorithm)
pub fn floyd_warshall_in_place1<W: Num + Copy + PartialOrd + Debug + Clone>(
    matrix_c: &Vec<Vec<Option<W>>>,
    b: usize,
) -> Vec<Vec<Option<W>>> {
    let mut result = matrix_c.to_owned();

    floyd_warshall_base!(result, result, result, b);
}

// FW in place where the output matrix is the same as the B matrix and where matrix A differs
pub fn floyd_warshall_in_place2<W: Num + Copy + PartialOrd + Debug + Clone>(
    matrix_c: &Vec<Vec<Option<W>>>,
    matrix_a: &[Vec<Option<W>>],
    b: usize,
) -> Vec<Vec<Option<W>>> {
    let mut result = matrix_c.to_owned();
    floyd_warshall_base!(matrix_a, result, result, b);
}

// FW in place where the output matrix is the same as the A matrix and where matrix B differs
pub fn floyd_warshall_in_place3<W: Num + Copy + PartialOrd + Debug + Clone>(
    matrix_c: &Vec<Vec<Option<W>>>,
    matrix_b: &Vec<Vec<Option<W>>>,
    b: usize,
) -> Vec<Vec<Option<W>>> {
    let mut result = matrix_c.to_owned();

    floyd_warshall_base!(result, matrix_b, result, b);
}

// FW in place where the output matrix, and matrix A and B are different.
pub fn floyd_warshall_in_place4<W: Num + Copy + PartialOrd + Debug + Clone>(
    matrix_c: &Vec<Vec<Option<W>>>,
    matrix_a: &[Vec<Option<W>>],
    matrix_b: &[&[Option<W>]],
    b: usize,
) -> Vec<Vec<Option<W>>> {
    let mut result = matrix_c.to_owned();

    floyd_warshall_base!(matrix_a, matrix_b, result, b);
}

pub fn slice_matrix_immut<W: Clone>(
    data: &[Vec<Option<W>>],
    block_row: usize,
    block_col: usize,
    block_size: usize,
) -> Vec<&[Option<W>]> {
    let row_start = block_row * block_size;
    let row_end = (block_row + 1) * block_size;
    let row_end = row_end.min(data.len());
    let col_start = block_col * block_size;
    let col_end = (block_col + 1) * block_size;
    let col_end = col_end.min(data[0].len());

    data[row_start..row_end]
        .iter()
        .map(|row| &row[col_start..col_end])
        .collect()
}
// Function to slice the matrix into a matrix [i0..istop][j0..jstop]
pub fn slice_matrix_block<W: Clone>(
    data: &[Vec<Option<W>>],
    block_row: usize,
    block_col: usize,
    block_size: usize,
) -> Vec<Vec<Option<W>>> {
    let row_start = block_row * block_size;
    let row_end = (block_row + 1) * block_size;
    let col_start = block_col * block_size;
    let col_end = (block_col + 1) * block_size;

    slice_matrix(data, row_start, row_end, col_start, col_end)
}

pub fn slice_matrix<W: Clone>(
    data: &[Vec<Option<W>>],
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
) -> Vec<Vec<Option<W>>> {
    let row_end = row_end.min(data.len());
    let col_end = col_end.min(data[0].len());
    let sliced_matrix: Vec<Vec<Option<W>>> = data[row_start..row_end]
        .iter()
        .map(|row| row[col_start..col_end].to_vec())
        .collect();
    sliced_matrix
}

// Function to copy the matrix back to the original matrix
pub fn write_back_to_distance<W: Copy>(
    distance: &mut [Vec<Option<W>>],
    block_matrix: &[Vec<Option<W>>],
    row_start: usize,
    col_start: usize,
) {
    let row_end = row_start + block_matrix.len();
    let col_end = col_start + block_matrix[0].len();
    for i in row_start..row_end {
        for j in col_start..col_end {
            distance[i][j] = block_matrix[i - row_start][j - col_start];
        }
    }
}
