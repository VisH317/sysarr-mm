use half::f16;
use std::{vec, clone::Clone, option::Option, option::Option::{None, Some}};
use crate::sysarr_utils::{stagger_horizontal, stagger_vertical};
use crate::matrix::{Matrix, SlicePos};


#[derive(Clone, Copy)]
enum DPUState {
    ACTIVE,
    DORMANT
}

#[derive(Clone, Copy)]
struct DPU {
    left: Option<f16>,
    top: Option<f16>,
    answer: f16,
    state: DPUState,
}

impl DPU {
    fn new() -> DPU {
        DPU { left: None, top: None, answer: f16::from_f32(0.0), state: DPUState::DORMANT }
    }

    fn clear(&mut self) -> () {
        self.left = None;
        self.top = None;
    }

    fn finish(&mut self) -> () {
        self.clear();
        self.state = DPUState::DORMANT;
    }

    fn set_left(&mut self, left: f16) -> () {
        self.state = DPUState::ACTIVE;
        self.left = Some(left);
    }

    fn set_top(&mut self, top: f16) -> () {
        self.state = DPUState::ACTIVE;
        self.top = Some(top);
    }

    fn get_answer(&self) -> f16 {
        if matches!(self.state, DPUState::DORMANT) {
            return f16::from_f32(0.0);
        }
        self.answer
    }

    fn compute(&mut self) -> () {
        if matches!(self.state, DPUState::DORMANT) {
            return;
        }

        match self.left {
            None => return,
            Some(l) => match self.top {
                None => return,
                Some(t) => {
                    // println!("{l}, {t}");
                    self.answer += t * l;
                    self.clear();
                }
            }
        }
    }
}

impl ToString for DPU {
    fn to_string(&self) -> String {
        String::from("DPU")
    }
}

// TODO: add multiple type support (currently only f16)

// note on the WCU: currently does not have a fixed size while the sysarr does for ease of implementation (this will be changed later)

struct TensorDims {
    left: Option<(usize, usize)>,
    top: Option<(usize, usize)>
}

impl TensorDims {
    fn new() -> TensorDims {
        TensorDims { left: None, top: None }
    }
}

struct WCU<'a> {
    left: Matrix<'a, f16>, // add a max size here?
    top: Matrix<'a, f16>,
    max_width: usize,
    max_height: usize,
    // is_active: bool,
    dim_config: TensorDims
}


impl<'a> WCU<'a> {
    fn new(max_width: usize, max_height: usize) -> WCU<'a> {
        let left = Matrix::<f16>::zeros(max_width, max_height);
        let top = Matrix::<f16>::zeros(max_width, max_height);

        WCU { left, top, max_width, max_height, dim_config: TensorDims::new() }
    }

    fn load_left(&mut self, mut left: Matrix<f16>) -> () {
        let left_stag = stagger_horizontal(&mut left); // idk about the double &mut reference, need to look into this more

        let height = left_stag.get_height();
        let width = left_stag.get_width();

        let start_idx = self.left.get_width() - width;


        let slice_idx: SlicePos = ((0, start_idx), (height, self.left.get_width()));
        self.left.set_matrix_slice(&left_stag, slice_idx, ((0, 0), (left_stag.get_height(), left_stag.get_width())));

        self.left.print();
        println!();
        
        self.dim_config.left = Some((width, height));
    }

    fn load_top(&mut self, mut top: Matrix<f16>) -> ()  {
        let top_stag = stagger_vertical(&mut top);

        let height = top_stag.get_height();
        let width = top_stag.get_width();

        let start_idx = self.top.get_height() - height;

        let slice_idx: SlicePos = ((start_idx, 0), (self.top.get_height(), width));
        self.top.set_matrix_slice(&top_stag, slice_idx, ((0, 0), (top_stag.get_height(), top_stag.get_width())));

        self.dim_config.top = Some((width, height))
    }

    fn check_ready(&self) -> bool {
        match self.dim_config.left {
            None => false,
            Some((l_w, _l_h)) => match self.dim_config.top {
                None => false,
                Some((_t_w, t_h)) => t_h == l_w
            }
        }
    }

    fn get_answer_dims(&self) -> (usize, usize) {
        match self.dim_config.left {
            None => panic!("Sizes don't exist"),
            Some((_l_w, l_h)) => match self.dim_config.top {
                None => panic!("Sizes don't exist"),
                Some((t_w, _t_h)) => (t_w, l_h)
            }
        }
    }

    fn clear_left(&mut self) -> () {
        self.left.set_all(f16::from_f32(0.0));
    }

    fn clear_top(&mut self) -> () {
        self.top.set_all(f16::from_f32(0.0));
    }

    fn get_left_slice(&self, step: u32) -> (SlicePos, usize, usize) {

        let max_idx = self.left.get_width() - 1;
        let start = max_idx as isize - step as isize;

        let zeroed = if start < 0 { 0 } else { start as usize };
        let offset = if start < 0 { (-1 * start) as usize } else { 0 };
        
        let slice_idx = ((0, zeroed), (self.left.get_height(), max_idx + 1));

        (slice_idx, offset, self.left.get_height()- zeroed)
    }

    fn get_top_slice(&self, step: u32) -> (SlicePos, usize, usize) {
        let max_idx = self.top.get_height() - 1;
        let start = max_idx as isize - step as isize; // starting point - goes from the end of the matrix to the start
        let zeroed = if start < 0 { 0 } else { start as usize }; // if we have to index something below 0 then we move everything over
        let offset = if start < 0 { (-1 * start) as usize } else { 0 }; // the offset to include in the systolic array when applying to DPUs

        let slice_idx = ((zeroed, 0), (max_idx + 1, self.top.get_width()));

        (slice_idx, offset, self.top.get_width() - zeroed)
    }

    fn get_top(&self) -> &Matrix<f16> {
        &(self.top)
    }

    fn get_left(&self) -> &Matrix<f16> {
        &(self.left)
    }
    
}


