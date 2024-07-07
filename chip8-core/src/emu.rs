#![allow(dead_code)]
extern crate piston_window;
use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::memory::Memory;

pub struct Chip8 {
    registers: [u8; 16], // V0 to VF
    stack: Vec<u16>,
    memory: Memory,
    display: Display,
    pc: u16,
    sp: u8,
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    key: Keyboard,
}

// enum Opcode {
//     OP_1NNN(u16),
//     OP_4XNN(u8, u8),
//     OP_6XNN(u8, u8),
//     OP_7XNN(u8, u8),
//     OP_8XY4(u8, u8),
//     OP_ANNN(u16),
//     OP_DXYN(u8, u8, u8),
//     UNKNOWN,
// }

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            registers: [0u8; 16],
            stack: vec![0; 16],
            memory: Memory::new(),
            display: Display::new(),
            pc: 0x200,
            sp: 0x00,
            i: 0x0000,
            delay_timer: 0x00,
            sound_timer: 0x00,
            key: Keyboard::new(),
        }
    }

    pub fn key_mut(&mut self) -> &mut Keyboard {
        &mut self.key
    }

    pub fn display_mut(&mut self) -> &mut Display {
        &mut self.display
    }

    pub fn load_rom(&mut self, filename: &str) {
        let rom_bytes = std::fs::read(filename).expect("Failed to read ROM file");
        self.memory.write_slice_at(0x200, &rom_bytes);
    }

    pub fn delay_timer_tick(&mut self) {
        if self.delay_timer != 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch the instruction from memory at the current pc
        let v = u16::from_be_bytes([
            self.memory.get_byte(self.pc as usize),
            self.memory.get_byte((self.pc + 1) as usize),
        ]);
        // Increment pc
        self.pc += 2;

        // Decode and Execute the instruction
        match v & 0xF000 {
            0x0000 => match v & 0x000F {
                0x0000 => {
                    self.op_00e0();
                }
                0x000E => {
                    self.op_00ee();
                }
                _ => println!("OPCODE {} not implemented!", v),
            },
            0x1000 => {
                // 1NNN
                let nnn: u16 = v & 0x0FFF;
                self.op_1nnn(nnn);
            }
            0x2000 => {
                // 2NNN
                let nnn: u16 = (v & 0x0FFF) as u16;
                self.op_2nnn(nnn);
            }
            0x3000 => {
                // 3XNN
                let x = ((v & 0x0F00) >> 8) as u8;
                let nn: u8 = (v & 0x00FF) as u8;
                self.op_3xnn(x, nn);
            }
            0x4000 => {
                // 4XNN
                let x = ((v & 0x0F00) >> 8) as u8;
                let nn: u8 = (v & 0x00FF) as u8;
                self.op_4xnn(x, nn);
            }
            0x5000 => {
                // 5XY0
                let x = ((v & 0x0F00) >> 8) as u8;
                let y = ((v & 0x00F0) >> 4) as u8;
                self.op_5xy0(x, y);
            }
            0x6000 => {
                // 6XNN
                let x = ((v & 0x0F00) >> 8) as u8;
                let nn: u8 = (v & 0x00FF) as u8;
                self.op_6xnn(x as usize, nn);
            }
            0x7000 => {
                // 7XNN
                let x = ((v & 0x0F00) >> 8) as u8;
                let nn: u8 = (v & 0x00FF) as u8;
                self.op_7xnn(x, nn);
            }
            0x8000 => {
                let x = ((v & 0x0F00) >> 8) as u8;
                let y = ((v & 0x00F0) >> 4) as u8;
                match v & 0x000F {
                    0x0000 => {
                        self.op_8xy0(x, y);
                    }
                    0x0001 => {
                        self.op_8xy1(x, y);
                    }
                    0x0002 => {
                        self.op_8xy2(x, y);
                    }
                    0x0003 => {
                        self.op_8xy3(x, y);
                    }
                    0x0004 => {
                        self.op_8xy4(x, y);
                    }
                    0x0005 => {
                        self.op_8xy5(x, y);
                    }
                    0x0006 => {
                        self.op_8xy6(x, y);
                    }
                    0x0007 => {
                        self.op_8xy7(x, y);
                    }
                    0x000E => {
                        self.op_8xye(x, y);
                    }
                    _ => println!("OPCODE {} not implemented!", v),
                }
            }
            0x9000 => {
                // 9xy0
                let x = ((v & 0x0F00) >> 8) as u8;
                let y = ((v & 0x00F0) >> 4) as u8;
                self.op_9xy0(x, y);
            }
            0xA000 => {
                // ANNN
                let nnn: u16 = v & 0x0FFF;
                self.op_annn(nnn);
            }
            0xB000 => {
                // BNNN - TODO
            }
            0xC000 => {
                // CXNN - TODO
            }
            0xD000 => {
                // DXYN
                let vx = ((v & 0x0F00) >> 8) as u8;
                let vy = ((v & 0x00F0) >> 4) as u8;
                let n: u8 = (v & 0x000F) as u8;
                self.op_dxyn(vx, vy, n);
            }
            0xE000 => {
                let x = ((v & 0x0F00) >> 8) as u8;
                match v & 0x00FF {
                    0x009E => {
                        self.op_ex9e(x);
                    }
                    0x00A1 => {
                        self.op_exa1(x);
                    }
                    _ => println!("OPCODE {} not implemented!", v),
                }
            }
            0xF000 => {
                let x = ((v & 0x0F00) >> 8) as u8;
                match v & 0x00FF {
                    0x0007 => {
                        self.op_fx07(x);
                    }
                    0x000A => {
                        self.op_fx0a(x);
                    }
                    0x0015 => {
                        self.op_fx15(x);
                    }
                    0x0018 => {
                        // self.op_fx18(x);
                    }
                    0x001E => {
                        self.op_fx1e(x);
                    }
                    0x0029 => {
                        // self.op_fx29(x);
                    }
                    0x0033 => {
                        self.op_fx33(x);
                    }
                    0x0055 => {
                        self.op_fx55(x);
                    }
                    0x0065 => {
                        self.op_fx65(x);
                    }

                    _ => println!("OPCODE {} not implemented!", v),
                }
            }
            _ => println!("OPCODE {} not implemented!", v),
        }
    }
}

