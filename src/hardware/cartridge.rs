pub struct Cartridge {
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        Cartridge { rom }
    }
}
