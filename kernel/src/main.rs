#![no_std]
#![no_main]

bootloader_api::entry_point!(kernel_main);

// OS Entry point
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
