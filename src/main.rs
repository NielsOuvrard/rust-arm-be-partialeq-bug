//! Semihosting hello-world.

#![no_std]
#![no_main]

use aarch32_rt::entry;
use cortex_r5_emul as _;
use semihosting::println;

#[derive(PartialEq)]
#[repr(u8)]
pub enum MyEnum {
    A,
    B,
    C,
    D,
}

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn my_main() -> ! {
    cortex_r5_emul::init();
    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    cortex_r5_emul::want_panic();

    let a = MyEnum::A;
    let _b = MyEnum::B;

    let cat;
    if a == MyEnum::B {
        cat = "bug";
    } else {
        cat = "not bug";
    }
    println!("{cat}");

    panic!("I am an example panic");
}
