use crate::hardware::memory::Memory;

#[derive(Clone)]
pub struct Chip8 {
    registers: Registers,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            registers: Registers::new(),
        }
    }

    pub fn run(&mut self, memory: &mut Memory) {
        //get random between 512 and 4095
        //let random_address = rand::random::<u16>() % 3583 + 512;
        //let random_value = rand::random::<u8>();
        //memory.write(random_address, random_value);
        // print!(".");
        // let op_code: u16 = memory.read(self.registers.pc) as u;
        // self.registers.increment_pc();
        // println!("op_code: {:04X}", op_code);
        let op_code: u16 = self.get_op_code(memory);
        self.registers.increment_pc();
        self.execute_op_code(op_code, memory);
    }

    fn get_op_code(&self, memory: &Memory) -> u16 {
        // read 2 bytes from memory and combine them into a single u16
        // opcodes are 2 bytes long
        // shift the first byte 8 bits to the left and combine it with the second byte
        // useful for bitwise operations refresher https://www.youtube.com/watch?v=6h89XQaGonE
        // not sure about endianness, perhaps I am overthinking but hexdump shows the bytes in reverse order
        //(memory.read(self.registers.pc+1) as u16) << 8 | memory.read(self.registers.pc) as u16
        // I was overthinking, the bytes are in the correct order
        (memory.read(self.registers.pc) as u16) << 8 | memory.read(self.registers.pc+1) as u16
    }

    fn execute_op_code(&mut self, op_code: u16, memory: &mut Memory) {
        // skip copilot
        //println!("op_code: {:04X} binary: {:#018b}", &op_code, &op_code);
        match op_code & 0xF000 {
            // bitwise AND on the first 4 bits of the op_code 1111 0000 0000 0000
            0x0000 => { // first 4 bits are 0
                match op_code & 0x00FF { // bitwise AND on the last 8 bits of the op_code 0000 0000 1111 1111
                    0x00E0 => { // 0000 0000 1110 0000
                        println!("CLS");
                    },
                    0x00EE => { // 0000 0000 1110 1110
                        println!("RET");
                    },
                    _ => {
                        println!("SYS addr: {:014b}", op_code & 0x0FFF); 
                    },
                }
            },
            0x1000 => {
                // println!("JP addr: {:014b}", op_code & 0x0FFF);
                // println!("JP addr: {:04X}, ", op_code & 0x0FFF);
                // 1nnn - JP addr -- Jump to location nnn.
                self.registers.pc = op_code & 0x0FFF;
            },
            0x2000 => {
                println!("CALL addr: {:014b}", op_code & 0x0FFF);
                println!("CALL addr: {:04X}, ", op_code & 0x0FFF);
                // 2nnn - CALL addr -- Call subroutine at nnn.
                self.registers.stack[self.registers.sp as usize] = self.registers.pc;
                self.registers.sp += 1;
                self.registers.pc = op_code & 0x0FFF;
            },
            0x3000 => {
                // 3xkk - SE Vx, byte -- Skip next instruction if Vx = kk.
                let x = ((op_code & 0x0F00) >> 8) as u8;
                let kk = (op_code & 0x00FF) as u8;
                if self.registers.v[x as usize] == kk {
                    self.registers.increment_pc();
                }
            },
            0x4000 => {
                // 4xkk - SNE Vx, byte -- Skip next instruction if Vx != kk.
                let x = ((op_code & 0x0F00) >> 8) as u8;
                let kk = (op_code & 0x00FF) as u8;
                if self.registers.v[x as usize] != kk {
                    self.registers.increment_pc();
                }
            },
            0x5000 => {
                // 5xy0 - SE Vx, Vy -- Skip next instruction if Vx = Vy.
                let x = ((op_code & 0x0F00) >> 8) as u8;
                let y = ((op_code & 0x00F0) >> 4) as u8;
                if self.registers.v[x as usize] == self.registers.v[y as usize] {
                    self.registers.increment_pc();
                }
            },
            0x6000 => {
                // 6xkk - LD Vx, byte -- Set Vx = kk.
                let x = ((op_code & 0x0F00) >> 8) as u8;
                let kk = (op_code & 0x00FF) as u8;
                self.registers.v[x as usize] = kk;
            },
            0x7000 => {
                // 7xkk - ADD Vx, byte -- Set Vx = Vx + kk.
                let x = ((op_code & 0x0F00) >> 8) as u8;
                let kk = (op_code & 0x00FF) as u8;
                self.registers.v[x as usize] += kk;
            },
            0x8000 => {
                let x = ((op_code & 0x0F00) >> 8) as u8;
                let y = ((op_code & 0x00F0) >> 4) as u8;
                match op_code & 0x000F {
                    0x0000 => {
                        //8xy0 - LD Vx, Vy -- Set Vx = Vy.
                        self.registers.v[x as usize] = self.registers.v[y as usize];
                    },
                    0x0001 => {
                        // 8xy1 - OR Vx, Vy -- Set Vx = Vx OR Vy.
                        self.registers.v[x as usize] |= self.registers.v[y as usize];
                    },
                    0x0002 => {
                        // 8xy2 - AND Vx, Vy -- Set Vx = Vx AND Vy.
                        self.registers.v[x as usize] &= self.registers.v[y as usize];
                    },
                    0x0003 => {
                        // 8xy3 - XOR Vx, Vy -- Set Vx = Vx XOR Vy.
                        self.registers.v[x as usize] ^= self.registers.v[y as usize];
                    },
                    0x0004 => {
                        // 8xy4 - ADD Vx, Vy - Set Vx = Vx + Vy, set VF = carry.
                        // 0XFF is the maximum value for a byte aka 255
                        if self.registers.v[y as usize] > (0xFF - self.registers.v[x as usize]) {
                            self.registers.v[0xF] = 1;
                        } else {
                            self.registers.v[0xF] = 0;
                        }
                        self.registers.v[x as usize] += self.registers.v[y as usize];

                    },
                    0x0005 => {
                        // 8xy5 - SUB Vx, Vy - Set Vx = Vx - Vy, set VF = NOT borrow.
                        if self.registers.v[y as usize] > self.registers.v[x as usize] {
                            self.registers.v[0xF] = 0;
                        } else {
                            self.registers.v[0xF] = 1;
                        }
                        self.registers.v[x as usize] -= self.registers.v[y as usize];
                    },
                    0x0006 => {
                        // 8xy6 - SHR Vx {, Vy} - Set Vx = Vx SHR 1.
                        self.registers.v[0xF] = self.registers.v[x as usize] & 0x1; // get the least significant bit
                        self.registers.v[x as usize] >>= 1; // divide by 2 aka shift right by 1
                    },
                    0x0007 => {
                        // 8xy7 - SUBN Vx, Vy - Set Vx = Vy - Vx, set VF = NOT borrow.
                        if self.registers.v[y as usize] > self.registers.v[x as usize] { // If Vy > Vx, then VF is set to 1
                            self.registers.v[0xF] = 1;
                        } else {
                            self.registers.v[0xF] = 0;
                        }
                        self.registers.v[x as usize] = self.registers.v[y as usize] - self.registers.v[x as usize];
                    },
                    0x000E => {
                        // 8xyE - SHL Vx {, Vy} - Set Vx = Vx SHL 1.
                        self.registers.v[0xF] = self.registers.v[x as usize] >> 7; // get the most significant bit
                        self.registers.v[x as usize] <<= 1; // multiply by 2 aka shift left by 1
                    },
                    _ => {
                        println!("we should not see this: Unknown opcode: {:04X}", op_code);
                    },
                }
            },
            0x9000 => {
                // 9xy0 - SNE Vx, Vy -- Skip next instruction if Vx != Vy.
                let x = ((op_code & 0x0F00) >> 8) as u8;
                let y = ((op_code & 0x00F0) >> 4) as u8;
                if self.registers.v[x as usize] != self.registers.v[y as usize] {
                    self.registers.increment_pc();
                }
            },
            0xA000 => {
                //Annn - LD I, addr -- Set I = nnn.
                self.registers.i = op_code & 0x0FFF;
            },
            0xB000 => {
                //Bnnn - JP V0, addr -- Jump to location nnn + V0.
                self.registers.pc = (op_code & 0x0FFF) + self.registers.v[0] as u16;
            },
            0xC000 => {
                //Cxkk - RND Vx, byte -- Set Vx = random byte AND kk.
                let x = ((op_code & 0x0F00) >> 8) as u8;
                let kk = (op_code & 0x00FF) as u8;
                self.registers.v[x as usize] = rand::random::<u8>() & kk;
            },
            0xD000 => { // Drawing
                //println!("DRW Vx, Vy, nibble");
                //Dxyn - DRW Vx, Vy, nibble -- Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                let x = ((op_code & 0x0F00) >> 8) as u8;
                let y = ((op_code & 0x00F0) >> 4) as u8;
                let height = (op_code & 0x000F) as u8;
                let mut collision = false;
                let mut line: u8 = 0;
                // Todo: implement drawing
                while line <= height {
                    let pixels = memory.read(self.registers.i + line); // row of pixels
                    for bit in 0..8 {
                        if (pixels & (0x80 >> bit)) != 0 {
                            // this gets me the bit at the position - not sure what to do with it yet
                            // need to read more
                        }
                    }
                    line += 1;
                }
            },
            0xE000 => { // Keyboard input
                match op_code & 0x00FF {
                    0x009E => {
                        // Ex9E - SKP Vx -- Skip next instruction if key with the value of Vx is pressed.
                        // TODO: implement
                    },
                    0x00A1 => {
                        // ExA1 - SKNP Vx -- Skip next instruction if key with the value of Vx is not pressed.
                        // TODO: implement
                    },
                    _ => {
                        println!("Unknown opcode: {:04X}", op_code);
                    },
                }
            },
            0xF000 => {
                // all these op codes use Vx - getting the x value here means less repetition of bitwise AND
                // hopefully making it easier to read
                let x = ((op_code & 0x0F00) >> 8) as usize;
                match op_code & 0x00FF {
                    0x0007 => {
                        // Fx07 - LD Vx, DT -- Set Vx = delay timer value.
                        self.registers.v[x] = self.registers.delay_timer;
                    },
                    0x000A => {
                        // Fx0A - LD Vx, K -- Wait for a key press, store the value of the key in Vx.
                        // TODO: implement
                    },
                    0x0015 => {
                        // Fx15 - LD DT, Vx -- Set delay timer = Vx.
                        self.registers.delay_timer = self.registers.v[x];
                    },
                    0x0018 => {
                        // Fx18 - LD ST, Vx -- Set sound timer = Vx.
                        self.registers.sound_timer = self.registers.v[x];
                    },
                    0x001E => {
                        // Fx1E - ADD I, Vx -- Set I = I + Vx.
                        self.registers.i += self.registers.v[x] as u16;
                    },
                    0x0029 => {
                        // Fx29 - LD F, Vx -- Set I = location of sprite for digit Vx.
                        self.registers.i = self.registers.v[x] as u16 * 5; // each sprite is 5 bytes long and starts at 0x000
                    },
                    0x0033 => {
                        // Fx33 - LD B, Vx -- Store BCD representation of Vx in memory locations I, I+1, and I+2.
                        // TODO: implement
                    },
                    0x0055 => {
                        // Fx55 - LD [I], Vx -- Store registers V0 through Vx in memory starting at location I.
                        for i in 0..=x {
                            memory.write(self.registers.i + i as u16, self.registers.v[i]);
                        }
                    },
                    0x0065 => {
                        // Fx65 - LD Vx, [I] -- Read registers V0 through Vx from memory starting at location I.
                        for i in 0..=x {
                            self.registers.v[i] = memory.read(self.registers.i + i as u16);
                        }
                    },
                    _ => {
                        println!("Unknown opcode: {:04X}", op_code);
                    },
                }
            },
            _ => {
                println!("Unknown opcode: {:04X}", op_code);
            },
        }

    }

    pub fn reset(&mut self) {
        self.registers = Registers::new();
    }
}


#[derive(Clone)]
struct Registers {
    pub v: [u8; 16],
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            v: [0; 16],
            pc: 0x200,
            sp: 0,
            stack: [0; 16], // 16 levels of stack in (0xEA0-0xEAF) in (0xEA0-0xEFF) 
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }
}