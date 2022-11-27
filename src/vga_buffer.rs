#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// Consider: Hello!\n World!
#[repr(C)]
struct CdPg437Char {
    character: u8,
    color_code: u8,
}

const BUF_WIDTH:  usize = 50;
const BUF_HEIGHT: usize = 50;

pub struct Writer {
    cursor_x: usize,
    cursor_y: usize,
    color_code:  u8,
    buffer:      &'static mut [[CdPg437Char; BUF_WIDTH]; BUF_HEIGHT],
}

impl Writer {

    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            color_code: ((background as u8) << 4 | (foreground as u8)),
            buffer: unsafe {&mut *(0xb8000 as *mut [[CdPg437Char; BUF_WIDTH]; BUF_HEIGHT])}
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        // write data to the console
        match byte {
            b'\n' => {
                self.cursor_y += 1;
                self.cursor_x =  0;
            },
            0x20..=0x7e => {
                self.buffer[self.cursor_y][self.cursor_x] = CdPg437Char {
                    character:  byte,
                    color_code: self.color_code
                };
                self.cursor_x += 1;
            },
            _ => {
                self.buffer[self.cursor_y][self.cursor_x] = CdPg437Char {
                    character:  0xfe,
                    color_code: self.color_code
                };
                self.cursor_x += 1;
            }
        }
        // increment the cursor location
        if self.cursor_x > (BUF_WIDTH-1) {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }
        if self.cursor_y > (BUF_HEIGHT-1) {
            self.cursor_x = 0;
            self.cursor_y = 0;
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }
}

pub struct Buffer {
    cursor_pos: usize,
    color_code: u8
}

impl Buffer {

    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            cursor_pos: 0,
            color_code: ((background as u8) << 4 | (foreground as u8))
        }
    }

    pub fn write_byte(&mut self, byte: &u8) {
        let vga_buffer = 0xb8000 as *mut u8;
        // write data to the console
        match byte {
            b'\n' => self.cursor_pos = (self.cursor_pos & 0xffff0) + 0x50,
            0x20..=0x7e  => {
                unsafe {
                    *vga_buffer.offset(self.cursor_pos as isize * 2) = *byte;
                    *vga_buffer.offset(self.cursor_pos as isize * 2 + 1) = self.color_code;
                }
                // increment the cursor location
                self.cursor_pos = self.cursor_pos + 1;
            },
            _ => unsafe {
                *vga_buffer.offset(self.cursor_pos as isize * 2) = 0xfe;
                *vga_buffer.offset(self.cursor_pos as isize * 2 + 1) = self.color_code;
            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(&byte);
        }
    }
}

