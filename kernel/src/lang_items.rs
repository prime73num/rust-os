


use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(pos) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            pos.file(),
            pos.line(),
            info.message().unwrap()
            );
    } else {
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
