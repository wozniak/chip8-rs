use std::fs;
use std::path::Path;
use rand;

const VIDEO_WIDTH: usize = 64;
const VIDEO_HEIGHT: usize = 32;

pub struct Chip8 {
    pub(crate) registers: [u8; 16],
    pub(crate) memory: [u8; 4096],
    pub(crate) index: u16,
    pub(crate) pc: u16,
    pub(crate) stack: [u16; 16],
    pub(crate) sp: usize,
    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,
    pub(crate) keypad: [u8; 16],
    pub(crate) video: [[u8; VIDEO_HEIGHT]; VIDEO_WIDTH],
    pub(crate) opcode: u16,
    pub(crate) dostep: bool,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        let mut chip8 = Chip8 {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: 0x200-2,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            video: [[0; 32]; 64],
            opcode: 0,
            dostep: true,
        };

        Chip8::memset(&mut chip8, 0x50, &fontset);

        chip8
    }

    pub fn cycle(&mut self) {
        self.pc += 2;

        let first = (self.memory[self.pc as usize] as u16) << 8;
        let second = self.memory[self.pc as usize + 1] as u16;

        self.opcode = first | second;

        let n1 = (self.opcode & 0xf000) >> 12;
        let n2 = (self.opcode & 0x0f00) >> 8;
        let n3 = (self.opcode & 0x00f0) >> 4;
        let n4 =  self.opcode & 0x000f;

        match (n1, n2, n3, n4) {
            (0x0, 0x0, 0xE, 0x0) => self.op_00e0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00ee(),
            (0x1,   _,   _,   _) => self.op_1nnn(),
            (0x2,   _,   _,   _) => self.op_2nnn(),
            (0x3,   _,   _,   _) => self.op_3xkk(),
            (0x4,   _,   _,   _) => self.op_4xkk(),
            (0x5,   _,   _, 0x0) => self.op_5xy0(),
            (0x6,   _,   _,   _) => self.op_6xkk(),
            (0x7,   _,   _,   _) => self.op_7xkk(),
            (0x8,   _,   _, 0x0) => self.op_8xy0(),
            (0x8,   _,   _, 0x1) => self.op_8xy1(),
            (0x8,   _,   _, 0x2) => self.op_8xy2(),
            (0x8,   _,   _, 0x3) => self.op_8xy3(),
            (0x8,   _,   _, 0x4) => self.op_8xy4(),
            (0x8,   _,   _, 0x5) => self.op_8xy5(),
            (0x8,   _,   _, 0x6) => self.op_8xy6(),
            (0x8,   _,   _, 0x7) => self.op_8xy7(),
            (0x8,   _,   _, 0xE) => self.op_8xye(),
            (0x9,   _,   _, 0x0) => self.op_9xy0(),
            (0xA,   _,   _,   _) => self.op_annn(),
            (0xB,   _,   _,   _) => self.op_bnnn(),
            (0xC,   _,   _,   _) => self.op_cxkk(),
            (0xD,   _,   _,   _) => self.op_dxyn(),
            (0xE,   _, 0x9, 0xE) => self.op_ex9e(),
            (0xE,   _, 0xA, 0x1) => self.op_exa1(),
            (0xF,   _, 0x0, 0x7) => self.op_fx07(),
            (0xF,   _, 0x0, 0xA) => self.op_fx0a(),
            (0xF,   _, 0x1, 0x5) => self.op_fx15(),
            (0xF,   _, 0x1, 0x8) => self.op_fx18(),
            (0xF,   _, 0x1, 0xE) => self.op_fx1e(),
            (0xF,   _, 0x2, 0x9) => self.op_fx29(),
            (0xF,   _, 0x3, 0x3) => self.op_fx33(),
            (0xF,   _, 0x5, 0x5) => self.op_fx55(),
            (0xF,   _, 0x6, 0x5) => self.op_fx65(),
            (  _,   _,   _,   _) => panic!(format!("{:0<2x} {:0<2x} {:0<2x} {:0<2x} is not a valid instruction", n1, n2, n3, n4)),
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 1 {
            self.sound_timer -= 1;
        }

        //self.pc += 2;
    }

    pub fn dump_memory(&self) {
        for i in (0..self.memory.len()).step_by(16) {
            println!("{:0>8X}   {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X}   \
                {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X}",
                     i,
                     self.memory[i],
                     self.memory[i + 1],
                     self.memory[i + 2],
                     self.memory[i + 3],
                     self.memory[i + 4],
                     self.memory[i + 5],
                     self.memory[i + 6],
                     self.memory[i + 7],
                     self.memory[i + 8],
                     self.memory[i + 9],
                     self.memory[i + 10],
                     self.memory[i + 11],
                     self.memory[i + 12],
                     self.memory[i + 13],
                     self.memory[i + 14],
                     self.memory[i + 15]
            );
        }
    }

    pub fn load_rom(&mut self, filename: &str) {
        let path = Path::new(filename);
        let romfile = fs::read(&path).unwrap();

        self.memset(0x200, &romfile);
    }

    fn memset(&mut self, address: usize, buffer: &[u8]) {
        for i in 0..buffer.len() {
            self.memory[address + i] = buffer[i];
        }
    }

    pub(crate) fn op_00e0(&mut self) {
        // clear display
        println!("CLS");
        self.video = [[0; 32]; 64];
    }

    pub(crate) fn op_00ee(&mut self) {
        println!("RET");
        self.sp = self.sp - 1;
        self.pc = self.stack[self.sp];
    }

    pub(crate) fn op_1nnn(&mut self) {
        println!("JP ${:x}", self.opcode & 0x0fff);
        self.pc = self.opcode & 0x0fff;
    }

    pub(crate) fn op_2nnn(&mut self) {
        println!("CALL ${:x}", self.opcode & 0x0fff);
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = self.opcode & 0x0fff;
    }

    pub(crate) fn op_3xkk(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let byte = self.opcode & 0x00ff;

        println!("SE V{:x} ({}), ${:x}", vx, self.registers[vx], byte);

        if self.registers[vx as usize] == byte as u8 {
            self.pc += 2;
        }
    }

    pub(crate) fn op_4xkk(&mut self) {
        let vx = (self.opcode & 0x0ff0) as usize >> 8;
        let byte = self.opcode & 0x00ff;

        println!("SNE V{:x} ({}), ${:x}", vx, self.registers[vx], byte);

        if self.registers[vx as usize] != byte as u8 {
            self.pc += 2;
        }
    }

    pub(crate) fn op_5xy0(&mut self) {
        let vx = (self.opcode & 0x0ff0) as usize >> 8;
        let vy = (self.opcode & 0x00f0) as usize >> 4;

        println!("SE V{:x} ({}), V{:x} ({:x})", vx, self.registers[vx], vy, self.registers[vy]);

        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    pub(crate) fn op_6xkk(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let byte = self.opcode & 0x00ff;

        println!("LD V{}, ${:x}", vx, byte);

        self.registers[vx as usize] = byte as u8;
    }

    pub(crate) fn op_7xkk(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let byte = self.opcode & 0x00ff;

        println!("ADD V{}, ${:x}", vx, byte);

        self.registers[vx] = (self.registers[vx] as u16 + byte) as u8;
    }

    pub(crate) fn op_8xy0(&mut self) {
        let vx = (self.opcode & 0x0ff0) as usize >> 8;
        let vy = (self.opcode & 0x00f0) as usize >> 4;

        println!("LD V{}, V{}", vx, vy);

        self.registers[vx as usize] = self.registers[vy as usize];
    }

    fn op_8xy1(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let vy = (self.opcode & 0x00f0) as usize >> 4;
        self.registers[vx as usize] |= self.registers[vy as usize];
    }

    fn op_8xy2(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let vy = (self.opcode & 0x00f0) as usize >> 4;
        self.registers[vx as usize] &= self.registers[vy as usize];
    }

    fn op_8xy3(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let vy = (self.opcode & 0x00f0) as usize >> 4;

        self.registers[vx as usize] ^= self.registers[vy as usize];
    }

    fn op_8xy4(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let vy = (self.opcode & 0x00f0) as usize >> 4;

        let sum = self.registers[vx as usize] as u16 + self.registers[vy as usize] as u16;

        if sum > 255 {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }

        self.registers[vx as usize] = (sum & 0xff) as u8;
    }

    fn op_8xy5(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let vy = (self.opcode & 0x00f0) as usize >> 4;

        if self.registers[vx] < self.registers[vy] {
            self.registers[0xf] = 1;
            self.registers[vx] = self.registers[vy] - self.registers[vx];
        } else {
            self.registers[0xf] = 0;
            self.registers[vx] -= self.registers[vy];
        }
    }

    fn op_8xy6(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        self.registers[0xf] = self.registers[vx] & 0x1;
        self.registers[vx as usize] >>= 1;
    }

    fn op_8xy7(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let vy = (self.opcode & 0x00f) as usize >> 4;

        if self.registers[vy as usize] > self.registers[vx] {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }

        self.registers[vx] = self.registers[vy] - self.registers[vx];
    }

    fn op_8xye(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        self.registers[0xf] = (self.registers[vx] & 0x80) >> 7;
        self.registers[vx] <<= 1;
    }

    fn op_9xy0(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let vy = (self.opcode & 0x00f0) as usize >> 4;

        if self.registers[vx] != self.registers[vy] {
            self.pc += 2;
        }
    }

    fn op_annn(&mut self) {
        let address = self.opcode & 0x0fff;
        self.index = address;
    }

    fn op_bnnn(&mut self) {
        let address = self.opcode & 0x0fff;
        self.pc = self.registers[0] as u16 + address;
    }

    fn op_cxkk(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let byte = (self.opcode & 0x00ff) as usize;

        self.registers[vx] = (rand::random::<u16>() as usize & byte) as u8;
    }

    fn op_dxyn(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        let height = self.opcode & 0x000F;

        // Wrap if going beyond screen boundaries
        let x_pos = self.registers[vx as usize] % VIDEO_WIDTH as u8;
        let y_pos = self.registers[vy as usize] % VIDEO_HEIGHT as u8;

        self.registers[0xF] = 0;

        for row in 0..height as u8 {
            let sprite_byte = self.memory[(self.index + row as u16 ) as usize];
            for col in 0..8 {
                let sprite_pixel = sprite_byte & (0x80 >> col);
                let screen_pixel = &mut self.video[(x_pos + col) as usize][(y_pos + row) as usize];

                // Sprite pixel is on
                if sprite_pixel > 0 {
                    if *screen_pixel == 1 {
                        self.registers[0xF] = 1;
                    }

                    *screen_pixel ^= 1;
                }
            }
        }
    }

    fn op_exa1(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let key = self.registers[vx] as usize;

        if self.keypad[key] != 0 {
            self.pc += 2;
        }
    }

    fn op_ex9e(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let key = self.registers[vx] as usize;

        if self.keypad[key] != 0 {
            self.pc += 2;
        }
    }

    fn op_fx07(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        self.registers[vx] = self.delay_timer;
    }

    fn op_fx0a(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;

        for i in 0..16 {
            if self.keypad[i] != 0 {
                self.registers[vx] = i as u8;
                break
            }
        }

        self.pc -= 2;
    }

    fn op_fx15(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        self.delay_timer = self.registers[vx];
    }

    fn op_fx18(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        self.sound_timer = self.registers[vx];
    }

    fn op_fx1e(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        self.index += self.registers[vx] as u16;
    }

    fn op_fx29(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let digit = self.registers[vx];

        self.index = (0x50 + (5 * digit)).into();
    }

    fn op_fx33(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;
        let mut value = self.registers[vx];

        self.memory[(self.index + 2) as usize] = value % 10;
        value /= 10;

        self.memory[(self.index + 1) as usize] = value % 10;
        value /= 10;

        self.memory[self.index as usize] = value % 10;
    }

    fn op_fx55(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;

        for i in 0..vx + 1 {
            self.memory[self.index as usize + i] = self.registers[i];
        }
    }

    fn op_fx65(&mut self) {
        let vx = (self.opcode & 0x0f00) as usize >> 8;

        for i in 0..vx + 1 {
            self.registers[i] = self.memory[self.index as usize + i];
        }
    }
}