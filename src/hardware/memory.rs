/// Memory
/// From: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap
/// The Chip-8 language is capable of accessing up to 4KB (4,096 bytes) of RAM, from location 0x000 (0) to 0xFFF (4095). The first 512 bytes, from 0x000 to 0x1FF, are where the original interpreter was located, and should not be used by programs.
/// Most Chip-8 programs start at location 0x200 (512), but some begin at 0x600 (1536). Programs beginning at 0x600 are intended for the ETI 660 computer.
/// The uppermost 256 bytes (0xF00-0xFFF) are reserved for display refresh, and the 96 bytes below that (0xEA0-0xEFF) are reserved for call stack, internal use, and other variables.

const MEMORY_SIZE: usize = 0x1000; // 4096 bytes or 4kb
const RESERVED_MEMORY_SIZE: usize = 0x200; // 512 bytes or 0.5kb

#[derive(Clone, Debug)]
pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

struct ROM {
    rom: [u8; RESERVED_MEMORY_SIZE],
}

struct RAM {
    ram: [u8; MEMORY_SIZE - RESERVED_MEMORY_SIZE],
}

impl ROM {
    pub fn new() -> Self {
        let mut rom = [0; RESERVED_MEMORY_SIZE];
        let fonts = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, //zero
            0x20, 0x60, 0x20, 0x20, 0x70, //one
            0xF0, 0x10, 0xF0, 0x80, 0xF0, //two
            0xF0, 0x10, 0xF0, 0x10, 0xF0, //three
            0xF0, 0x10, 0xF0, 0x10, 0xF0, //four
            0xF0, 0x80, 0xF0, 0x10, 0xF0, //five
            0xF0, 0x80, 0xF0, 0x90, 0xF0, //six
            0xF0, 0x10, 0x20, 0x40, 0x40, //seven
            0xF0, 0x90, 0xF0, 0x90, 0xF0, //eight
            0xF0, 0x90, 0xF0, 0x10, 0xF0, //nine
            0xF0, 0x90, 0xF0, 0x90, 0x90, //A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
            0xF0, 0x80, 0x80, 0x80, 0xF0, //C
            0xE0, 0x90, 0x90, 0x90, 0xE0, //D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
            0xF0, 0x80, 0xF0, 0x80, 0x80, //F
        ];
        rom[..fonts.len()].copy_from_slice(&fonts);
        Self {
            rom,
        }
    }
}

impl RAM {
    pub fn new() -> Self {
        Self {
            ram: [0; MEMORY_SIZE - RESERVED_MEMORY_SIZE],
        }
    }
}

impl Memory {
    pub fn new() -> Self {
            let mut memory = [0; MEMORY_SIZE];
            let rom = ROM::new().rom;
            let ram = RAM::new().ram;
    
            // Copy ROM contents into memory
            memory[..RESERVED_MEMORY_SIZE].copy_from_slice(&rom);
            // Copy RAM contents into memory
            memory[RESERVED_MEMORY_SIZE..].copy_from_slice(&ram);
    
            Self { memory }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < RESERVED_MEMORY_SIZE as u16 {
            panic!("Attempted to write to reserved memory address: {:04X}", address);
        }
        self.memory[address as usize] = value;
    }

    pub fn dump(&self, start: u16, end: u16) {
        for i in start..=end {
            if i % 16 == 0 {
                println!();
            }
            print!("{:02X} ", self.memory[i as usize]);
        }
        println!();
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (i, byte) in rom.iter().enumerate() {
            self.memory[i + RESERVED_MEMORY_SIZE] = *byte;
        }
        // panic!("ROM loaded into memory");
        // The above copies the data in little endian format, which is not what we want??? 
        //self.memory[RESERVED_MEMORY_SIZE..].copy_from_slice(&rom);
        // use a for loop to copy the data into memory
        // for (i, byte) in rom.iter().enumerate() {
        //     print!("{:?} {:02X} ", i, byte);
        // }
    }

    pub fn get_reserved_end(&self) -> u16 {
        RESERVED_MEMORY_SIZE as u16
    }

    pub fn reset(&mut self) {
        self.memory = Memory::new().memory;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_read_write() {
        let mut memory = Memory::new();
        memory.write(0x0000, 0xFF);
        assert_eq!(memory.read(0x0000), 0xFF);
    }

    #[test]
    #[should_panic]
    fn test_memory_write_reserved() {
        let mut memory = Memory::new();
        memory.write(0x0000, 0xFF);
        memory.write(0x01FF, 0xFF);
    }

    #[test]
    fn test_memory_get_reserved_end() {
        let memory = Memory::new();
        assert_eq!(memory.get_reserved_end(), 0x200);
    }

    #[test]
    fn test_memory_reset() {
        let mut memory = Memory::new();
        memory.write(0x0000, 0xFF);
        memory.reset();
        assert_eq!(memory.read(0x0000), 0x00);
    }
}
