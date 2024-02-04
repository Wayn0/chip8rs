use super::{cpus::chip8::Chip8, cartridge::Cartridge, clock::Clock, display, keyboard, memory::Memory};

pub struct Computer {
    pub clock: Clock,
    pub cpu: Chip8,
    pub memory: Memory,
    pub display: display::Display,
    pub keyboard: keyboard::Keyboard,
    pub cartridge: Cartridge,
    pub last_cycle: u128,
} 

impl Computer {
    pub fn new(cartridge: Cartridge) -> Self {
        Computer {
            clock: Clock::new(),
            cpu: Chip8::new(),
            memory: Memory::new(),
            display: display::Display::new(),
            keyboard: keyboard::Keyboard::new(),
            cartridge,
            last_cycle: 0,
        }
    }
    
    pub fn power_on(&mut self) {
        println!("Powering on... ");
        self.memory.reset();
        self.memory.load_rom(self.cartridge.rom.clone());

        self.cpu.run(&mut self.memory);
        self.clock.first_run();
    }

    pub fn run(&mut self) {
        self.clock.run();
        //if self.clock.cycles != self.last_cycle {
        self.cpu.run(&mut self.memory);
        //    self.last_cycle = self.clock.cycles;
        //}
    }
}
