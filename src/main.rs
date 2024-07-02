// The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return
// to when finished with a subroutine. Chip-8 allows for up to 16 levels of nested subroutines.
#![allow(dead_code)]
extern crate piston_window;
use piston_window::*;

struct Chip8 {
    registers: [u8; 16], // V0 to VF
    stack: [u8; 32],
    memory: [u8; 4096],
    display: [[u8; 32]; 64],
    pc: u16,
    sp: u8,
    i: u16,
}

impl Chip8 {
    fn new() -> Self {
        Chip8 {
            registers: [0u8; 16],
            stack: [0u8; 32],
            memory: [0u8; 4096],
            display: [[0u8; 32]; 64],
            pc: 0x200,
            sp: 0x00,
            i: 0x0000,
        }
    }

    pub fn load_rom(&mut self, filename: &str) {
        let rom_bytes = std::fs::read(filename).expect("Failed to read ROM file");
        let start_address = 0x200;
        for (i, &byte) in rom_bytes.iter().enumerate() {
            self.memory[start_address + i] = byte;
        }
    }

    fn emulate_cycle(&mut self) {
        // Fetch the instruction from memory at the current pc, increment pc
        let v = u16::from_be_bytes([
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
        ]);
        self.pc += 2;

        // Decode and Execute the instruction
        // In general CPUs the first byte of the instruction is what is known as the OPCODE
        // so maybe at some point this match statement should be structured according to this structure
        // println!("Instruction: {}", v);
        match v {
            v if (v & 0xF00F) == 0x8004 => {
                // 8XY4
                let x = ((v & 0x0F00) >> 8) as u8;
                let y = ((v & 0x00F0) >> 4) as u8;
                self.op_add_vy_vx(x as usize, y as usize);
            }
            v if (v & 0xF000) == 0x1000 => {
                // 1NNN
                let nnn: u16 = v & 0x0FFF;
                self.op_jump(nnn);
            }
            v if (v & 0xF000) == 0x6000 => {
                // 6XNN
                let x = ((v & 0x0F00) >> 8) as u8;
                let nn: u8 = (v & 0x00FF) as u8;
                self.op_set_vx_nn(x as usize, nn);
            }
            v if (v & 0xF000) == 0x7000 => {
                // 7XNN
                let x = ((v & 0x0F00) >> 8) as u8;
                let nn: u8 = (v & 0x00FF) as u8;
                self.op_add_vx_nn(x as usize, nn);
            }
            v if (v & 0xF000) == 0xA000 => {
                // ANNN
                let nnn: u16 = v & 0x0FFF;
                self.op_i_nnn(nnn);
            }
            v if (v & 0xF000) == 0xD000 => {
                // DXYN
                let vx = ((v & 0x0F00) >> 8) as u8;
                let vy = ((v & 0x00F0) >> 4) as u8;
                let n: u8 = (v & 0x000F) as u8;
                self.op_draw_vx_vy_n(vx, vy, n);
            }
            v if (v & 0xF000) == 0x4000 => {
                // 4XNN
                let x = ((v & 0x0F00) >> 8) as u8;
                let nn: u8 = (v & 0x00FF) as u8;
                self.op_skip_next(x.into(), nn);
            }
            _ => println!("OPCODE {} not implemented!", v),
        }
    }

    fn draw_graphics(&self, window: &mut PistonWindow, e: &Event) {
        window.draw_2d(e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g); // Clear the screen to black
            for x in 0..64 {
                for y in 0..32 {
                    if self.display[x][y] != 0 {
                        rectangle(
                            [1.0, 1.0, 1.0, 1.0],                           // White color
                            [x as f64 * 10.0, y as f64 * 10.0, 10.0, 10.0], // Scale each pixel to 10x10
                            c.transform,
                            g,
                        );
                    }
                }
            }
        });
    }
}

impl Chip8 {
    // 00E0 - Clear screen
    fn op_cls(&mut self) {
        self.display = [[0u8; 32]; 64];
    }

    // 1NNN - Jumps to address nnn
    fn op_jump(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    // 4xkk - SNE Vx, byte
    // Skip next instruction if Vx != kk.
    // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn op_skip_next(&mut self, x: usize, nn: u8) {
        if self.registers[x] != nn {
            self.pc += 4;
        }
    }

    // 6XNN	- Sets Vx to nn.
    fn op_set_vx_nn(&mut self, x: usize, nn: u8) {
        self.registers[x] = nn;
    }

