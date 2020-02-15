#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u8 = 0xa0000 as *mut _;
const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 200;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut color: u8 = 0x00;
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            unsafe {
                VGA_BUFFER.offset((y * SCREEN_WIDTH + x) as isize).write_volatile(color);
                color = (color + 1) % 0xff;
            }
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
