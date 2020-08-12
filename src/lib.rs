#![allow(dead_code, unused)]

#[macro_use]
pub mod parameters;
pub mod canvas;
pub mod models;

pub mod prelude {
    pub use super::canvas::Canvas;
    pub use super::models::prelude::*;
    pub use super::parameters::*;
}

mod requests;
