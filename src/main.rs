
#![allow(dead_code)]

mod cpu;
mod screen;

extern crate rand;
extern crate sdl2;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

static FONT_FILE: &'static str = "FONTS.chip8";

fn main() {
    println!("Hello, world!");
    let  s = screen::Screen::new(
        u32::from(screen::SCREEN_WIDTH), 
        u32::from(screen::SCREEN_HEIGHT), 
        5);

    let mut c = cpu::CPU::new(Some(s));
    c.load_rom(FONT_FILE, 0);
    c.load_rom("PONG", cpu::PC_START );

//    c.mem[cpu::PC_START] = 0xf0;
//    c.mem[cpu::PC_START + 1] = 0x0a; 
    c.execute_insn();

     loop {
        c.execute_insn();
        if (c.mem[c.pc] == 0x0) && (c.mem[c.pc + 1] == 0xfd ) {
            break;
        }
    }
 
    loop {

    }
    
    
}
