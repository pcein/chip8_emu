// screen.rs

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Default screen height in pixels
const SCREEN_HEIGHT:u16 = 32;

/// Default screen width in pixels
const SCREEN_WIDTH:u16 = 64;

/// Screen depth in pixels
const SCREEN_DEPTH:u8 = 8;

/// CHIP-8 uses only two colors: 0 for OFF and
/// 1 for ON.
lazy_static! {
    pub static ref PIXEL_COLORS:[Color; 2] = 
        [Color::RGBA(0, 0, 0, 255), 
         Color::RGBA(250, 250, 250, 255)];
}

pub struct Screen {
    width: u32,
    height: u32,
    scale_factor: u32,
    canvas: Canvas<Window>,
    events: EventPump,
    /// `mem' is a representation of the display within the
    /// virtual machine. If mem[i] is 1, the corresponding 
    /// pixel on the real screen is ON, otherwise OFF.
    ///  
    /// Had to use this because rust-sdl2 
    /// does not seem to provide an easy way to get the
    /// color of a pixel.
    mem: [u8; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
}

impl Screen {
    pub fn new(width: u32, height: u32, scale_factor: u32) -> Screen {
        let ctxt = sdl2::init().expect("SDL2 library initialization failed.");
        let video = ctxt.video().expect("Unable to get video subsystem.");
        let window = 
            video.window("CHIP-8 Demo!", width * scale_factor, height * scale_factor)
            .position_centered()
            .opengl()
            .build()
            .expect("Unable to get Window");
        
        let mut canvas = window.into_canvas().build().expect("Unable to get canvas");

        canvas.set_draw_color(PIXEL_COLORS[0]);
        canvas.clear();
        canvas.present();
        
        let events = ctxt.event_pump().expect("Unable to get event pump"); 

        Screen{ 
            width: width, height: height,
            scale_factor: scale_factor,
            canvas: canvas, events: events,
            mem: [0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color:Color) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(
            Rect::new(
                x as i32 * self.scale_factor as i32, 
                y as i32 * self.scale_factor as i32,
                self.scale_factor, self.scale_factor)).expect("Error in draw_point");
        self.mem[(y * u32::from(SCREEN_WIDTH) + x) as usize] = 1;
        self.canvas.present();
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> u8 {
        self.mem[(y * u32::from(SCREEN_WIDTH) + x) as usize]
    }
 }
