#[allow(dead_code)]
const MAXMEMVAL: usize = 0x10000;

pub struct Cpu {
    // 8 bit registers
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,

    // Special registers
    reg_sp: u16,
    reg_pc: u16,
    reg_a: u8,
    reg_f: u8,

    // Memory
    mem: [u8; MAXMEMVAL]
}

impl Cpu {
    pub fn new() -> Self {
        return Self {
            reg_b: 0x00,
            reg_c: 0x13,
            reg_d: 0x00,
            reg_e: 0xD8,
            reg_h: 0x01,
            reg_l: 0x4D,

            reg_sp: 0xFFFE,
            reg_pc: 0x0100,
            reg_a: 0x01,
            reg_f: 0b1000,

            mem: [0; MAXMEMVAL]
        };
    }
}