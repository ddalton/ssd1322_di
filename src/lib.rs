#![no_std]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

//! Builder example
extern crate embedded_hal as hal;

pub mod builder;
mod command;
mod displaysize;
mod graphics;
pub mod prelude;
pub mod properties;

pub use crate::builder::Builder;
