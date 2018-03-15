#![feature(asm, concat_idents, const_fn, const_cell_new, try_from)]
#![no_std]
#![crate_name = "cc2538"]
#![crate_type = "rlib"]
extern crate cortexm3;
#[allow(unused_imports)]
#[macro_use(debug)]
extern crate kernel;


//#[macro_us'extern]


// Amod peripheral_registers;

pub mod chip;
pub mod crt1;


pub use crt1::init;