    // 7XNN - ADD Vx, byte
    // Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn op_add_vx_nn(&mut self, x: usize, nn: u8) {
        self.registers[x] += nn;
    }

    // 8XY4 - Set Vx = Vx + Vy, set VF = carry.
    // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,)
    // VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn op_add_vy_vx(&mut self, x: usize, y: usize) {
        let result: u16 = (self.registers[x] + self.registers[y]).into();
        if result > 255 {
            self.op_set_vx_nn(x, 255u8);
            self.op_set_vx_nn(0x0F, 0x01);
        } else {
            self.op_set_vx_nn(x, result.try_into().unwrap());
            self.op_set_vx_nn(0x0F, 0x00);
        }
    }

    // Annn - LD I, addr
    // Set I = nnn.
    // The value of register I is set to nnn.
    fn op_i_nnn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    // DXYN
    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    fn op_draw_vx_vy_n(&mut self, vx: u8, vy: u8, n: u8) {
        self.registers[0xF] = 0; // Reset VF before starting the drawing
        for byte_index in 0..n {
            let byte = self.memory[(self.i + byte_index as u16) as usize];
            for bit_index in 0..8 {
                let pixel_bit = byte & (0x80 >> bit_index);
                let vx_wrapped = ((self.registers[vx as usize] as usize + bit_index) % 64) as usize;
                let vy_wrapped = ((self.registers[vy as usize] as usize + byte_index as usize) % 32) as usize;
                let current_pixel = self.display[vx_wrapped][vy_wrapped];
                let new_pixel = (pixel_bit > 0) as u8 ^ current_pixel;
                if current_pixel == 1 && new_pixel == 0 {
                    self.registers[0xF] = 1;
                }
                self.display[vx_wrapped][vy_wrapped] = new_pixel;
            }
        }
    }
}

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_rom("IBM_Logo.ch8");
    // chip8.load_rom("1-chip8-logo.ch8");

    let mut window: PistonWindow = WindowSettings::new("CHIP-8 Emulator", [640, 320])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new().ups(500)); // Update at 500 Hz
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.update_args() {
            chip8.emulate_cycle(); // Execute one cycle of the emulator
        }

        if let Some(_) = e.render_args() {
            chip8.draw_graphics(&mut window, &e); // Draw the current state of the display
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_addition_opcode() {
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0x60; // 60 00 - the opcode for setting V0
        emulator.memory[0x201] = 0x00;
        emulator.memory[0x202] = 0x61; // 61 03 - the opcode for setting V1
        emulator.memory[0x203] = 0x03;
        emulator.memory[0x204] = 0x80; // 80 14 - the opcode for adding V1 to V0
        emulator.memory[0x205] = 0x14;
        // emulator.pc = 0x200;

        emulator.emulate_cycle();
        assert_eq!(emulator.registers[0], 0x00); // Check V0 after setting it to 0

        emulator.emulate_cycle();
        assert_eq!(emulator.registers[1], 0x03); // Check V1 after setting it to 3

        emulator.emulate_cycle();
        assert_eq!(emulator.registers[0], 0x03); // Check V0 after addition
    }

    #[test]
    fn test_add_nn_to_vx_opcode() {
        let mut emulator = Chip8::new();

        // Initialize V3 with a value, for example 0x05
        emulator.registers[3] = 0x05;

        // Program starts at 0x200, load our opcode there:
        // Opcode 7312: Add 0x12 to V3
        emulator.memory[0x200] = 0x73;
        emulator.memory[0x201] = 0x12;

        // Set program counter to start of program
        emulator.pc = 0x200;

        // Execute the opcode
        emulator.emulate_cycle();

        // Check the result, expecting V3 to now be 0x05 + 0x12 = 0x17
        assert_eq!(
            emulator.registers[3], 0x17,
            "V3 should be incremented by 0x12"
        );
    }

    #[test]
    fn test_set_index_register_opcode() {
        let mut emulator = Chip8::new();

        // Opcode AABC: Set I to 0xABC
        emulator.memory[0x200] = 0xAA;
        emulator.memory[0x201] = 0xBC;

        // Set program counter to start of program
        emulator.pc = 0x200;

        // Execute the opcode
        emulator.emulate_cycle();

        // Check the result, expecting I to be set to 0xABC
        assert_eq!(emulator.i, 0xABC, "Index register I should be set to 0xABC");
    }
}
