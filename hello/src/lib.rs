#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::naked_asm;

const FLAG_LE: usize = 0b0;
const FLAG_PAGE_SIZE_4K: usize = 0b10;
const FLAG_ANY_MEM: usize = 0b1000;

#[naked]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.head")]
/// The entry point of the kernel.
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        naked_asm!(
            // code0/code1
            "nop",
            "bl {entry}",
            // text_offset
            ".quad 0",
            // image_size
            ".quad _kernel_size",
            // flags
            ".quad {flags}",
            // Reserved fields
            ".quad 0",
            ".quad 0",
            ".quad 0",
            // magic - yes 0x644d5241 is the same as ASCII string "ARM\x64"
            ".ascii \"ARM\\x64\"",
            // Another reserved field at the end of the header
            ".byte 0, 0, 0, 0",
            flags = const FLAG_LE | FLAG_PAGE_SIZE_4K | FLAG_ANY_MEM,
            entry = sym primary_entry,
        )
    }
}

#[naked]
#[unsafe(link_section = ".text.boot")]
/// The entry point of the kernel.
unsafe extern "C" fn primary_entry() -> ! {
    unsafe {
        naked_asm!(
            "ADR      x11, .",
            "LDR      x10, ={this_func}",
            "SUB      x18, x10, x11", // x18 = va_offset
            "MOV      x19, x0",        // x19 = dtb_addr

            // setup stack
            "LDR      x1,  =_stack_top",
            "SUB      x1,  x1, x18", // X1 == STACK_TOP
            "MOV      sp,  x1",


            "MOV      x0,  x18",
            "MOV      x1,  x19",
            "BL       {entry}",
            this_func = sym primary_entry,
            entry = sym rust_entry,
        )
    }
}

fn rust_entry(text_va: usize, fdt: *mut u8) -> ! {
    



    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
