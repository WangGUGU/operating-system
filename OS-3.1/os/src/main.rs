//! - #![no-std] 
//!   禁用标准库
#![no_std]
//! - #![no_main]
//!   不使用 main 函数作为程序入口
#![no_main]
//! 使用汇编
//! - #![feature(llvm_asm)]
//!   内嵌汇编
#![feature(llvm_asm)]

//! - #![feature(global_asm)]
//!   内嵌汇编文件
#![feature(global_asm)]

//! - #![feature(panic_info_message)] 
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]

//! - #![feature(alloc_error_handler)]
//!   内存分配错误回调
#![feature(alloc_error_handler)]

// 汇编程序入口
global_asm!(include_str!("entry.asm"));

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod memory;
extern crate alloc;

/// Rust 入口函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // 初始化
    interrupt::init();
    memory::init();

    // 物理页分配
    for _ in 0..2 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }

    panic!("Process ended. System shuting down.")
}
