#![no_std]
#![cfg_attr(test, no_main)]

use air_monitor as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
