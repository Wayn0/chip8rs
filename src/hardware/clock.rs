
const CLOCK_SPEED: u16 = 500;
pub struct Clock {
    pub cycles: u128,
    seconds: u128,
    last_cycle_update: std::time::Instant,
    last_second_update: std::time::Instant,
    cycle_interval: std::time::Duration,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            seconds: 0,
            last_cycle_update: std::time::Instant::now(),
            last_second_update: std::time::Instant::now(),
            cycle_interval: std::time::Duration::from_millis(1000 / CLOCK_SPEED as u64),
        }
    }

    fn update(&mut self) {
        let now = std::time::Instant::now();
        if self.cycles == u128::MAX {
            panic!("cycles overflowed"); // highly unlikely :)
        }

        // if now.duration_since(self.last_cycle_update) >= self.cycle_interval {
        //     self.last_cycle_update = now;
        //     self.cycles += 1;
        // }
        self.cycles += 1;

        if now.duration_since(self.last_second_update) >= std::time::Duration::from_secs(1) {
            self.last_second_update = now;
            self.seconds += 1;
            println!("seconds: {} - cycles: {}", self.seconds, self.cycles);
        }
    }


    pub fn first_run(&mut self) {
        self.cycles += 1;
    }

    pub fn run(&mut self) -> bool {
        self.update();
        true
    }
}


// pub fn start(mut chip8_cpu: Cpu) {

//     let mut cycles: u128 = 0;
//     let mut seconds: u128 = 0;
//     let start = std::time::Instant::now();
//     let mut last_cycle_update = start;
//     let mut last_second_update = start;
//     let cycle_interval = std::time::Duration::from_millis(1000 / CLOCK_SPEED as u64);

//     // run one cycle to start
//     cycles += 1;
//     chip8_cpu.run();
//     chip8_cpu.memory.reset();

//     while seconds < 15 {
//         let now = std::time::Instant::now();
//         if cycles == u128::MAX {
//             panic!("cycles overflowed"); // highly unlikely :)
//         }

//         if now.duration_since(last_cycle_update) >= cycle_interval {
//             last_cycle_update = now;
//             cycles += 1;
//             chip8_cpu.run();
//         }

//         if now.duration_since(last_second_update) >= std::time::Duration::from_secs(1) {
//             println!();
//             last_second_update = now;
//             seconds += 1;
//             chip8_cpu.memory.write(chip8_cpu.memory.get_reserved_end() + (seconds as u16), seconds as u8);
//             println!("seconds: {} - cycles: {}", seconds, cycles);
//         }
//     }
//     chip8_cpu.memory.write(0x0FFF, 0xFF);
//     chip8_cpu.memory.dump(0x0000, 0xFFF);
//     println!("Value at 0x0000: {:02X}", chip8_cpu.memory.read(0x0000));
//     println!("Value at 0x0FFF: {:02X}", chip8_cpu.memory.read(0x0FFF));
// }
