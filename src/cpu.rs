// Based on: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

struct Registers {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8, // In an addition operation, VF is the carry flag, while in subtraction, it is the "no borrow" flag.
    i: u16, // Supposed width 12 bits
    pc: u16, // used to store the currently executing address
    sp: u8, // stack pointer is used to point to the topmost level of the stack.
}

// The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return
// to when finished with a subroutine. Chip-8 allows for up to 16 levels of nested subroutines.
struct Stack {
    values: [u16; 16],
}

