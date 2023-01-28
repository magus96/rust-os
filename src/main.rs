#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use core::panic::PanicInfo;
use rust_os::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> !{

    use rust_os::allocator;
    println!("Hello World {}", 1.3);
    rust_os::init();
    
    
    //create page fault
    use x86_64::{structures::paging::{Translate, Page}, registers::control::Cr3, VirtAddr};
    use rust_os::{memory, memory::BootInfoFrameAllocator};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe{ memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe{ BootInfoFrameAllocator::init(&boot_info.memory_map)};

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    
    let heap_value = Box::new(41);
    println!("heap value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500{
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 2]);
    let cloned_refernce = reference_counted.clone();
    println!("current refernce count is {}", Rc::strong_count(&cloned_refernce));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_refernce));

    
    // let page = Page::containing_address(VirtAddr::new(0));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe{ page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    

    #[cfg(test)]
    test_main();
    
    println!("it did not crash");
    rust_os::hlt_loop();
}

// #[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> !{

    
// }


//0x207396
//panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    rust_os::hlt_loop();  
}

//panic handler prints to serial port for testing
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    rust_os::test_panic_handler(info)
}


