use hardware::{cartridge, computer::Computer};
mod hardware;
extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Chip8 Window", 64 * 15, 32 * 15)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let file = std::fs::read("data/IBM_logo.ch8")
    .expect("Unable to read file");
    //let data = file.into_iter().map(|x| x as u8).collect::<Vec<u8>>();

    let cartridge = cartridge::Cartridge::new(file);
    let mut computer = Computer::new(cartridge);
    computer.power_on();
    let mut counter = 0;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    // Handle 'A' key press
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    // Handle 'Q' key press
                    break 'running;
                },
                // Handle other keys or events
                _ => {}
            }
        }
        computer.run();
        counter += 1;
        if counter > 20 {
            break;
        }

        // Your emulator's rendering and logic update code goes here
        // Example: just clear the screen with a different color
        //canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.clear();
        canvas.present();
    }
    computer.memory.dump(0x0000, 0x0FFF);

    Ok(())
}