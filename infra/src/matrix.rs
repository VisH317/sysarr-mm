use std::ops::{Index, IndexMut};
// use std::option::{Option, Option::{None, Some}};
use std::marker::PhantomData;

use half::f16;
// matrix defs and helpers

// pub type Matrix<T> = Vec<Vec<T>>;

pub type SlicePos = ((usize, usize), (usize, usize));

// requirements

pub struct Matrix<'a, T> where T: Clone, T: Copy, T: ToString {
    height: usize,
    width: usize,
    arr: Vec<T>,
    phantom: PhantomData<&'a T> // for lifetime :D
}

impl<'a, T> Matrix<'a, T> where T: Clone, T: Copy, T: ToString {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Matrix<'a, T> {
        if width * height != data.len() { panic!("Matrix dimension mismatch") };
        Matrix { width, height, arr: data, phantom: PhantomData }
    }

    pub fn zeros(width: usize, height: usize) -> Matrix<'a, f16> {
        let data = vec![f16::from_f32(0.0); width*height];
        Matrix { width, height, arr: data, phantom: PhantomData }
    }

    // fn new_sliced(width: usize, height: usize, data: Vec<T, inv: bool) -> Matrix<'a, T> {
    //     if width * height != data.len() { panic!("Sliced matrix dimension mismatched") }

    //     Matrix { width, height, arr: data, transpose_index: inv }
    // }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn rows(&self) -> Vec<Vec<T>> {
        let mut ret: Vec<Vec<T>> = Vec::new();

        for i in 0..self.height {
            ret.push(self[i].to_vec());
        };

        ret
    }

    pub fn set_all(&mut self, value: T) -> () {
        for x in &mut(self.arr) {
            *x = value;
        }
    }

    pub fn set_matrix(&mut self, value: &Matrix<T>) -> () {
        for i in 0..value.get_height() {
            for j in 0..value.get_width() {
                self[i][j] = value[i][j];
            }
        }
    }

    pub fn set_matrix_slice(&mut self, value: &Matrix<T>, self_dims: SlicePos, value_dims: SlicePos) {
        let (in_start, in_end) = self_dims;
        let (out_start, out_end) = value_dims;

        // println!("testing: ({}, {}), ({}, {})", in_start.1, in_end.1, out_start.1, out_end.1);

        if in_end.0 - in_start.0 != out_end.0 - out_start.0 || in_end.1 - in_start.1 != out_end.1 - out_start.1 {
            panic!("Dimension mismatch on slice set")
        }

        let in_height_range: Vec<usize> = (in_start.0..in_end.0).collect();
        let in_width_range: Vec<usize> = (in_start.1..in_end.1).collect();
        let out_height_range: Vec<usize> = (out_start.0..out_end.0).collect();
        let out_width_range: Vec<usize> = (out_start.1..out_end.1).collect();

        let height = in_end.0 - in_start.0;
        let width = in_end.1 - in_start.1;

        for i in 0..height {
            for j in 0..width {
                self[in_height_range[i]][in_width_range[j]] = value[out_height_range[i]][out_width_range[j]];
            }
        }
    }

    pub fn transform_and_set<F, U> (&mut self, value: &mut Matrix<U>, self_dims: SlicePos, f: F) where F: Fn(&T) -> U, U: Clone, U: Copy, U: ToString {
        if value.get_height() != self_dims.1.0 - self_dims.0.0 || value.get_width() != self_dims.1.1 - self_dims.0.1 {
            panic!("dimension mismatch in transform_and_set")
        }

        for i in 0..(self_dims.1.0 - self_dims.0.0) {
            for j in 0..(self_dims.1.1 - self_dims.0.1) {
                value[i][j] = f(&(self[i + self_dims.0.0][j + self_dims.0.1]));
            }
        }
    }

    pub fn for_each<F>(&mut self, f: F) where F: Fn(&mut T) -> () {
        for x in self.arr.iter_mut() {
            f(x);
        }
    }

    pub fn for_each_enumerate<F>(&mut self, f: F) where F: Fn((usize, usize), &mut T) -> () {
        for i in 0..self.height {
            for j in 0..self.width {
                f((i, j), &mut(self[i][j]));
            }
        }
    }

    pub fn for_each_enumerate_slice<F>(&mut self, f: F, slice_pos: SlicePos) where F: Fn((usize, usize), &mut T) -> () {
        for i in slice_pos.0.0..slice_pos.1.0 {
            for j in slice_pos.0.1..slice_pos.1.1 {
                // println!("{i}, {j}, {}, {}", self.get_height(), self.get_width());
                f((i, j), &mut(self[i][j]));
            }
        }
    }

    pub fn print(&self) -> () {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}, ", self[i][j].to_string())
            }
            println!();
        }
    }

}

impl<'a, T> Index<usize> for Matrix<'a, T> where T: Clone, T: Copy, T: ToString {
    type Output = [T];

    fn index(&self, index: usize) -> &[T] {
        if index >= self.height {
            panic!("Matrix error: Index out of range")
        }

        &(self.arr[self.width * index .. self.width * (index + 1)])
    }
}

