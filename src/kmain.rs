use core::arch::asm;
use crate::register;
use crate::drivers;
use crate::memory;
use crate::{print, println};

static mut STACK: usize = 0;

#[no_mangle]
unsafe fn kernel_init(stack: usize) {
    // save stack address
    STACK = stack;

    // set MPP to S-mode
    let mut status = r_reg!("mstatus");
    status &= !(register::status::MPP_M);
    status |= register::status::MPP_S;
    w_reg!("mstatus", status);

    // set MEPC to kernel_main
    w_reg!("mepc", kernel_main as usize);

    // disable paging
    w_reg!("satp", 0);

    // // delegate all interrupts and exceptions
    let sie = r_reg!("mie");
    w_reg!("medeleg", 0xffff);
    w_reg!("mideleg", 0xffff);
    w_reg!("sie", sie
                | register::ie::SEIE
                | register::ie::STIE
                | register::ie::SSIE);

    // // allow access to all memory
    w_reg!("pmpaddr0", 0x3fffffffffffffusize);
    w_reg!("pmpcfg0", 0xf);

    // // finish
    register::ret::mret();
}

#[no_mangle]
unsafe fn kernel_main() {
    drivers::UART::default().init();
    let t = memory::buddy::BuddySystem::new(STACK as *mut u8, 0x88000000 as *mut u8);
    let t1 = t.malloc(1);
    let t2 = t.malloc(2);
    let t3 = t.malloc(3);
    let t4 = t.malloc(2);
    let t5 = t.malloc(4);
    let t6 = t.malloc(1);
    let t7 = t.malloc(2);
    println!("{:p} {:p} {:p} {:p} {:p} {:p} {:p}", t1, t2, t3, t4, t5, t6, t7);
    println!("kernel is booting");
    println!("stack address: 0x{:x}", STACK);
    loop {}
}