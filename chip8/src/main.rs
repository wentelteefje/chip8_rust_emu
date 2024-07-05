extern crate piston_window;
use piston_window::*;

use chip8_core::emu::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    
    // chip8.load_rom("roms/IBM_Logo.ch8");
    // chip8.load_rom("roms/1-chip8-logo.ch8");
    // chip8.load_rom("roms/3-corax+.ch8");
    chip8.load_rom("roms/4-flags.ch8");
    // chip8.load_rom("roms/5-quirks.ch8");

    let mut window: PistonWindow = WindowSettings::new("CHIP-8 Emulator", [640, 320])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new().ups(500)); // Update at 500 Hz
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.press_args() {
            if let Button::Keyboard(key) = args {
                chip8.key_mut().handle_key_press(key);
            }
        }

        if let Some(args) = e.release_args() {
            if let Button::Keyboard(key) = args {
                chip8.key_mut().handle_key_release(key);
            }
        }

        if let Some(_) = e.update_args() {
            chip8.emulate_cycle(); // Execute one cycle of the emulator
        }

        if let Some(_) = e.render_args() {
            chip8.display_mut().draw_graphics(&mut window, &e); // Draw the current state of the display
        }
    }
}