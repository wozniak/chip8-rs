#[cfg(test)]
use crate::chip8::Chip8;

#[test]
fn test_00e0() {
    let mut chip8 = Chip8::new();
    chip8.video = [[1 as u8; 32]; 64];
    chip8.op_00e0();
    for row in 0..64 {
        for col in 0..32 {
            assert_eq!(chip8.video[row][col], 0);
        }
    }
}

#[test]
fn test_00ee() {
    let mut chip8 = Chip8::new();
    chip8.sp = 0x1;
    chip8.stack[chip8.sp - 1] = 0xf;
    chip8.op_00ee();
    assert_eq!(chip8.pc, 0xf);
    assert_eq!(chip8.sp, 0x0);
}

#[test]
fn test_1nnn() {
    let mut chip8 = Chip8::new();
    chip8.opcode = 0x1300;
    chip8.op_1nnn();
    assert_eq!(chip8.pc, 0x300)
}

#[test]
fn test_2nnn() {
    let mut chip8 = Chip8::new();
    chip8.opcode = 0x2300;
    chip8.pc = 0x200;
    chip8.op_2nnn();

    assert_eq!(chip8.pc, 0x300);
    assert_eq!(chip8.stack[chip8.sp - 1], 0x200);
}

#[test]
fn test_3xkk() {
    let mut chip8 = Chip8::new();
    chip8.pc = 0x200;
    chip8.opcode = 0x30ff;
    chip8.registers[0] = 0xff;
    chip8.op_3xkk();
    assert_eq!(chip8.pc, 0x202);

    chip8.pc = 0x200;
    chip8.opcode = 0x30ff;
    chip8.registers[0] = 0x00;
    chip8.op_3xkk();
    assert_eq!(chip8.pc, 0x200);
}

#[test]
fn test_4xkk() {
    let mut chip8 = Chip8::new();
    chip8.pc = 0x200;
    chip8.opcode = 0x30ff;
    chip8.registers[0] = 0x0;
    chip8.op_4xkk();
    assert_eq!(chip8.pc, 0x202);

    chip8.pc = 0x200;
    chip8.opcode = 0x30ff;
    chip8.registers[0] = 0xff;
    chip8.op_4xkk();
    assert_eq!(chip8.pc, 0x200);
}

#[test]
fn test_5xy0() {
    let mut chip8 = Chip8::new();
    chip8.pc = 0x200;
    chip8.opcode = 0x5010;
    chip8.op_5xy0();
    assert_eq!(chip8.pc, 0x202);

    chip8.pc = 0x200;
    chip8.registers[1] = 0xff;
    chip8.op_5xy0();
    assert_eq!(chip8.pc, 0x200);
}

#[test]
fn test_6xkk() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0xff;
    chip8.opcode = 0x6000;
    chip8.op_6xkk();

    assert_eq!(chip8.registers[0], 0);
}

#[test]
fn test_7xkk() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0xf;
    chip8.opcode = 0x70f0;

    chip8.op_7xkk();
    assert_eq!(chip8.registers[0], 0xff);
}

#[test]
fn test_8xy0() {
    let mut chip8 = Chip8::new();
    chip8.registers[1] = 0xff;
    chip8.opcode = 0x8010;
    chip8.op_8xy0();

    assert_eq!(chip8.registers[0], 0xff);
}

#[test]
fn test_8xy1() {

}