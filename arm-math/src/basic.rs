use fixed::types::I1F7;
use num_traits::ToPrimitive;

use crate::error::Error;
use crate::scalar::Scalar;

pub trait Abs: Scalar {
    fn abs(src: &[Self], dst: &mut [Self]) -> Result<(), Error>;
    fn abs_in_place(values: &mut [Self]) -> Result<(), Error>;
}

impl Abs for I1F7 {
    fn abs(src: &[Self], dst: &mut [Self]) -> Result<(), Error> {
        if src.len() < dst.len() {
            Err(Error::SizeMismatch)
        } else {
            let src: &[i8] = unsafe { core::mem::transmute(src) };
            let dst: &mut [i8] = unsafe { core::mem::transmute(dst) };
            unsafe {
                arm_math_sys::bindings::arm_abs_q7(
                    src.as_ptr(),
                    dst.as_mut_ptr(),
                    src.len().to_u32().ok_or(Error::LengthError)?,
                );
            }
            Ok(())
        }
    }

    fn abs_in_place(values: &mut [Self]) -> Result<(), Error> {
        let values: &mut [i8] = unsafe { core::mem::transmute(values) };
        unsafe {
            arm_math_sys::bindings::arm_abs_q7(
                values.as_ptr(),
                values.as_mut_ptr(),
                values.len().to_u32().ok_or(Error::LengthError)?,
            );
        }
        Ok(())
    }
}
