use std::rc::Rc;

use crate::dialect::{Elem, Variable};

use crate::{CubeType, ExpandElement};

/// Form of CubeType that encapsulates all primitive types:
/// Numeric, UInt, Bool
pub trait PrimitiveVariable: CubeType<ExpandType = ExpandElement> {
    type Primitive;

    /// Return the element type to use on GPU
    fn into_elem() -> Elem;

    // For easy CPU-side casting
    fn to_f64(&self) -> f64;
    fn from_f64(val: f64) -> Self;
    fn from_i64(val: i64) -> Self;
}

macro_rules! impl_into_expand_element {
    ($type:ty) => {
        impl From<$type> for ExpandElement {
            fn from(value: $type) -> Self {
                ExpandElement::new(Rc::new(Variable::from(value)))
            }
        }
    };
}

impl_into_expand_element!(u32);
impl_into_expand_element!(usize);
impl_into_expand_element!(bool);
impl_into_expand_element!(f32);
impl_into_expand_element!(i32);
impl_into_expand_element!(i64);
