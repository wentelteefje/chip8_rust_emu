// The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return
// to when finished with a subroutine. Chip-8 allows for up to 16 levels of nested subroutines.
#![allow(dead_code)]

struct Chip8 {
    registers: [u8; 16], // V0 to VF
    stack: [u16; 16],
    memory: [u8; 4096],
    pc: u16,
    sp: u8,
}

impl Chip8 {
    fn new() -> Self {
        Chip8 {
            registers: [0u8; 16],
            stack: [0u16; 16],
            memory: [0u8; 4096],
            pc: 0x200,
            sp: 0x00,
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
            _ => println!("OPCODE not implemented!"),
        }
    }
}

struct Instruction(u16);

// A 12-bit value, the lowest 12 bits of the instruction
struct Addr(u16);
impl Addr {
    fn from_instruction(i: Instruction) -> Self {
        Addr(i.0 & 4095u16)
    }
}

impl Chip8 {
    // 1NNN - Jumps to address nnn
    fn op_jump(&mut self, nnn: u16) {
        self.pc = nnn;
    }
    // 6XNN	- Sets Vx to nn.
    fn op_set_vx_nn(&mut self, x: usize, nn: u8) {
        self.registers[x] = nn;
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
}

fn main() {
    println!("Hello, world!");
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
        emulator.pc = 0x200;

        emulator.emulate_cycle();
        assert_eq!(emulator.registers[0], 0x00); // Check V0 after setting it to 0

        emulator.emulate_cycle();
        assert_eq!(emulator.registers[1], 0x03); // Check V1 after setting it to 3

        emulator.emulate_cycle();
        assert_eq!(emulator.registers[0], 0x03); // Check V0 after addition
    }
}
