#![deny(unsafe_op_in_unsafe_fn)]

mod common;
pub use common::*;

pub mod api;
pub mod dll;

mod syscall;
