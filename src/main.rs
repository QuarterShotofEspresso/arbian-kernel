
#![no_std]
#![no_main]

use core::panic::PanicInfo;

//static HELLO: &[u8] = b"Hellow World!";

use vga_buffer::Color;
use vga_buffer::Buffer;
use vga_buffer::Writer;
mod vga_buffer;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    let mut myBuffer: Buffer = Buffer::new(Color::Pink, Color::DarkGray);
    myBuffer.write_byte(&b'H');
    myBuffer.write_byte(&b'e');
    myBuffer.write_byte(&b'\n');

    myBuffer.write_string(&"Hello\nWonderful\nPeople");
    myBuffer.write_string(&"Hello\nWonderful\nPeople\t");

    let mut myWriter: Writer = Writer::new(Color::Yellow, Color::Black);
    myWriter.write_string("Hello Beauultiful World. What do you say to anevening snaack on this fine afternoon. Hmmm?");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    loop {}

}
