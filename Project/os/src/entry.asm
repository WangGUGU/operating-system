
 	.section .text.entry
	.globl _start
# ⽬前 _start 的功能：将预留的栈空间写⼊ $sp，然后跳转⾄ rust_main
_start:
	la sp, boot_stack_top
	call rust_main
 # 回忆：bss 段是 ELF ⽂件中只记录⻓度，⽽全部初始化为 0 的⼀段内存空间
 # 这⾥声明字段 .bss.stack 作为操作系统启动时的栈
	.section .bss.stack
	.global boot_stack
boot_stack:
 # 16K 启动栈⼤⼩
	.space 4096 * 16
	.global boot_stack_top
boot_stack_top:
 # 栈结尾
