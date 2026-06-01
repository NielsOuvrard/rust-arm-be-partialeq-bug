#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    "
    .section .text._start
    .global _start
    .type _start, %function
_start:
    ldr sp, =__stack_top
    bl rust_main
1:
    b 1b
"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MyEnum {
    A,
    B,
    C,
    D,
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let a = MyEnum::A;
    let b = MyEnum::B;

    // with one of these, the test works
    // even if we uncomment the db one, the comparison of a variable will work
    // let da = unsafe { *(&a as *const MyEnum as *const u8) };
    // let db = unsafe { *(&b as *const MyEnum as *const u8) };

    if a == MyEnum::A {
        semihosting_exit(0)
    } else {
        semihosting_exit(1)
    }

    // with a match this works
    /*
    match a {
        MyEnum::A => semihosting_exit(0),
        MyEnum::B => semihosting_exit(1),
        MyEnum::C => semihosting_exit(2),
        _ => semihosting_exit(3),
    }
    */
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    semihosting_exit(1)
}

fn semihosting_exit(code: u32) -> ! {
    unsafe {
        let exit_data = [0x20026_u32, code];
        let args = exit_data.as_ptr();
        core::arch::asm!("svc 0x123456", in("r0") 0x20_u32, in("r1") args, options(noreturn));
    }
}
