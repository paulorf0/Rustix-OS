#![no_main]
#![no_std]

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};

entry_point!(kernel_main);
fn kernel_main(boot_info: &mut  BootInfo) -> ! {
   let framebuffer = boot_info.framebuffer.as_mut().expect("No framebuffer.");
   let info = framebuffer.info();
   let buffer = framebuffer.buffer_mut();
   let bytes_per_pixel = info.bytes_per_pixel;

   let bytes_middle_hscreen = (info.height / 2) * info.stride * bytes_per_pixel; 
    
   for x_coordinate in 0..info.width {
      let pixel_coordinate = bytes_middle_hscreen + x_coordinate * bytes_per_pixel;
      
      buffer[pixel_coordinate] = 0;
      buffer[pixel_coordinate + 1] = 255;
      buffer[pixel_coordinate + 2] = 0;
   }

   loop{} 
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
loop{}
}