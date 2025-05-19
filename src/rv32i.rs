use std::ptr;

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

pub struct RV32I {
    pub pc: u32,
    pub rvg: [u32; 32],
    pub mem: Vec<u8>,
}

impl RV32I {
    pub fn new() -> Self {
        RV32I {
            pc: 0,
            rvg: [0; 32],
            mem: vec![0; 1024 * 1024], // 1MB of memory
        }
    }
    pub fn fetch_instr(&self) -> u32 {
        let ptr = &self.mem[self.pc as usize] as *const u8;
        let ptr = ptr as *const u32;
        unsafe { ptr::read(ptr) }
    }

    pub fn tick(&mut self) {
        let instr = self.fetch_instr();
        let opcode = instr & 0x7F;
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;
        let funct7 = (instr >> 25) & 0x7F;
        match opcode {
            0b0110111 => self.lui(instr),
            0b0010111 => self.auipc(instr),
            0b1101111 => self.jal(instr),
            0b1100111 => self.jalr(instr),
            0b1100011 => self.b_type(instr),
            0b0000011 => {}
            0b0100011 => {}
            0b0010011 => {}
            0b0110011 => {}
            0b0001111 => {}
            0b1110011 => {}
            _ => {}
        }
        self.pc += 4;
    }
    fn lui(&mut self, instr: u32) {
        let rd = (instr >> 7) & 0b11111;
        let imm = instr & 0xFFFFF000;
        self.rvg[rd as usize] = imm;
    }
    fn auipc(&mut self, instr: u32) {
        let rd = (instr >> 7) & 0b11111;
        let imm = instr & 0xFFFFF000;
        self.rvg[rd as usize] = imm + self.pc;
    }
    fn jal(&mut self, instr: u32) {
        let rd = (instr >> 7) & 0b11111;
        let imm_tmp = instr & 0xFFFFF000;
        let imm = ((imm_tmp & 0b10000000000000000000000000000000) >> 11) //[20]
            +((imm_tmp & 0b111111111100000000000000000000) >> 19) // [10:1]
            + ((imm_tmp & 0b100000000000000000000) >> 9) // [11]
            + (imm_tmp & 0b1111111100000000000); // [19:12]
        self.rvg[rd as usize] = self.pc + 4;
        self.pc += imm;
    }
    fn jalr(&mut self, instr: u32) {
        let rd = (instr >> 7) & 0b11111;
        self.rvg[rd as usize] = self.pc + 4;
        self.pc += instr >> 20;
    }
    fn b_type(&mut self, instr: u32) {
        let funct3 = (instr >> 12) & 0b111;
        let rs1 = (instr >> 15) & 0b11111;
        let rs2 = (instr >> 20) & 0b11111;
        let imm_tmp = instr & 0xFFFFF000;
        let imm = ((imm_tmp & 0b10000000000000000000000000000000) >> 11) //[20]
            +((imm_tmp & 0b111111111100000000000000000000) >> 19) // [10:1]
            + ((imm_tmp & 0b100000000000000000000) >> 9) // [11]
            + (imm_tmp & 0b1111111100000000000); // [19:12]
        match funct3 {
            0b000 => self.beq(rs1, rs2, imm),
            0b001 => self.bne(rs1, rs2, imm),
            0b100 => self.blt(rs1, rs2, imm),
            0b101 => self.bge(rs1, rs2, imm),
            0b110 => self.bltu(rs1, rs2, imm),
            0b111 => self.bgeu(rs1, rs2, imm),
            _ => {}
        }
    }
    fn beq(&mut self, rs1: u32, rs2: u32, imm: u32) {}
    fn bne(&mut self, rs1: u32, rs2: u32, imm: u32) {}
    fn blt(&mut self, rs1: u32, rs2: u32, imm: u32) {}
    fn bge(&mut self, rs1: u32, rs2: u32, imm: u32) {}
    fn bltu(&mut self, rs1: u32, rs2: u32, imm: u32) {}
    fn bgeu(&mut self, rs1: u32, rs2: u32, imm: u32) {}
}
