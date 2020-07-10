mod chip8;
extern crate pixel_canvas;

fn main() {
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom("BC_test.ch8");
    println!("chip8 loaded");

    const WHITE: pixel_canvas::Color = pixel_canvas::Color { r: 255, g: 255, b: 255 };
    const BLACK: pixel_canvas::Color = pixel_canvas::Color { r: 0,   g: 0,   b: 0   };

    let canvas = pixel_canvas::Canvas::new(64, 32)
        .title("chip-8");

    canvas.render(move |_, image| {
        chip8.cycle();
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                if chip8.video[y][x] == 1 {
                    *pixel = WHITE;
                } else {
                    *pixel = BLACK;
                }
            }
        }
    });
}