use half::{f16, bf16};

pub enum Type {
    FLOAT16(f16),
}

// restricted type trait for MAC types
pub trait TypeImpl {}
impl TypeImpl for i16 {}
impl TypeImpl for f32 {}
impl TypeImpl for f16 {}
impl TypeImpl for bf16 {}