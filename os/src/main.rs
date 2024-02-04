/* os/src/main.rs
* 2024.1.31    lucky ma
* 2024.2.4     实现格式化输出 编写基于console_putchar的println!宏
*/

#![no_std]
#![no_main]
#![feature(panic_info_message)]

//实现paintln!
#[macro_use]
mod console;
mod lang_items;
mod sbi;


/*
 * include_str! 宏将同目录下的汇编代码
 * entry.asm转化为字符串
 * 通过global_sam! 宏嵌入到代码中
 */
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

use crate::sbi::shutdown;
#[no_mangle] // 以免编译器混淆名字
pub fn rust_main() -> !{
    clear_bss();
    println!("Hello,world!");
    panic!("Shutdown machine!");
    //shutdown(true);
}
// 完成对.bss段的清零操作
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

