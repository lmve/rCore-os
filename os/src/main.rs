// os/src/main.rs
// 2024.1.31    lucky ma
//

#![no_std]
#![no_main]
mod lang_items;

/*
 * include_str! 宏将同目录下的汇编代码
 * entry.asm转化为字符串
 * 通过global_sam! 宏嵌入到代码中
 */
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

fn main() {
    //println!("Hello, world!");
}