// helper function

pub enum State {
    OFF,
    READY,
    RUNNING,
    FINISHED
}

pub struct SysArr<'a> {
    width: usize,
    height: usize,
    array: Matrix<'a, DPU>,
    wcu: WCU<'a>,
    current_step: u32,
    state: State
}

impl<'a> SysArr<'a> {
    pub fn new(width: usize, height: usize) -> SysArr<'a> {
        let datalist: Vec<DPU> = vec![DPU::new(); width * height];
        let array: Matrix<DPU> = Matrix::new(width, height, datalist);

        SysArr { width, height, array, wcu: WCU::new(width, height), current_step: 0, state: State::OFF }
    }

    pub fn load<'b, 'c>(&mut self, mat1: Matrix<f16>, mat2: Matrix<f16>) -> () {
        let mat1_is_left = mat1.get_width() == mat2.get_height();
        if !mat1_is_left { panic!("Input mat mismatch") }

        self.wcu.load_left(mat1);
        self.wcu.load_top(mat2);

        self.state = State::READY;
    }


    fn clear(&mut self) -> () {
        self.wcu.clear_left();
        self.wcu.clear_top();
    }


    fn run_array(&mut self) -> () {
        self.array.for_each_enumerate(|(i, j), dpu| { dpu.compute()}) //println!("{}, {}", i, j);
    }


    fn clear_array(&mut self) -> () {
        self.array.for_each(|dpu| dpu.finish())
    }


    fn step(&mut self) -> () {
        if matches!(self.state, State::FINISHED) {
            return;
        } else if !matches!(self.state, State::RUNNING) {
            panic!("SysArr state mismatch")
        }

        let (left_slice, left_offset, left_len) = self.wcu.get_left_slice(self.current_step);
        let (top_slice, top_offset, top_len) = self.wcu.get_top_slice(self.current_step);

        if left_offset >= self.width && top_offset >= self.height {
            self.state = State::FINISHED;
            return;
        } else {
            if left_offset < self.width {
                let self_dims = ((0, left_offset), (self.height, (if left_offset + left_len >= self.width { self.width } else { left_offset + left_len })));
                let wcu_left = self.wcu.get_left();

                self.array.for_each_enumerate_slice(|(i, j), dpu| dpu.set_left(wcu_left[i - self_dims.0.0 + left_slice.0.0][j - self_dims.0.1 + left_slice.0.1]), self_dims);    
            }
            
            if top_offset < self.height {
                let self_dims = ((top_offset, 0), ((if top_offset + top_len >= self.height { self.height } else { top_offset + top_len }), self.width));
                let wcu_top = self.wcu.get_top();

                self.array.for_each_enumerate_slice(|(i, j), dpu| dpu.set_top(wcu_top[i - self_dims.0.0 + top_slice.0.0][j - self_dims.0.1 + top_slice.0.1]), self_dims);
            }

            self.run_array();

            self.current_step += 1; // end handling
        }
    }


    pub fn run(&mut self) -> () {
        if !matches!(self.state, State::READY) {
            panic!("state not ready!")
        }

        self.state = State::RUNNING;

        while matches!(self.state, State::RUNNING) {
            self.step();
        }
    }

    pub fn print_arr(&self) -> () {
        for i in 0..self.array.get_height() {
            for j in 0..self.array[0].len() {
                print!("{}, ", self.array[i][j].get_answer().to_string())
            }
            println!();
        }
    }

    pub fn get_output(&mut self) -> Result<Matrix<f16>, &'static str> {
        if !matches!(self.state, State::FINISHED) {
            return Err("SysArr not finished with computation");
        }
        
        let (width, height) = self.wcu.get_answer_dims();

        let mut mat = Matrix::<f16>::zeros(width, height);

        self.array.transform_and_set(&mut mat, ((0, 0), (height, width)), |dpu| dpu.get_answer());

        // clearing outputs
        self.clear_array();
        self.clear();

        return Ok(mat);
    }

    pub fn get_state(&self) -> &State {
        &(self.state)
    }
}

