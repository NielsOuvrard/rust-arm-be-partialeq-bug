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

pub fn writec(c: u8) {
    unsafe {
        core::arch::asm!(
            "svc 0x123456",          // A/R-profile semihosting trap
            in("r0") 3u32,           // SYS_WRITEC
            in("r1") &c as *const u8,
            options(nostack),
        );
    }
}

fn print_nmb_test(nb: u8) {
    writec(b'0' + nb);
    writec(b'o');
    writec(b'k');
    writec(b'\n');
}

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn my_main() -> ! {
    qemu_be_error::init();
    qemu_be_error::want_panic();

    // 1. Basic arithmetic
    let a: u32 = 0x01020304;
    let b: u32 = 0x04030201;
    assert_eq!(a.wrapping_add(b), 0x05050505);
    print_nmb_test(1);

    // 2. Byte order — most important for BE
    let val: u32 = 0xDEADBEEF;
    let bytes = val.to_be_bytes();
    assert_eq!(bytes, [0xDE, 0xAD, 0xBE, 0xEF]);

    print_nmb_test(2);

    // 3. u16 byte order
    // let bytes16 = val16.to_be_bytes();

    let val16: u16 = 0x1234;
    let raw: *const u8 = &val16 as *const _ as *const u8;
    unsafe {
        qemu_be_error::exit(*raw.offset(0) as i32);
    }

    // assert_eq!(bytes16[0], 0x34);
    // qemu_be_error::exit(bytes16[0] as i32); // 52 18
    //                                         // assert_eq!(bytes16[1], 0x12);
    // print_nmb_test(2);
    // assert_eq!(bytes16, [0x34, 0x12]);

    print_nmb_test(3);

    // 4. Struct layout in memory
    #[repr(C)]
    struct Header {
        a: u8,
        b: u8,
        c: u16,
    }
    let h = Header {
        a: 0x01,
        b: 0x02,
        c: 0x0304,
    };
    let raw: *const u8 = &h as *const _ as *const u8;
    unsafe {
        assert_eq!(*raw.offset(0), 0x01);
        assert_eq!(*raw.offset(1), 0x02);
        assert_eq!(*raw.offset(2), 0x03); // BE: high byte first
        assert_eq!(*raw.offset(3), 0x04);
    }
    print_nmb_test(4);

    // 5. Pointer arithmetic
    let arr: [u32; 4] = [0x11, 0x22, 0x33, 0x44];
    assert_eq!(arr[0], 0x11);
    assert_eq!(arr[3], 0x44);

    // 6. u32 from raw BE bytes (typical PUS packet parsing)
    let raw_bytes: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let parsed = u32::from_be_bytes(raw_bytes);
    assert_eq!(parsed, 0xDEADBEEF);

    writec(b'o');
    writec(b'k');

    qemu_be_error::exit(42);
    // All passed — exit with distinctive code
    panic!("I am an example panic");
}
