pub const MEMORY_SIZE: usize = 4096;

pub struct Memory {
    bytes: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            bytes: [0; MEMORY_SIZE],
        }
    }

    pub fn get_byte(&mut self, pos: usize) -> u8 {
        self.bytes[pos]
    }

    pub fn set_byte(&mut self, pos: usize, value: u8) {
        self.bytes[pos] = value;
    }

    pub fn write_slice_at(&mut self, at: usize, data: &[u8]) {
        // Ensure the operation is safe
        assert!(at + data.len() <= MEMORY_SIZE);
        self.bytes[at..at + data.len()].copy_from_slice(&data);
    }

    pub fn read_slice_at(&mut self, at: usize, n: usize) -> &[u8] {
        // Ensure the operation is safe
        assert!(at + n <= MEMORY_SIZE);
        &self.bytes[at..=at + n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_slice_test() {
        let mut mem = Memory::new();

        // Get one byte after another
        for i in 0..12 {
            assert_eq!(mem.get_byte(1527 + i), 0);
        }
        // Get bytes all at once
        assert_eq!(mem.read_slice_at(1527, 1), [0; 1]);
    }
}
