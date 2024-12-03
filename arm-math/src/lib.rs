#![no_std]

pub(crate) mod sealed {
    pub trait Sealed {}
}

pub mod error;
pub mod scalar;

pub mod basic;