// Implement Opcodes
impl Chip8 {
    /// 00E0 - CLS
    fn op_00e0(&mut self) {
        self.display = Display::new();
    }

    /// 00EE - RET
    fn op_00ee(&mut self) {
        let top_stack_address = self.stack.pop().unwrap_or_default();
        self.pc = top_stack_address;
        // self.sp -= 1;
    }

    /// 1NNN - JP addr
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    // 2NNN - CALL addr
    fn op_2nnn(&mut self, nnn: u16) {
        self.stack.push(self.pc);
        self.pc = nnn;
    }

    // 3XNN - SE Vx, byte
    fn op_3xnn(&mut self, x: u8, nn: u8) {
        if self.registers[x as usize] == nn {
            self.pc += 2;
        }
    }

    // 4XNN - SNE Vx, byte
    fn op_4xnn(&mut self, x: u8, nn: u8) {
        if self.registers[x as usize] != nn {
            self.pc += 2;
        }
    }

    // 5XY0 - SE Vx, Vy
    fn op_5xy0(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.pc += 2;
        }
    }

    // 6XNN	- LD Vx, byte
    fn op_6xnn(&mut self, x: usize, nn: u8) {
        self.registers[x] = nn;
    }

    // 7XNN - ADD Vx, byte
    fn op_7xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = self.registers[x as usize].wrapping_add(nn);
    }

    // 8XY0 - LD Vx, Vy
    fn op_8xy0(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    // 8XY1 - OR Vx, Vy
    fn op_8xy1(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
    }

    // 8XY2 - AND Vx, Vy
    fn op_8xy2(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
    }

    // 8XY3 - XOR Vx, Vy
    fn op_8xy3(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
    }

    // 8XY4 - ADD Vx, Vy
    fn op_8xy4(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] =
            self.registers[x as usize].wrapping_add(self.registers[y as usize]);
        if (vx as u16 + vy as u16) > 255 {
            self.registers[0x0F] = 0x01;
        } else {
            self.registers[0x0F] = 0x00;
        }
    }

    // 8xy5 - SUB Vx, Vy
    fn op_8xy5(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] =
            self.registers[x as usize].wrapping_sub(self.registers[y as usize]);
        if vx >= vy {
            self.registers[0x0F] = 0x01;
        } else {
            self.registers[0x0F] = 0x00;
        }
    }

    // 8XY6 - SHR Vx {, Vy}
    fn op_8xy6(&mut self, x: u8, y: u8) {
        // Set vX to vY
        self.registers[x as usize] = self.registers[y as usize];
        let shifted_bit: u8 = self.registers[x as usize] & 0x01;
        // Shift right by one
        self.registers[x as usize] = self.registers[x as usize] >> 1;
        // Store shifted bit in vF
        self.registers[0x0F] = shifted_bit;
    }

    // 8XY7 - SUBN Vx, Vy
    fn op_8xy7(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] =
            self.registers[y as usize].wrapping_sub(self.registers[x as usize]);
        if vy >= vx {
            self.registers[0x0F] = 0x01;
        } else {
            self.registers[0x0F] = 0x00;
        }
    }

    // 8XYE - SHL Vx {, Vy}
    fn op_8xye(&mut self, x: u8, y: u8) {
        // Set vX to vY
        let shifted_bit: u8 = (self.registers[y as usize] & 0x80) >> 7;
        self.registers[y as usize] = self.registers[y as usize] << 1;
        self.registers[x as usize] = self.registers[y as usize];
        // Shift right by one
        // Store shifted bit in vF
        self.registers[0x0F] = shifted_bit;
    }

    // 9XY0 - SNE Vx, Vy
    fn op_9xy0(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] != self.registers[y as usize] {
            self.pc += 2;
        }
    }

    // Fx07 - LD Vx, DT
    fn op_fx07(&mut self, x: u8) {
        self.registers[x as usize] = self.delay_timer;
    }

    // Fx15 - LD DT, Vx
    fn op_fx15(&mut self, x: u8) {
        self.delay_timer = self.registers[x as usize];
    }

    // FX65 - LD Vx, [I]
    fn op_fx65(&mut self, x: u8) {
        let mem_bytes = self.memory.read_slice_at(self.i as usize, x as usize);
        self.registers[0..=(x as usize)].copy_from_slice(mem_bytes);
        self.i += (x + 1) as u16;
    }

    // FX55 - LD [I], Vx
    fn op_fx55(&mut self, x: u8) {
        self.memory
            .write_slice_at(self.i as usize, &self.registers[..=(x as usize)]);
    }

    // FX33 - LD B, Vx
    fn op_fx33(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        let hundreds = vx / 100;
        let tens = (vx % 100) / 10;
        let ones = vx % 10;

        self.memory
            .write_slice_at(self.i as usize, &[hundreds, tens, ones]);
    }

    // Fx1E
    fn op_fx1e(&mut self, x: u8) {
        self.i += self.registers[x as usize] as u16;
    }

    // FX0A - LD Vx, K
    fn op_fx0a(&mut self, x: u8) {
        self.key_mut().set_waiting_for_key_release();
        match self.key_mut().get_key_register() {
            Some(key) => self.registers[x as usize] = key as u8,
            None => self.pc -= 2,
        }
    }

    // Ex9E - SKP Vx
    fn op_ex9e(&mut self, x: u8) {
        let key = self.registers[x as usize];
        if self.key_mut().key_is_pressed(key) {
            self.pc += 2;
        }
    }

    // EXA1 - SKNP Vx
    fn op_exa1(&mut self, x: u8) {
        let key = self.registers[x as usize];
        if !self.key_mut().key_is_pressed(key) {
            self.pc += 2;
        }
    }

    // ANNN - LD I, addr
    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    // DXYN - DRW Vx, Vy, nibble
    fn op_dxyn(&mut self, vx: u8, vy: u8, n: u8) {
        self.registers[0xF] = 0; // Reset VF before starting the drawing
        for byte_index in 0..n {
            let byte = self.memory.get_byte((self.i + byte_index as u16) as usize);
            for bit_index in 0..8 {
                let pixel_bit = byte & (0x80 >> bit_index);
                let vx_wrapped = ((self.registers[vx as usize] as usize + bit_index) % 64) as usize;
                let vy_wrapped =
                    ((self.registers[vy as usize] as usize + byte_index as usize) % 32) as usize;
                let current_pixel = self.display.get_pixel(vx_wrapped, vy_wrapped);
                let new_pixel = (pixel_bit > 0) as u8 ^ current_pixel;
                if current_pixel == 1 && new_pixel == 0 {
                    self.registers[0xF] = 1;
                }
                self.display.update_pixel(vx_wrapped, vy_wrapped, new_pixel);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_addition_opcode() {
        let mut emulator = Chip8::new();
        emulator.memory.write_slice_at(0x200, &[0x60]);
        emulator.memory.write_slice_at(0x201, &[0x00]);
        emulator.memory.write_slice_at(0x202, &[0x61]);
        emulator.memory.write_slice_at(0x203, &[0x03]);
        emulator.memory.write_slice_at(0x204, &[0x80]);
        emulator.memory.write_slice_at(0x205, &[0x14]);
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

        emulator.memory.write_slice_at(0x200, &[0x73]);
        emulator.memory.write_slice_at(0x201, &[0x12]);
        // emulator.memory[0x200] = 0x73;
        // emulator.memory[0x201] = 0x12;

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
        emulator.memory.write_slice_at(0x200, &[0xAA]);
        emulator.memory.write_slice_at(0x201, &[0xBC]);
        // emulator.memory[0x200] = 0xAA;
        // emulator.memory[0x201] = 0xBC;

        // Set program counter to start of program
        emulator.pc = 0x200;

        // Execute the opcode
        emulator.emulate_cycle();

        // Check the result, expecting I to be set to 0xABC
        assert_eq!(emulator.i, 0xABC, "Index register I should be set to 0xABC");
    }

    #[test]
    fn test_8xye_with_vf_flag_with_vf_as_input() {
        let mut emulator = Chip8::new();

        // Test round 1: 8XYE

        // set vX to vY and shift vX one bit to the left, set vF to the bit shifted out, even if X=F
        // fn op_8xye(&mut self, x: u8, y: u8) {
        // Init registers
        emulator.registers[0x0F] = 188;

        // Program starts at 0x200, load our opcode there:
        // Opcode 8FFE
        emulator.memory.write_slice_at(0x200, &[0x8F]);
        emulator.memory.write_slice_at(0x201, &[0xFE]);
        // emulator.memory[0x200] = 0x8F;
        // emulator.memory[0x201] = 0xFE;

        // Set program counter to start of program
        emulator.pc = 0x200;

        // Execute the opcode
        emulator.emulate_cycle();

        // Check the result, expecting V3 to now be 0x05 + 0x12 = 0x17
        assert_eq!(emulator.registers[0x0F], 0x01, "vF should be equal to 0x01");
    }
}