impl<'a, T> IndexMut<usize> for Matrix<'a, T> where T: Clone, T: Copy, T: ToString {
    fn index_mut(&mut self, index: usize) -> &mut[T] {
        if index >= self.height {
            panic!("Matrix error: Index out of range")
        }

        &mut(self.arr[self.width * index .. self.width * (index + 1)])
    }
}

// impl<'a, T> Index<(Range<usize>, Range<usize>)> for Matrix<'a, T> where T: Clone {
//     type Output = MatrixSlice<'a, T>;

//     fn index(&self, index: (Range<usize>, Range<usize>)) -> &MatrixSliceMut<'a, T> {
//         let (start, end) = index;
        
//         if start.start < 0 || start.end < 0 || end.start >= self.height || end.end >= self.width {
//             panic!("Range index error (non-transpose)")
//         }
//         // let slice = &(self.arr[self.width * index.start as usize .. self.width * index.end as usize]);

//         &(MatrixSlice::new(self, start.start, end.start, start.end, end.end))
//     }
// }

// impl<'a, T> IndexMut<(Range<usize>, Range<usize>)> for Matrix<'a, T> where T: Clone {
//     fn index_mut(&mut self, index: (Range<usize>, Range<usize>)) -> &mut MatrixSliceMut<'a, T> {
//         let (start, end) = index;
        
//         if start.start < 0 || start.end < 0 || end.start >= self.height || end.end >= self.width {
//             panic!("Range index error (non-transpose)")
//         }
//         // let slice = &(self.arr[self.width * index.start as usize .. self.width * index.end as usize]);

//         &mut (MatrixSliceMut::new(self, start.start, end.start, start.end, end.end))
//     }
// }



// pub struct MatrixSlice<'a, T> where T: Clone {
//     mat: &'a Matrix<'a, T>,
//     x_start: usize,
//     x_end: usize,
//     y_start: usize,
//     y_end: usize
// }

// impl<'a, T> MatrixSlice<'a, T> where T: Clone {
//     pub fn new(mat: &'a Matrix<T>, x_start: usize, x_end: usize, y_start: usize, y_end: usize) -> MatrixSlice<'a, T> {
//         if x_start < 0 || y_start < 0 || x_end >= mat.get_height() || y_end >= mat.get_width() { panic!("Slice dimension mismatch") };
//         MatrixSlice { mat, x_start, x_end, y_start, y_end }
//     }

//     pub fn get_width(&self) -> usize {
//         self.y_end - self.y_start
//     }

//     pub fn get_height(&self) -> usize {
//         self.x_end - self.x_start
//     }

//     // pub fn set(&mut self, value: T) -> () {
//     //     for i in self.x_start .. self.x_end {
//     //         for j in self.y_start .. self.y_end {
//     //             self.mat[i][j] = value;
//     //         }
//     //     }
//     // }

//     // pub fn get(&self)

//     pub fn for_each<F>(&mut self, f: F) where F: Fn(&T) -> () {
//         for i in self.x_start .. self.x_end {
//             for j in self.y_start .. self.y_end {
//                 f(&(self.mat[i][j]));
//             }
//         }
//     }

//     // pub fn reset_iter(&mut self) -> () {
//     //     self.iteration = 0;
//     // }

//     pub fn for_each_enumerate<F>(&self, f: F) where F: Fn((usize, usize), &T) -> () {
//         for i in self.x_start .. self.x_end {
//             for j in self.y_start .. self.y_end {
//                 f((i, j), &(self.mat[i][j]));
//             }
//         }
//     }
// }


// pub struct MatrixSliceMut<'a, T> where T: Clone {
//     mat: &'a mut Matrix<'a, T>,
//     x_start: usize,
//     x_end: usize,
//     y_start: usize,
//     y_end: usize
// }

// impl<'a, T> MatrixSliceMut<'a, T> where T: Clone {
//     pub fn new(mat: &'a mut Matrix<T>, x_start: usize, x_end: usize, y_start: usize, y_end: usize) -> MatrixSliceMut<'a, T> {
//         if x_start < 0 || y_start < 0 || x_end >= mat.get_height() || y_end >= mat.get_width() { panic!("Slice dimension mismatch") };
//         MatrixSliceMut { mat, x_start, x_end, y_start, y_end }
//     }

//     pub fn get_width(&self) -> usize {
//         self.y_end - self.y_start
//     }

//     pub fn get_height(&self) -> usize {
//         self.x_end - self.x_start
//     }

//     pub fn set(&mut self, value: T) -> () {
//         for i in self.x_start .. self.x_end {
//             for j in self.y_start .. self.y_end {
//                 self.mat[i][j] = value;
//             }
//         }
//     }

//     // pub fn get(&self)

//     pub fn for_each<F>(&mut self, f: F) where F: Fn(&mut T) -> () {
//         for i in self.x_start .. self.x_end {
//             for j in self.y_start .. self.y_end {
//                 f(&mut (self.mat[i][j]));
//             }
//         }
//     }

//     // pub fn reset_iter(&mut self) -> () {
//     //     self.iteration = 0;
//     // }

//     pub fn for_each_enumerate<F>(&mut self, f: F) where F: Fn((usize, usize), &mut T) -> () {
//         for i in self.x_start .. self.x_end {
//             for j in self.y_start .. self.y_end {
//                 f((i, j), &mut (self.mat[i][j]));
//             }
//         }
//     }
// }

