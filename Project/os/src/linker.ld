OUTPUT_ARCH(riscv)

ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
	. = BASE_ADDRESS;
	kernel_start = .;
	text_start = .;
	.text : {
		*(.text.entry)
		*(.text .text.*)
	}
	rodata_start = .;
 	/* .rodata 字段 */
 	.rodata : {
 	/* 要链接的⽂件的 .rodata 字段集中放在这⾥ */
 	*(.rodata .rodata.*)
 	}
 	data_start = .;
 	/* .data 字段 */
 	.data : {
	 /* 要链接的⽂件的 .data 字段集中放在这⾥ */
	 *(.data .data.*)
	 }
 	bss_start = .;
	 /* .bss 字段 */
	 .bss : {
 	/* 要链接的⽂件的 .bss 字段集中放在这⾥ */
 	*(.sbss .bss .bss.*)
 	}
	 /* 结束地址 */
 	kernel_end = .;
}

