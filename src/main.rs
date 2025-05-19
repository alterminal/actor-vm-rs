use std::ptr;
enum Inst {
    Mov,
    Add,
    FAdd,
    Sub,
    FSub,
    Mul,
    Div,
    And,
    Reg64,
    Ldr,
    Str,
}

impl Inst {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Inst::Mov,
            1 => Inst::Add,
            2 => Inst::FAdd,
            3 => Inst::Sub,
            4 => Inst::FSub,
            5 => Inst::Mul,
            6 => Inst::Div,
            7 => Inst::And,
            8 => Inst::Reg64,
            9 => Inst::Ldr,
            10 => Inst::Str,
            _ => panic!("Unknown instruction"),
        }
    }
}

enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    SP,
}

impl Reg {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Reg::R0,
            1 => Reg::R1,
            2 => Reg::R2,
            3 => Reg::R3,
            4 => Reg::R4,
            5 => Reg::R5,
            6 => Reg::R6,
            7 => Reg::R7,
            8 => Reg::PC,
            9 => Reg::SP,
            _ => panic!("Unknown register"),
        }
    }
}

enum RV64I {
    ADD,
    ADDI,
    AND,
    ANDI,
    OR,
    ORI,
    XOR,
    XORI,
    SLL,
    SLLI,
    SRL,
    SRLI,
    SRA,
    SRAI,
    SUB,
    SUBI,
}

// RISC-V general purpose registers
enum RVG {
    ZERO,
    RA,
    SP,
    GP,
    TP,
    T0,
    T1,
    T2,
    FP,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

// RISC-V floating point registers
enum RGF {
    FT0,
    FT1,
    FT2,
    FT3,
    FT4,
    FT5,
    FT6,
    FT7,
    FS0,
    FS1,
    FA0,
    FA1,
    FA2,
    FA3,
    FA4,
    FA5,
    FA6,
    FA7,
    FS2,
    FS3,
    FS4,
    FS5,
    FS6,
    FS7,
    FS8,
    FS9,
    FS10,
    FS11,
    FT8,
    FT9,
    FT10,
    FT11,
}

struct ActorVm {
    m: Vec<u8>,
    regs: [usize; 9],
    rvi: [usize; 32],
    rvf: [f64; 32],
}

impl ActorVm {
    fn fetch_instr(&self) -> Inst {
        Inst::from_u8(self.m[self.pc()])
    }
    fn fetch_reg(&self, offset: usize) -> Reg {
        Reg::from_u8(self.m[self.pc() + offset])
    }
    fn fetch_8(&self, addr: usize) -> u8 {
        let ptr = &self.m[addr] as *const u8;
        unsafe { ptr::read(ptr) }
    }
    fn fetch_usize(&self, addr: usize) -> usize {
        let ptr = &self.m[addr] as *const u8;
        let ptr = ptr as *const usize;
        unsafe { ptr::read(ptr) }
    }
    fn fetch_64(&self, addr: usize) -> u64 {
        let ptr = &self.m[addr] as *const u8;
        let ptr = ptr as *const u64;
        unsafe { ptr::read(ptr) }
    }
    fn fetch_addr(&self, offset: usize) -> usize {
        let ptr = &self.m[self.pc() + offset] as *const u8;
        let ptr = ptr as *const usize;
        unsafe { ptr::read(ptr) }
    }
    fn fetch_u64(&self, addr: usize) -> u64 {
        let ptr = &self.m[addr] as *const u8;
        let ptr = ptr as *const u64;
        unsafe { ptr::read(ptr) }
    }
    fn set_reg(&mut self, reg: Reg, value: usize) {
        self.regs[reg as usize] = value;
    }
    fn get_reg(&self, reg: Reg) -> usize {
        self.regs[reg as usize]
    }
    fn get_reg_float64(&self, reg: Reg) -> f64 {
        let ptr = &self.regs[reg as usize] as *const usize;
        let ptr = ptr as *const f64;
        unsafe { ptr::read(ptr) }
    }
    fn tick(&mut self) {
        let inst = self.fetch_instr();
        match inst {
            Inst::Add => {
                self.add();
            }
            Inst::Reg64 => {
                self.reg_64();
            }
            Inst::Mov => {
                self.mov();
            }
            Inst::Ldr => {
                self.ldr();
            }
            Inst::Str => {
                self.str();
            }
            _ => {
                println!("hello:{:?}", inst as u8);
            }
        }
    }
    fn ldr(&mut self) {
        let reg = self.fetch_reg(1);
        let addr = self.fetch_addr(2);
        let value = self.fetch_64(addr);
        self.set_reg(reg, value as usize);
        self.inc_ps(10);
    }
    fn str(&mut self) {
        let reg = self.fetch_reg(1);
        let value = self.get_reg(reg);
        let addr = self.fetch_addr(2);
        let ptr = &mut self.m[addr] as *mut u8;
        let ptr = ptr as *mut usize;
        unsafe { ptr::write(ptr, value) }
        self.inc_ps(10);
    }
    fn add(&mut self) {
        let to = self.fetch_reg(1);
        let left = self.fetch_reg(2);
        let right = self.fetch_reg(3);
        let value = self.get_reg(left) + self.get_reg(right);
        self.set_reg(to, value);
        self.inc_ps(4);
    }
    fn reg_64(&mut self) {
        let reg = self.fetch_reg(1);
        let value = self.fetch_64(self.pc() + 2) as usize;
        self.set_reg(reg, value);
        self.inc_ps(10);
    }
    fn mov(&mut self) {
        let to = self.fetch_reg(1);
        let from = self.fetch_reg(2);
        let value = self.get_reg(from);
        self.set_reg(to, value);
        self.inc_ps(3);
    }
    fn print(&self) {
        println!("PC: {}", self.pc());
        println!("Registers: {:?}", self.regs);
    }
    fn print_mem(&self, addr: usize, offset: usize) {
        for i in 0..offset {
            println!("{}: {}", addr + i, self.m[addr + i]);
        }
    }
    fn pc(&self) -> usize {
        self.regs[Reg::PC as usize]
    }
    fn inc_ps(&mut self, inc: usize) {
        self.regs[Reg::PC as usize] += inc;
    }
    fn new() -> Self {
        ActorVm {
            m: vec![0; 1024],
            rvi: [0; 32],
            rvf: [0.0; 32],
            regs: [0; 9],
        }
    }
}

fn main() {}
