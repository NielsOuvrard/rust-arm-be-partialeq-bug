//! Semihosting hello-world.

#![no_std]
#![no_main]

use aarch32_rt::entry;
use qemu_be_error as _;
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
    qemu_be_error::init();
    let x = 1.0f64;
    // or whatever semihosting crate you're using
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    qemu_be_error::want_panic();

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
