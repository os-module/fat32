#![feature(exclusive_range_pattern)]
#![feature(associated_type_defaults)]
#![feature(error_in_core)]
#![no_std]
mod cache;
mod device;
mod dir;
mod entry;
mod fat32;
mod layout;
mod utils;

extern crate alloc;

pub use crate::fat32::Fat32;
pub use device::BlockDevice;
pub use dir::{Dir, File, OperationError};
