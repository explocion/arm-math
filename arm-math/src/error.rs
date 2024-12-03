use arm_math_sys::bindings::arm_status;

use crate::sealed::Sealed;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    ArgumentError,
    LengthError,
    NanInfError,
    SingularError,
    SizeMismatch,
    TestFailure,
    Unknown,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::error::Error for Error {}

pub trait Status {
    fn check(self) -> Result<(), Error>;
}

impl Sealed for arm_status::Type {}

impl Status for arm_status::Type {
    fn check(self) -> Result<(), Error> {
        match self {
            arm_status::ARM_MATH_SUCCESS => Ok(()),
            arm_status::ARM_MATH_ARGUMENT_ERROR => Err(Error::ArgumentError),
            arm_status::ARM_MATH_LENGTH_ERROR => Err(Error::LengthError),
            arm_status::ARM_MATH_NANINF => Err(Error::NanInfError),
            arm_status::ARM_MATH_SINGULAR => Err(Error::SingularError),
            arm_status::ARM_MATH_SIZE_MISMATCH => Err(Error::SizeMismatch),
            arm_status::ARM_MATH_TEST_FAILURE => Err(Error::TestFailure),
            _ => Err(Error::Unknown),
        }
    }
}
