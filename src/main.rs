// Copyright 2021 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pw_blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use pw_blog_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "1");

    pw_blog_os::init();

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    let ptr = 0x204adb as *mut u32;
    unsafe {
        let x = *ptr;
    }
    println!("read worked");

    //let ptr = 0xdeadbeef as *mut u32;
    unsafe {
        *ptr = 42;
    }

    #[cfg(test)]
    test_main();

    println!("it did not crash");
    pw_blog_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    pw_blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pw_blog_os::test_panic_handler(info)
}
