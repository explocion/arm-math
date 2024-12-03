use fixed::{
    traits::Fixed,
    types::{I1F15, I1F31, I1F63, I1F7},
};
use num_traits::Float;

use crate::sealed::Sealed;

pub trait Scalar: Sized + Sealed {}

impl Sealed for I1F7 {}
impl Scalar for I1F7 {}

impl Sealed for I1F15 {}
impl Scalar for I1F15 {}

impl Sealed for I1F31 {}
impl Scalar for I1F31 {}

impl Sealed for I1F63 {}
impl Scalar for I1F63 {}

impl Sealed for f32 {}
impl Scalar for f32 {}

impl Sealed for f64 {}
impl Scalar for f64 {}

pub trait FixedScalar: Scalar + Fixed + Sealed {}

impl FixedScalar for I1F7 {}

impl FixedScalar for I1F15 {}

impl FixedScalar for I1F31 {}

impl FixedScalar for I1F63 {}

pub trait FloatScalar: Scalar + Float + Sealed {}

impl FloatScalar for f32 {}

impl FloatScalar for f64 {}
