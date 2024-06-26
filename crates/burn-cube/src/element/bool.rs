use crate::dialect::Elem;

use crate::{CubeContext, CubeType, ExpandElement, PrimitiveVariable};

#[derive(Clone, Copy)]
/// Boolean type for kernels
pub struct Bool {
    pub val: <Self as PrimitiveVariable>::Primitive,
    pub vectorization: u8,
}

impl CubeType for Bool {
    type ExpandType = ExpandElement;
}

impl Bool {
    /// Make a boolean literal
    pub fn new(val: <Self as PrimitiveVariable>::Primitive) -> Self {
        Self {
            val,
            vectorization: 1,
        }
    }

    /// Expand version of lit
    pub fn new_expand(
        _context: &mut CubeContext,
        val: <Self as PrimitiveVariable>::Primitive,
    ) -> <Self as CubeType>::ExpandType {
        val.into()
    }
}

impl PrimitiveVariable for Bool {
    type Primitive = bool;

    fn into_elem() -> Elem {
        Elem::Bool
    }

    fn to_f64(&self) -> f64 {
        match self.val {
            true => 1.,
            false => 0.,
        }
    }

    fn from_f64(val: f64) -> Self {
        Self::new(val > 0.)
    }

    fn from_i64(val: i64) -> Self {
        Self::from_f64(val as f64)
    }
}
