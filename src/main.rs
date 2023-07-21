#![no_std]
#![no_main]

use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // set memory allocator, initialize a logger and provide a panic handler
    uefi_services::init(&mut system_table).unwrap();

    log::info!("Hello world!");

    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}
