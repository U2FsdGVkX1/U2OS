#![allow(dead_code)]
use core::arch::asm;

#[inline]
pub fn mret() {
    unsafe { asm!("mret") }
}