const MEM_SIZE: usize = 1024 * 64; // Memory size

#[derive(Debug)]
struct Mem([u8; MEM_SIZE]);

impl Mem {
    fn read_addr(self: &Self, addr: usize) -> u8 {
        self.0[addr]
    }

    fn write_addr(self: &mut Self, addr: usize, value: u8) -> u8 {
        self.0[addr] = value;
        self.0[addr]
    }
}

// Opcodes
const INS_LDA_IM: u8 = 0xA9;
const INS_LDA_ZP: u8 = 0xA5;
const INS_LDA_ZPX: u8 = 0x85;
const INS_JSR: u8 = 0x20;

#[derive(Debug)]
struct Cpu {
    pc: u16, // Program counter
    stack: u16, // Stack pointer
    mem: Mem, // The memory

    // Registers
    a: u8,
    x: u8,
    y: u8,

    // Flags
    c: u8,
    z: u8,
    i: u8,
    d: u8,
    b: u8,
    v: u8,
    n: u8,
}


impl Cpu {
    fn reset() -> Self {
        Self {
            pc: 0xFFFC,
            stack: 0x10FF,
            mem: Mem([0; MEM_SIZE]),

            a: 0,
            x: 0,
            y: 0,

            c: 0,
            z: 0,
            i: 0,
            d: 0,
            b: 0,
            v: 0,
            n: 0,
        }
    }

    fn execute(self: &mut Self, mut cycles: u32) {
        while cycles > 0 {
             let ins: u8 = self.fetch_byte(&mut cycles);
            println!("{}", self.a);

            match ins {
                INS_LDA_IM => {
                    let val: u8 = self.fetch_byte(&mut cycles);

                    self.a = val;
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                },
                INS_LDA_ZP => {
                    let zero_addr: u8 = self.fetch_byte(&mut cycles);

                    self.a = self.read_byte(zero_addr, &mut cycles);
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                },
                INS_LDA_ZPX => {
                    let zero_addr: u8 = self.fetch_byte(&mut cycles) + self.x;
                    cycles -= 1;

                    self.a = self.read_byte(zero_addr, &mut cycles);
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                },
                INS_JSR => {
                    let sub_addr: u16 = self.fetch_word(&mut cycles);

                    self.write_word(self.pc as u8, self.pc, &mut cycles);
                    self.pc = sub_addr;
                    cycles -= 1;
                },
                _ => {}
            }
        }
    }

    fn fetch_byte(self: &mut Self, cycles: &mut u32) -> u8 {
        let data: u8 = self.mem.read_addr(self.pc as usize);
        self.pc += 1;
        *cycles -= 1;

        data
    }

    fn read_byte(self: &mut Self, addr: u8, cycles: &mut u32) -> u8 {
        let data: u8 = self.mem.read_addr(addr as usize);
        *cycles -= 1;

        data
    }

    fn fetch_word(self: &mut Self, cycles: &mut u32) -> u16 {
        let data: u16 = self.mem.read_addr((self.pc << 8) as usize) as u16;
        self.pc += 2;
        *cycles -= 2;

        data
    }

    fn write_word(self: &mut Self, addr: u8, value: u16, cycles: &mut u32) {
        self.mem.write_addr(addr as usize, (value & 0xFF) as u8);
        self.mem.write_addr(addr as usize, (value >> 8) as u8);
        *cycles -= 2;
    }
}

fn main() {
    let mut cpu = Cpu::reset();

    cpu.mem.0[0xFFFC] = INS_LDA_IM;
    cpu.mem.0[0xFFFD] = 0x42;

    cpu.execute(3);

}
