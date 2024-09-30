// use std::ops::{Index, IndexMut, Range};
// use std::option::{Option, Option::{None, Some}};
// use std::marker::PhantomData;

// use half::f16;
// // matrix defs and helpers

// // pub type Matrix<T> = Vec<Vec<T>>;

// // requirements

// pub struct Matrix<'a, T> where T: Clone {
//     height: usize,
//     width: usize,
//     arr: Vec<T>,
//     phantom: PhantomData<&'a T> // for lifetime :D
// }

// impl<'a, T> Matrix<'a, T> where T: Clone {
//     pub fn new(width: usize, height: usize, data: Vec<T>) -> Matrix<'a, T> {
//         if width * height != data.len() { panic!("Matrix dimension mismatch") };
//         Matrix { width, height, arr: data, phantom: PhantomData }
//     }

//     pub fn zeros(width: usize, height: usize) -> Matrix<'a, f16> {
//         let data = vec![f16::from_f32(0.0); width*height];
//         Matrix { width, height, arr: data, phantom: PhantomData }
//     }

//     // fn new_sliced(width: usize, height: usize, data: Vec<T, inv: bool) -> Matrix<'a, T> {
//     //     if width * height != data.len() { panic!("Sliced matrix dimension mismatched") }

//     //     Matrix { width, height, arr: data, transpose_index: inv }
//     // }

//     pub fn get_width(&self) -> usize {
//         self.width
//     }

//     pub fn get_height(&self) -> usize {
//         self.height
//     }

//     pub fn rows(&self) -> Vec<Vec<T>> {
//         let mut ret: Vec<Vec<T>>;

//         for i in 0..self.height {
//             ret.push(self[i].to_vec());
//         };

//         ret
//     }

//     pub fn set(&mut self, value: T) -> () {
//         for x in &(self.arr) {
//             *x = value;
//         }
//     }

//     pub fn for_each<F>(&mut self, f: F) where F: Fn(&mut T) -> () {
//         for x in self.arr.iter_mut() {
//             f(x);
//         }
//     }

//     pub fn for_each_enumerate<F>(&mut self, f: F) where F: Fn((usize, usize), &mut T) -> () {
//         for i in 0..self.height {
//             for j in 0..self.width {
//                 f((i, j), &mut(self[i][j]));
//             }
//         }
//     }

// }

// impl<'a, T> Index<usize> for Matrix<'a, T> where T: Clone {
//     type Output = [T];

//     fn index(&self, index: usize) -> &[T] {
//         if index >= self.height {
//             panic!("Matrix error: Index out of range")
//         }

//         &(self.arr[self.width * index .. self.width * (index + 1)])
//     }
// }

// impl<'a, T> IndexMut<usize> for Matrix<'a, T> where T: Clone {
//     fn index_mut(&mut self, index: usize) -> &mut[T] {
//         if index >= self.height {
//             panic!("Matrix error: Index out of range")
//         }

//         &mut(self.arr[self.width * index .. self.width * (index + 1)])
//     }
// }

// impl<'a, T> Index<Range<usize>> for Matrix<'a, T> where T: Clone {
//     type Output = MatrixSlice<'a, T>;

//     fn index(&self, index: Range<usize>) -> &MatrixSlice<'a, T> {
//         if index.start < 0 || index.end as usize >= self.height || index.start >= index.end {
//             panic!("Range index error (non-transpose)")
//         }

//         let height = (index.end - index.start) as usize;
//         // let slice = &(self.arr[self.width * index.start as usize .. self.width * index.end as usize]);
//         let mut slice: Vec<&mut T>;

//         for i in index {
//             for val in self.arr[(i as usize)..].iter_mut().step_by(self.width) {
//                 slice.push(val);
//             }
//         }

//         &(MatrixSlice::new(self.width, height, slice))
//     }
// }

// pub struct MatrixSlice<'a, T> {
//     height: usize,
//     width: usize,
//     arr: Vec<&'a mut T>,
//     iteration: usize
// }

// impl<'a, T> MatrixSlice<'a, T> {
//     pub fn new(width: usize, height: usize, data: Vec<&'a mut T>) -> MatrixSlice<'a, T> {
//         if width * height != data.len() { panic!("Matrix dimension mismatch") };
//         MatrixSlice { width, height, arr: data, iteration: 0 }
//     }

//     pub fn get_width(&self) -> usize {
//         self.width
//     }

//     pub fn get_height(&self) -> usize {
//         self.height
//     }

//     pub fn set(&mut self, value: T) -> () {
//         for x in self.arr.iter_mut() {
//             **x = value;
//         }
//     }

//     pub fn for_each<F>(&mut self, f: F) where F: Fn(&mut T) -> () {
//         for x in self.arr.iter_mut() {
//             f(x);
//         }
//     }

//     pub fn reset_iter(&mut self) -> () {
//         self.iteration = 0;
//     }

//     pub fn for_each_enumerate<F>(&self, f: F) where F: Fn((usize, usize), &mut T) -> () {
//         for i in 0..self.height {
//             for j in 0..self.width {
//                 f((i, j), &mut(self[i][j]));
//             }
//         }
//     }
// }


// impl<'a, T> Iterator for MatrixSlice<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<&'a mut T> {
//         self.iteration += 1;

//         if self.iteration > self.arr.len() {
//             return None;
//         }

//         let val = &(self.arr[self.iteration]);

//         return Some(self.arr[self.iteration]);
//     }
// }


// impl<'a, T> Index<usize> for MatrixSlice<'a, T> {
//     type Output = [&'a mut T];

//     fn index(&self, index: usize) -> &[&'a mut T] {
//         if index >= self.height {
//             panic!("Matrix error: Index out of range")
//         }

//         &(self.arr[self.width * index .. self.width * (index + 1)])
//     }
// }

// impl<'a, T> IndexMut<usize> for MatrixSlice<'a, T> {
//     fn index_mut(&mut self, index: usize) -> &mut [&'a mut T] {
//         if index >= self.height {
//             panic!("Matrix error: Index out of range")
//         }

//         &mut(self.arr[self.width * index .. self.width * (index + 1)])
//     }
// }

// impl<'a, T> Index<Range<usize>> for MatrixSlice<'a, T> {
//     type Output = MatrixSlice<'a, T>;

//     fn index(&self, index: Range<usize>) -> &MatrixSlice<'a, T> {
//         if index.start < 0 || index.end as usize >= self.height || index.start >= index.end {
//             panic!("Range index error (non-transpose)")
//         }

//         let height = (index.end - index.start) as usize;
//         // let slice = &(self.arr[self.width * index.start as usize .. self.width * index.end as usize]);
//         let mut slice: Vec<&mut T>;

//         for i in index { //
//             for &val in self.arr[(i as usize)..].iter() {
//                 slice.push(val);
//             }
//         }

//         &(MatrixSlice::new(self.width, height, slice))
//     }
// }

// // impl<T> IndexMut<usize> for Matrix<T> {

    

// //     fn index(&self, index: usize) -> Row<T> {
// //         let (width, height) = index;
// //         if width >= self.width || height >= self.height {
// //             panic!("Matrix error: Index out of range")
// //         }

// //         self.arr[self.width * index .. self.width * (index + 1) - 1]
// //     }
// // }



// // pub fn print_matrix<T: ToString>(mat: &Matrix<T>) -> () {

// //     for i in 0..mat.len() {
// //         for j in 0..mat[0].len() {
// //             print!("{}, ", mat[i][j].to_string())
// //         }
// //         println!();
// //     }
// // }

// // pub fn create_matrix()

// // will probably make a custom matrix type here when we get to the memory controller