// os/src/lang_items.rs
// 2024.1.31


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}
