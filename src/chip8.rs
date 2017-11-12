// chip8.rs
// A Chip8 system is composed of a CPU, Screen and Keyboard

use cpu;
use screen;

struct Chip8{
    cpu: cpu::CPU,
    screen: screen::Screen,
} 

impl Chip8 {
    fn new(scale_factor: u32) -> Self {
        let w = screen::SCREEN_WIDTH;
        let h = screen::SCREEN_HEIGHT;
        Chip8 {
            cpu: cpu::CPU::new(),
            screen: screen::Screen::new(w, h, scale_factor),
        }
    }
    
}