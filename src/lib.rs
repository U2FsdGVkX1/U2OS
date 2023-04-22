#![no_std]
#![no_main]
#![feature(int_log)]

#[macro_use]
mod register;
mod drivers;
mod panic;
mod kmain;
mod memory;
use core::arch::global_asm;

global_asm!(include_str!("boot/start.S"));