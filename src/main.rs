use std::ptr;

enum Inst {
    MOV,
    ADD,
    FADD,
    SUB,
    FSUB,
    MUL,
    Div,
    And,
    Reg64,
    LDR,
    STR,
}

impl Inst {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Inst::MOV,
            1 => Inst::ADD,
            2 => Inst::FADD,
            3 => Inst::SUB,
            4 => Inst::FSUB,
            5 => Inst::MUL,
            6 => Inst::Div,
            7 => Inst::And,
            8 => Inst::Reg64,
            9 => Inst::LDR,
            10 => Inst::STR,
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

struct Vm {
    m: Vec<u8>,
    regs: [usize; 9],
}

impl Vm {
    fn fetch_instr(&self) -> Inst {
        Inst::from_u8(self.m[self.pc()])
    }
    fn fetch_reg(&self, offset: usize) -> Reg {
        Reg::from_u8(self.m[self.pc() + offset])
    }
    fn fetch(&mut self) -> u8 {
        self.m[self.pc()]
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
            Inst::ADD => {
                self.add();
            }
            Inst::Reg64 => {
                self.reg_64();
            }
            Inst::MOV => {
                self.mov();
            }
            Inst::LDR => {
                self.ldr();
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
        self.inc_ps(9);
    }
    fn add(&mut self) {
        let to = self.fetch_reg(1);
        let left = self.fetch_reg(2);
        let right = self.fetch_reg(3);
        let value = self.get_reg(left) + self.get_reg(right);
        self.set_reg(to, value);
        self.inc_ps(3);
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
    fn pc(&self) -> usize {
        self.regs[Reg::PC as usize]
    }
    fn inc_ps(&mut self, inc: usize) {
        self.regs[Reg::PC as usize] += inc;
    }

    fn new() -> Self {
        Vm {
            m: vec![0; 1024],
            regs: [0; 9],
        }
    }
}

fn main() {
    let mut vm = Vm::new();
    vm.m[0] = Inst::Reg64 as u8;
    vm.m[1] = Reg::R2 as u8;
    vm.m[2] = 255;
    vm.m[3] = 255;
    vm.m[4] = 255;
    vm.m[5] = 255;
    vm.m[6] = 0;
    vm.m[7] = 0;
    vm.m[8] = 0;
    vm.m[9] = 0;
    vm.m[10] = Inst::MOV as u8;
    vm.m[11] = Reg::R0 as u8;
    vm.m[12] = Reg::R2 as u8;
    vm.m[13] = Inst::ADD as u8;
    vm.m[14] = Reg::R1 as u8;
    vm.m[15] = Reg::R0 as u8;
    vm.m[16] = Reg::R2 as u8;
    vm.m[17] = Inst::LDR as u8;
    vm.m[18] = Reg::R6 as u8;
    vm.m[19] = 0xFF;
    vm.m[255] = 0xFF;
    vm.m[256] = 0xFF;
    vm.m[257] = 0xFF;
    vm.m[258] = 0xFF;
    vm.tick();
    vm.tick();
    vm.tick();
    vm.tick();
    println!("addr: {:?}", vm.m[255]);
}
