
#![no_std]
#![no_main]

use user_lib::syscall::sys_exit;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    sys_exit(0);
    0
}
