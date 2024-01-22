#![no_std]
#![no_main]

mod writer;

use bootloader_api::config::Mapping;
use writer::FrameBufferWriter;
use x86_64::instructions::hlt;

//Use the entry_point macro to register the entry point function: bootloader_api::entry_point!(kernel_main)
//optionally pass a custom config
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

macro_rules! println {
    ($fb_writer:expr, $($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($fb_writer, $($arg)*);
    }};
}


bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {

    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();

    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    let mut frame_buffer_writer = 
        FrameBufferWriter::new(buffer, frame_buffer_info);
    // change the position to (80,60) then print 
    frame_buffer_writer.set_position(80, 60);
    println!(frame_buffer_writer, "Testing testing {} and {} with println! macro", 1, 50.0/2.0);
    //change the postion again to (180,200)
    frame_buffer_writer.set_position(180, 200);
    println!(frame_buffer_writer, "Testing testing again with println! macro!");
    //and again to (200,500)
    frame_buffer_writer.set_position(200, 500);
    println!(frame_buffer_writer, "Testing testing again and again with println! macro!");
   






    loop {
        hlt(); 
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}