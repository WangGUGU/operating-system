use crate::sbi::*;
use core::fmt::{self, Write};

struct Stdout;
impl Write for Stdout {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		let mut buffer = [0u8; 4];
		for c in s.chars() {
			for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
				console_putchar(*code_point as usize);
 			}
 		}
 		Ok(())
	}
}
pub fn print(args: fmt::Arguments) {
	Stdout.write_fmt(args).unwrap();
}
/// 实现类似于标准库中的 `print!` 宏
///
/// 使⽤实现了 [`core::fmt::Write`] trait 的 [`console::Stdout`]
#[macro_export]
macro_rules! print {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::print(format_args!($fmt $(, $($arg)+)?));
	}
}
/// 实现类似于标准库中的 `println!` 宏
///
/// 使⽤实现了 [`core::fmt::Write`] trait 的 [`console::Stdout`]
#[macro_export]
macro_rules! println {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::print(format_args!(concat!($fmt, "\n") $(,$($arg)+)?));
	}
}
