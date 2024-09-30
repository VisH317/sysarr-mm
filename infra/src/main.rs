use infra::{sysarr::SysArr, sysarr_utils::stagger_vertical};
use infra::matrix::Matrix;
use half::f16;
use infra::config::{HEIGHT, WIDTH};

fn main() {
    let mut sysarr = SysArr::new(WIDTH, HEIGHT);

    let mut mat1: Matrix<f16> = Matrix::new(2, 3, vec![f16::from_f32(0.5), f16::from_f32(0.25), f16::from_f32(1.5), f16::from_f32(2.3), f16::from_f32(4.2), f16::from_f32(0.42)]);
    let mut mat2: Matrix<f16> = Matrix::new(3, 2, vec![f16::from_f32(0.5), f16::from_f32(0.25), f16::from_f32(1.5), f16::from_f32(2.3), f16::from_f32(4.2), f16::from_f32(0.42)]);
    let mut mat3: Matrix<f16> = Matrix::new(2, 2, vec![f16::from_f32(0.0), f16::from_f32(0.0), f16::from_f32(0.0), f16::from_f32(0.0)]);

    // let mat_stag = stagger_vertical(&mut mat2);

    // mat_stag.print();

    // mat1.set_matrix_slice(&mat3, ((1, 0), (3, 2)), ((0, 0), (2, 2)));
    // mat1.print();

    // println!();

    // mat2.print();

    sysarr.load(mat1, mat2);

    sysarr.run();

    sysarr.print_arr();

    // println!("\nThe actual result:");

    // let res = sysarr.get_output();

    // match res {
    //     Ok(mat) => { print_matrix(&mat) },
    //     Err(e) => { println!("Err: {}", e) }
    // }
}
