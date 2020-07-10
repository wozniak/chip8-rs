mod chip8;

#[cfg(test)]
mod tests {
    #[test]
    fn test_00e0() {
        chip8 = chip8::Chip8::new();
        chip8.video = []
        chip8.op_00e0();
        assert_eq!(chip8.video, [])
    }
}