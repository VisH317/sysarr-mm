use half::f16;
use crate::matrix::Matrix;
// sysarr utils

pub fn stagger_horizontal<'a>(mat: &mut Matrix<f16>) -> Matrix<'a, f16> {
    let width = mat.get_width();
    let height = mat.get_height();
    let staggered_width: usize = width + height - 1;

    let mut mat_stag = Matrix::<f16>::zeros(staggered_width, height);

    for (i, row) in mat.rows().iter_mut().enumerate() {
        let start_width = staggered_width - width - i;
        let mut combined_row = vec![f16::from_f32(0.0); start_width];
        combined_row.append(row);
        
        let end_row = &mut vec![f16::from_f32(0.0); i];
        combined_row.append(end_row);

        for (j, x) in combined_row.iter().enumerate() {
            mat_stag[i][j] = combined_row[j];
        }
    }

    return mat_stag;
}

pub fn stagger_vertical<'a>(mat: &mut Matrix<f16>) -> Matrix<'a, f16> {
    let width = mat.get_width();
    let height = mat.get_height();
    let staggered_height: usize = width + height - 1;

    let mut mat_stag = Matrix::<f16>::zeros(width, staggered_height);

    for (i, row) in (*mat).rows().iter_mut().enumerate() {
        let row_len: usize = row.len();

        for (j, value) in row.iter_mut().enumerate() {
            let row = mat.get_height() + i - j;

            mat_stag[row][j] = *value;
        }
    }

    return mat_stag;
}
