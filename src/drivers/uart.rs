#![allow(dead_code)]
use core::fmt::{Result, Write};

const RHR: usize = 0x00;
const THR: usize = 0x00;
const DLL: usize = 0x00;
const IER: usize = 0x01;
const DLM: usize = 0x01;
const IIR: usize = 0x02;
const FCR: usize = 0x02;
const LCR: usize = 0x03;
const MCR: usize = 0x04;
const LSR: usize = 0x05;
const MSR: usize = 0x06;
const SCR: usize = 0x07;

pub struct UART(*mut u8);
impl UART {
    #[inline]
    fn read(&self) -> u8 {
        unsafe { *self.0 }
    }
    #[inline]
    fn write(&self, offset: usize, byte: u8) {
        unsafe { *self.0.add(offset) = byte }
    }
    pub fn init(&self) {
        self.write(IER, 0x00);
        self.write(LCR, 0x80);
        self.write(DLL, 0x03);
        self.write(DLM, 0x00);
        self.write(LCR, 0x03);
        self.write(FCR, 0x03);
        self.write(IER, 0x03);
    }
}

impl Write for UART {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            self.write(0, c as u8);
        }
        Ok(())
    }
}

impl Default for UART {
    fn default() -> Self {
        Self(0x10000000 as *mut u8)
    }
}