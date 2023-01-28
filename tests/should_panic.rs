#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::{QemuExitCode, exit_qemu, serial_println};
use rust_os::serial_print;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop{}
}

#[no_mangle]
extern "C" fn _start() -> !{
    should_fail();
    serial_println!("Test did not panic");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

fn should_fail(){
    serial_print!("This test should panic..should fail\t");
    assert_eq!(0,1);
}