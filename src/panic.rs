use core::panic::PanicInfo;
use crate::{print, println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}