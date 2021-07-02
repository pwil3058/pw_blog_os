// Copyright 2021 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pw_blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use pw_blog_os::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use pw_blog_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "1");

    pw_blog_os::init();

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut fram_allocater = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    //let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    //memory::create_example_mapping(page, &mut mapper, &mut fram_allocater);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) }

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
