
#![allow(dead_code)]

mod cpu;
mod screen;

extern crate rand;
extern crate sdl2;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

fn main() {
    println!("Hello, world!");
    let mut s = screen::Screen::new(64, 32, 6);

    s.draw_pixel(32, 16, screen::PIXEL_COLORS[1]); 

    loop {

    }
    
    
}
