/* 
 *   sbi.rs        os/src/sbi.rs
 *   2024.2.4      lucky_ma
 */
#![allow(unused)]
// 实现输出功能
pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

// 实现关机功能 failure表示是否正常退出
pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
