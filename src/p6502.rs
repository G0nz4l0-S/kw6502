use copperline::Copperline;

#[derive(Default)]
pub struct P6502 {
    /// Program counter
    pub pc: u16,
    // Stack pointer
    pub sp: u8,
    /// Accumulator
    pub a: u8,
    /// Register X
    pub x: u8,
    /// Register Y
    pub y: u8,
    /// Procesor flags
    pub flags: P6502Flags,
    /// The memory associated to the CPU
    pub memory: Memory,
    /// The number of cycles remaining for the CPU
    cycles: usize,
}
/// Represents the 6502 seven status flags
#[derive(Default)]
pub struct P6502Flags {
    /// Carry flag
    pub c: bool,
    /// Zero flag
    pub z: bool,
    /// Interrupt mask
    pub i: bool,
    /// Decimal flag
    pub d: bool,
    /// Break flag
    pub b: bool,
    /// Overflow flag
    pub v: bool,
    /// Negative flag
    pub n: bool,
}

impl P6502Flags {
    /// Returns an 8-bit representarion of the 7 flags.
    pub fn as_binary(&self) -> u8 {
        let mut representation: u8 = 0;
        representation |= self.c as u8;
        representation |= (self.z as u8) << 1;
        representation |= (self.i as u8) << 2;
        representation |= (self.d as u8) << 3;
        representation |= (self.b as u8) << 4;
        representation |= 1 << 5;
        representation |= (self.v as u8) << 6;
        representation |= (self.n as u8) << 7;

        representation
    }

    /// Sets the status flag from an 8-bit number.
    pub fn from_binary(value: u8) -> Self {
        Self {
            c: (value & 0b1) != 0,
            z: (value & 0b10) != 0,
            i: (value & 0b100) != 0,
            d: (value & 0b1000) != 0,
            b: (value & 0b10000) != 0,
            v: (value & 0b1000000) != 0,
            n: (value & 0b10000000) != 0,
        }
    }
}

impl P6502 {
    /// Indicates the begining of the stack page. As is the 6502, it corresponds to the second page: $0100 - $01ff
    const STACK_PAGE: u16 = 0x0100;
    /// The default memory location where the program counter is set when starting
    /// or after a CPU reset.
    pub const PROGRAM_START: u16 = 0x0600;

    /* LIST OF OPCODES */

    /* System operations */
    pub const INS_NOP: u8 = 0xEA;
    pub const INS_BRK: u8 = 0x00;
    pub const INS_RTI: u8 = 0x40;

    /* Load and store operations */
    pub const INS_LDA_IMM: u8 = 0xA9;
    pub const INS_LDA_ZP0: u8 = 0xA5;
    pub const INS_LDA_ZPX: u8 = 0xB5;
    pub const INS_LDA_ABS: u8 = 0xAD;
    pub const INS_LDA_ABX: u8 = 0xBD;
    pub const INS_LDA_ABY: u8 = 0xB9;
    pub const INS_LDA_IDX: u8 = 0xA1;
    pub const INS_LDA_IDY: u8 = 0xB1;
    pub const INS_LDX_IMM: u8 = 0xA2;
    pub const INS_LDX_ZP0: u8 = 0xA6;
    pub const INS_LDX_ZPY: u8 = 0xB6;
    pub const INS_LDX_ABS: u8 = 0xAE;
    pub const INS_LDX_ABY: u8 = 0xBE;
    pub const INS_LDY_IMM: u8 = 0xA0;
    pub const INS_LDY_ZP0: u8 = 0xA4;
    pub const INS_LDY_ZPX: u8 = 0xB4;
    pub const INS_LDY_ABS: u8 = 0xAC;
    pub const INS_LDY_ABX: u8 = 0xBC;

    pub const INS_STA_ZP0: u8 = 0x85;
    pub const INS_STA_ZPX: u8 = 0x95;
    pub const INS_STA_ABS: u8 = 0x8D;
    pub const INS_STA_ABX: u8 = 0x9D;
    pub const INS_STA_ABY: u8 = 0x99;
    pub const INS_STA_IDX: u8 = 0x81;
    pub const INS_STA_IDY: u8 = 0x91;
    pub const INS_STX_ZP0: u8 = 0x86;
    pub const INS_STX_ZPY: u8 = 0x96;
    pub const INS_STX_ABS: u8 = 0x8E;
    pub const INS_STY_ZP0: u8 = 0x84;
    pub const INS_STY_ZPX: u8 = 0x94;
    pub const INS_STY_ABS: u8 = 0x8C;

    /* Bit shifts */
    pub const INS_ASL_ACC: u8 = 0x0A;
    pub const INS_ASL_ZP0: u8 = 0x06;
    pub const INS_ASL_ZPX: u8 = 0x16;
    pub const INS_ASL_ABS: u8 = 0x0E;
    pub const INS_ASL_ABX: u8 = 0x1E;
    pub const INS_LSR_ACC: u8 = 0x4A;
    pub const INS_LSR_ZP0: u8 = 0x46;
    pub const INS_LSR_ZPX: u8 = 0x56;
    pub const INS_LSR_ABS: u8 = 0x4E;
    pub const INS_LSR_ABX: u8 = 0x5E;
    pub const INS_ROL_ACC: u8 = 0x2A;
    pub const INS_ROL_ZP0: u8 = 0x26;
    pub const INS_ROL_ZPX: u8 = 0x36;
    pub const INS_ROL_ABS: u8 = 0x2E;
    pub const INS_ROL_ABX: u8 = 0x3E;
    pub const INS_ROR_ACC: u8 = 0x6A;
    pub const INS_ROR_ZP0: u8 = 0x66;
    pub const INS_ROR_ZPX: u8 = 0x76;
    pub const INS_ROR_ABS: u8 = 0x6E;
    pub const INS_ROR_ABX: u8 = 0x7E;

    /* Status flags operations */
    pub const INS_CLC: u8 = 0x18;
    pub const INS_CLD: u8 = 0xD8;
    pub const INS_CLI: u8 = 0x58;
    pub const INS_CLV: u8 = 0xB8;
    pub const INS_SEC: u8 = 0x38;
    pub const INS_SED: u8 = 0xF8;
    pub const INS_SEI: u8 = 0x78;

    /* Register transfers */
    pub const INS_TAX: u8 = 0xAA;
    pub const INS_TAY: u8 = 0xA8;
    pub const INS_TXA: u8 = 0x8A;
    pub const INS_TYA: u8 = 0x98;
    pub const INS_TSX: u8 = 0xBA;
    pub const INS_TXS: u8 = 0x9A;

    /* Stack operations */
    pub const INS_PHA: u8 = 0x48;
    pub const INS_PHP: u8 = 0x08;
    pub const INS_PLA: u8 = 0x68;
    pub const INS_PLP: u8 = 0x28;

    /* Jumps, branches and returns */
    pub const INS_JMP_ABS: u8 = 0x4C;
    pub const INS_JMP_IND: u8 = 0x6C;
    pub const INS_JSR_ABS: u8 = 0x20;
    pub const INS_RTS: u8 = 0x60;
    pub const INS_BCC_REL: u8 = 0x90;
    pub const INS_BCS_REL: u8 = 0xB0;
    pub const INS_BEQ_REL: u8 = 0xF0;
    pub const INS_BMI_REL: u8 = 0x30;
    pub const INS_BNE_REL: u8 = 0xD0;
    pub const INS_BPL_REL: u8 = 0x10;
    pub const INS_BVC_REL: u8 = 0x50;
    pub const INS_BVS_REL: u8 = 0x70;

    /* Increments and decrements */
    pub const INS_INC_ZP0: u8 = 0xE6;
    pub const INS_INC_ZPX: u8 = 0xF6;
    pub const INS_INC_ABS: u8 = 0xEE;
    pub const INS_INC_ABX: u8 = 0xFE;
    pub const INS_INX: u8 = 0xE8;
    pub const INS_INY: u8 = 0xC8;
    pub const INS_DEC_ZP0: u8 = 0xC6;
    pub const INS_DEC_ZPX: u8 = 0xD6;
    pub const INS_DEC_ABS: u8 = 0xCE;
    pub const INS_DEC_ABX: u8 = 0xDE;
    pub const INS_DEX: u8 = 0xCA;
    pub const INS_DEY: u8 = 0x88;

    /* Arithmetic and comparisions */
    pub const INS_ADC_IMM: u8 = 0x69;
    pub const INS_ADC_ZP0: u8 = 0x65;
    pub const INS_ADC_ZPX: u8 = 0x75;
    pub const INS_ADC_ABS: u8 = 0x6D;
    pub const INS_ADC_ABX: u8 = 0x7D;
    pub const INS_ADC_ABY: u8 = 0x79;
    pub const INS_ADC_IDX: u8 = 0x61;
    pub const INS_ADC_IDY: u8 = 0x71;
    pub const INS_SBC_IMM: u8 = 0xE9;
    pub const INS_SBC_ZP0: u8 = 0xE5;
    pub const INS_SBC_ZPX: u8 = 0xF5;
    pub const INS_SBC_ABS: u8 = 0xED;
    pub const INS_SBC_ABX: u8 = 0xFD;
    pub const INS_SBC_ABY: u8 = 0xF9;
    pub const INS_SBC_IDX: u8 = 0xE1;
    pub const INS_SBC_IDY: u8 = 0xF1;

    pub const INS_CMP_IMM: u8 = 0xC9;
    pub const INS_CMP_ZP0: u8 = 0xC5;
    pub const INS_CMP_ZPX: u8 = 0xD5;
    pub const INS_CMP_ABS: u8 = 0xCD;
    pub const INS_CMP_ABX: u8 = 0xDD;
    pub const INS_CMP_ABY: u8 = 0xD9;
    pub const INS_CMP_IDX: u8 = 0xC1;
    pub const INS_CMP_IDY: u8 = 0xD1;
    pub const INS_CPX_IMM: u8 = 0xE0;
    pub const INS_CPX_ZP0: u8 = 0xE4;
    pub const INS_CPX_ABS: u8 = 0xEC;
    pub const INS_CPY_IMM: u8 = 0xC0;
    pub const INS_CPY_ZP0: u8 = 0xC4;
    pub const INS_CPY_ABS: u8 = 0xCC;

    /* Binary logical operations */
    pub const INS_AND_IMM: u8 = 0x29;
    pub const INS_AND_ZP0: u8 = 0x25;
    pub const INS_AND_ZPX: u8 = 0x35;
    pub const INS_AND_ABS: u8 = 0x2D;
    pub const INS_AND_ABX: u8 = 0x3D;
    pub const INS_AND_ABY: u8 = 0x39;
    pub const INS_AND_IDX: u8 = 0x21;
    pub const INS_AND_IDY: u8 = 0x31;
    pub const INS_ORA_IMM: u8 = 0x09;
    pub const INS_ORA_ZP0: u8 = 0x05;
    pub const INS_ORA_ZPX: u8 = 0x15;
    pub const INS_ORA_ABS: u8 = 0x0D;
    pub const INS_ORA_ABX: u8 = 0x1D;
    pub const INS_ORA_ABY: u8 = 0x19;
    pub const INS_ORA_IDX: u8 = 0x01;
    pub const INS_ORA_IDY: u8 = 0x11;
    pub const INS_EOR_IMM: u8 = 0x49;
    pub const INS_EOR_ZP0: u8 = 0x45;
    pub const INS_EOR_ZPX: u8 = 0x55;
    pub const INS_EOR_ABS: u8 = 0x4D;
    pub const INS_EOR_ABX: u8 = 0x5D;
    pub const INS_EOR_ABY: u8 = 0x59;
    pub const INS_EOR_IDX: u8 = 0x41;
    pub const INS_EOR_IDY: u8 = 0x51;
    pub const INS_BIT_ZP0: u8 = 0x24;
    pub const INS_BIT_ABS: u8 = 0x2C;

    /// Returns a string representation of the processor's registers and the number of cycles consumed.
    pub fn status(&self) -> String {
        format!(
            "PC=${:04x}, SP=${:02x}\nA=${:02x}, X=${:02x}, Y=${:02x}\nFlags={:08b}\n",
            self.pc,
            self.sp,
            self.a,
            self.x,
            self.y,
            self.flags.as_binary(),
        )
    }
}
impl P6502 {
    /// Runs the whole code loaded into memory until a $00 opcode (BRK) is reached.
    pub fn execute(&mut self) {
        loop {
            let instruction: u8 = self.next_byte();

            match instruction {
                P6502::INS_NOP => {
                    self.clock_tick(1);
                }
                /*
                 In this implementation, the BRK command (=$00) is used to quit the program, since it represents an empty memory location.
                 A real BRK instruction would look something like this:

                 P6502::INS_BRK => {
                    self.flags.b = true;
                    self.push_word(self.pc);
                    self.pc = 0xFFFE;
                    self.push_byte(self.flags.as_binary());
                    self.clock_tick(3);
                    break;
                } */

                P6502::INS_BRK => {
                    self.pc = self.pc.saturating_sub(1);
                    break;
                }

                P6502::INS_RTI => {
                    let bin_flags: u8 = self.pull_byte();
                    let new_pc: u16 = self.pull_word();

                    self.flags = P6502Flags::from_binary(bin_flags);
                    self.pc = new_pc;
                    self.clock_tick(2);
                }

                P6502::INS_LDA_IMM => {
                    let value: u8 = self.next_byte();
                    self.a = value;

                    self.flags.z = value == 0;
                    self.flags.n = (value & 0b10000000) > 0;
                }

                P6502::INS_LDA_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr as u16);
                    self.a = value;

                    self.flags.z = value == 0;
                    self.flags.n = (value & 0b10000000) > 0;
                }

                P6502::INS_LDA_ZPX => {
                    let zp_addr: u8 = self.read_byte_from_addr(self.pc);
                    let value: u8 = self.read_byte_from_addr(zp_addr.saturating_add(self.x) as u16);
                    self.a = value;
                    self.clock_tick(1);
                    self.flags.z = value == 0;
                    self.flags.n = (value & 0b10000000) > 0;
                }

                P6502::INS_LDA_ABS => {
                    let addr: u16 = self.next_word();
                    self.a = self.read_byte_from_addr(addr);
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_LDA_ABX => {
                    let addr: u16 = self.next_word();
                    self.a = self.read_byte_from_addr(addr + self.x as u16);
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_LDA_ABY => {
                    let addr: u16 = self.next_word();
                    self.a = self.read_byte_from_addr(addr + self.y as u16);
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_LDA_IDX => {
                    let indirect: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indexed_indirect_addr(indirect);
                    self.a = self.read_byte_from_addr(final_addr);
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_LDA_IDY => {
                    let indirect_addr: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indirect_indexing_addr(indirect_addr);
                    self.a = self.read_byte_from_addr(final_addr);
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & (1 << 7)) != 0;
                }
                P6502::INS_JSR_ABS => {
                    let jump_addr: u16 = self.next_word();
                    self.push_word(self.pc - 1);
                    self.pc = jump_addr;
                    self.clock_tick(1);
                }

                P6502::INS_RTS => {
                    self.pc = self.pull_word() + 1;
                    self.clock_tick(3);
                }

                P6502::INS_BCC_REL => {
                    let jump_addr: u8 = self.next_byte();
                    if !self.flags.c {
                        self.relative_jump(jump_addr);
                    }
                }

                P6502::INS_BCS_REL => {
                    let jump_addr: u8 = self.next_byte();
                    if self.flags.c {
                        self.relative_jump(jump_addr);
                    }
                }

                P6502::INS_BEQ_REL => {
                    let jump_addr: u8 = self.next_byte();
                    if self.flags.z {
                        self.relative_jump(jump_addr);
                    }
                }

                P6502::INS_BMI_REL => {
                    let jump_addr: u8 = self.next_byte();
                    if self.flags.n {
                        self.relative_jump(jump_addr);
                    }
                }

                P6502::INS_BNE_REL => {
                    let jump_addr: u8 = self.next_byte();
                    if !self.flags.z {
                        self.relative_jump(jump_addr);
                    }
                }

                P6502::INS_BPL_REL => {
                    let jump_addr: u8 = self.next_byte();
                    if !self.flags.n {
                        self.relative_jump(jump_addr);
                    }
                }

                P6502::INS_BVC_REL => {
                    let jump_addr: u8 = self.next_byte();
                    if !self.flags.v {
                        self.relative_jump(jump_addr);
                    }
                }

                P6502::INS_BVS_REL => {
                    let jump_addr: u8 = self.next_byte();
                    if self.flags.v {
                        self.relative_jump(jump_addr);
                    }
                }

                P6502::INS_LDX_IMM => {
                    let value: u8 = self.next_byte();
                    self.x = value;

                    self.flags.z = value == 0;
                    self.flags.n = (value & 0b10000000) > 0;
                }

                P6502::INS_LDX_ZP0 => {
                    let addr: u16 = self.next_byte() as u16;
                    self.x = self.read_byte_from_addr(addr);
                    self.flags.z = self.x == 0;
                    self.flags.n = (self.x & 0b10000000) > 0;
                }

                P6502::INS_LDX_ZPY => {
                    let addr: u8 = self.next_byte();
                    self.x = self.read_byte_from_addr(addr.wrapping_add(self.y) as u16);

                    self.flags.z = self.x == 0;
                    self.flags.n = (self.x & 0b10000000) > 0;
                }

                P6502::INS_LDX_ABS => {
                    let addr: u16 = self.next_word();
                    self.x = self.read_byte_from_addr(addr);

                    self.flags.z = self.x == 0;
                    self.flags.n = (self.x & 0b10000000) > 0;
                }

                P6502::INS_LDX_ABY => {
                    let addr: u16 = self.next_word() + self.y as u16;
                    self.x = self.read_byte_from_addr(addr);

                    self.flags.z = self.x == 0;
                    self.flags.n = (self.x & 0b10000000) > 0;
                }

                P6502::INS_LDY_IMM => {
                    self.y = self.next_byte();

                    self.flags.z = self.y == 0;
                    self.flags.n = (self.y & 0b10000000) > 0;
                }

                P6502::INS_LDY_ZP0 => {
                    let addr: u16 = self.next_byte() as u16;
                    self.y = self.read_byte_from_addr(addr);
                    self.flags.z = self.y == 0;
                    self.flags.n = (self.y & 0b10000000) > 0;
                }

                P6502::INS_LDY_ZPX => {
                    let addr: u8 = self.next_byte();
                    self.x = self.read_byte_from_addr(addr.saturating_add(self.x) as u16);

                    self.flags.z = self.y == 0;
                    self.flags.n = (self.y & 0b10000000) > 0;
                }

                P6502::INS_LDY_ABS => {
                    let addr: u16 = self.next_word();
                    self.x = self.read_byte_from_addr(addr);

                    self.flags.z = self.y == 0;
                    self.flags.n = (self.y & 0b10000000) > 0;
                }

                P6502::INS_LDY_ABX => {
                    let addr: u16 = self.next_word();
                    self.x = self.read_byte_from_addr(addr + self.x as u16);

                    self.flags.z = self.y == 0;
                    self.flags.n = (self.y & 0b10000000) > 0;
                }

                P6502::INS_PHA => {
                    self.push_byte(self.a);
                    self.clock_tick(1);
                }

                P6502::INS_PHP => {
                    self.push_byte(self.flags.as_binary());
                    self.clock_tick(1);
                }

                P6502::INS_PLA => {
                    self.a = self.pull_byte();
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                    self.clock_tick(2);
                }

                P6502::INS_PLP => {
                    self.flags = P6502Flags::from_binary(self.pull_byte());
                    self.clock_tick(2);
                }

                P6502::INS_JMP_ABS => {
                    let addr: u16 = self.next_word();
                    self.pc = addr;
                    self.clock_tick(1);
                }

                P6502::INS_JMP_IND => {
                    let indirect_addr: u16 = self.next_word();
                    let jump_addr: u16 = if indirect_addr & 0x00FF == 0xFF {
                        let lsb: u8 = self.read_byte_from_addr(indirect_addr);
                        let msb: u8 = self.read_byte_from_addr(indirect_addr & 0xFF00);
                        (msb as u16) << 8 | (lsb as u16)
                    } else {
                        self.read_word_from_addr(indirect_addr)
                    };
                    self.pc = jump_addr;
                    self.clock_tick(1);
                }

                P6502::INS_TAX => {
                    self.x = self.a;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                    self.clock_tick(1);
                }

                P6502::INS_TAY => {
                    self.y = self.a;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                    self.clock_tick(1);
                }

                P6502::INS_TSX => {
                    self.x = self.sp;
                    self.flags.z = self.x == 0;
                    self.flags.n = (self.x & 0b10000000) > 0;
                    self.clock_tick(1);
                }

                P6502::INS_TXA => {
                    self.a = self.x;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                    self.clock_tick(1);
                }

                P6502::INS_TXS => {
                    self.sp = self.x;
                    self.clock_tick(1);
                }

                P6502::INS_TYA => {
                    self.a = self.y;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                    self.clock_tick(1);
                }
                P6502::INS_INC_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    self.increment_memory(zp_addr as u16);
                }

                P6502::INS_INC_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    self.increment_memory(zp_addr.wrapping_add(self.x) as u16);
                    self.clock_tick(1);
                }

                P6502::INS_INC_ABS => {
                    let addr: u16 = self.next_word();
                    self.increment_memory(addr);
                    self.clock_tick(1);
                }

                P6502::INS_INC_ABX => {
                    let addr: u16 = self.next_word();
                    self.increment_memory(addr + self.x as u16);
                    self.clock_tick(2);
                }

                P6502::INS_INX => {
                    self.x = self.x.wrapping_add(1);
                    self.flags.z = self.x == 0;
                    self.flags.n = (self.x & 0b10000000) > 0;
                    self.clock_tick(1);
                }

                P6502::INS_INY => {
                    self.y = self.y.wrapping_add(1);
                    self.flags.z = self.y == 0;
                    self.flags.n = (self.y & 0b10000000) > 0;
                    self.clock_tick(1);
                }

                P6502::INS_DEC_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    self.decrement_memory(zp_addr as u16);
                    self.clock_tick(1);
                }

                P6502::INS_DEC_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    self.decrement_memory(zp_addr.wrapping_add(self.x) as u16);
                    self.clock_tick(1);
                }

                P6502::INS_DEC_ABS => {
                    let addr: u16 = self.next_word();
                    self.decrement_memory(addr);
                }

                P6502::INS_DEC_ABX => {
                    let addr: u16 = self.next_word();
                    self.decrement_memory(addr);
                    self.clock_tick(1);
                }

                P6502::INS_DEX => {
                    self.x = self.x.wrapping_sub(1);
                    self.flags.z = self.x == 0;
                    self.flags.n = (self.x & 0b10000000) > 0;
                    self.clock_tick(1);
                }

                P6502::INS_DEY => {
                    self.y = self.y.wrapping_sub(1);
                    self.flags.z = self.y == 0;
                    self.flags.n = (self.y & 0b10000000) > 0;
                    self.clock_tick(1);
                }

                P6502::INS_STA_ZP0 => {
                    let addr: u8 = self.next_byte();
                    self.write_byte_to_addr(addr as u16, self.a);
                }

                P6502::INS_STA_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    self.write_byte_to_addr(zp_addr.wrapping_add(self.x) as u16, self.a);
                    self.clock_tick(1);
                }

                P6502::INS_STA_ABS => {
                    let addr: u16 = self.next_word();
                    self.write_byte_to_addr(addr, self.a);
                }

                P6502::INS_STA_ABX => {
                    let addr: u16 = self.next_word();
                    self.write_byte_to_addr(addr + self.x as u16, self.a);
                    self.clock_tick(1);
                }

                P6502::INS_STA_ABY => {
                    let addr: u16 = self.next_word();
                    self.write_byte_to_addr(addr + self.y as u16, self.a);
                    self.clock_tick(1);
                }

                P6502::INS_STA_IDX => {
                    let indirect: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indexed_indirect_addr(indirect);
                    self.write_byte_to_addr(final_addr, self.a);
                }

                P6502::INS_STA_IDY => {
                    let indirect_addr: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indirect_indexing_addr(indirect_addr);
                    self.write_byte_to_addr(final_addr, self.a);
                    self.clock_tick(1);
                }

                P6502::INS_STX_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    self.write_byte_to_addr(zp_addr as u16, self.x);
                }

                P6502::INS_STX_ZPY => {
                    let zp_addr: u8 = self.next_byte();
                    self.write_byte_to_addr(zp_addr.wrapping_add(self.x) as u16, self.x);
                    self.clock_tick(1);
                }

                P6502::INS_STX_ABS => {
                    let addr: u16 = self.next_word();
                    self.write_byte_to_addr(addr as u16, self.x);
                    self.clock_tick(1);
                }

                P6502::INS_STY_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    self.write_byte_to_addr(zp_addr as u16, self.y);
                }

                P6502::INS_STY_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    self.write_byte_to_addr(zp_addr.wrapping_add(self.x) as u16, self.y);
                    self.clock_tick(1);
                }

                P6502::INS_STY_ABS => {
                    let addr: u16 = self.next_word();
                    self.write_byte_to_addr(addr as u16, self.y);
                    self.clock_tick(1);
                }

                P6502::INS_CMP_IMM => {
                    let value: u8 = self.next_byte();
                    self.flags.c = self.a >= value;
                    self.flags.z = self.a == value;
                    self.flags.n = (self.a.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CMP_ZP0 => {
                    let zp_addr: u16 = self.next_byte() as u16;
                    let value: u8 = self.read_byte_from_addr(zp_addr);
                    self.flags.c = self.a >= value;
                    self.flags.z = self.a == value;
                    self.flags.n = (self.a.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CMP_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr.wrapping_add(self.x) as u16);
                    self.flags.c = self.a >= value;
                    self.flags.z = self.a == value;
                    self.flags.n = (self.a.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CMP_ABS => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr);
                    self.flags.c = self.a >= value;
                    self.flags.z = self.a == value;
                    self.flags.n = (self.a.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CMP_ABX => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr + self.x as u16);
                    self.flags.c = self.a >= value;
                    self.flags.z = self.a == value;
                    self.flags.n = (self.a.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CMP_ABY => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr + self.y as u16);
                    self.flags.c = self.a >= value;
                    self.flags.z = self.a == value;
                    self.flags.n = (self.a.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CMP_IDX => {
                    let indirect: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indexed_indirect_addr(indirect);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.flags.c = self.a >= value;
                    self.flags.z = self.a == value;
                    self.flags.n = (self.a.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CMP_IDY => {
                    let indirect_addr: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indirect_indexing_addr(indirect_addr);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.flags.c = self.a >= value;
                    self.flags.z = self.a == value;
                    self.flags.n = (self.a.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CPX_IMM => {
                    let value: u8 = self.next_byte();
                    self.flags.c = self.x >= value;
                    self.flags.z = self.x == value;
                    self.flags.n = (self.x.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CPX_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr as u16);
                    self.flags.c = self.x >= value;
                    self.flags.z = self.x == value;
                    self.flags.n = (self.x.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CPX_ABS => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr as u16);
                    self.flags.c = self.x >= value;
                    self.flags.z = self.x == value;
                    self.flags.n = (self.x.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CPY_IMM => {
                    let value: u8 = self.next_byte();
                    self.flags.c = self.y >= value;
                    self.flags.z = self.y == value;
                    self.flags.n = (self.y.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CPY_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr as u16);
                    self.flags.c = self.y >= value;
                    self.flags.z = self.y == value;
                    self.flags.n = (self.y.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_CPY_ABS => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr as u16);
                    self.flags.c = self.y >= value;
                    self.flags.z = self.y == value;
                    self.flags.n = (self.y.wrapping_sub(value) & 0b10000000) > 0;
                }

                P6502::INS_ADC_IMM => {
                    let value: u8 = self.next_byte();
                    self.add_with_carry(value);
                }

                P6502::INS_ADC_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr as u16);
                    self.add_with_carry(value);
                }

                P6502::INS_ADC_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr.wrapping_add(self.x) as u16);
                    self.add_with_carry(value);
                }

                P6502::INS_ADC_ABS => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr);
                    self.add_with_carry(value);
                }

                P6502::INS_ADC_ABX => {
                    let addr: u16 = self.next_word();
                    let final_addr: u16 = addr + self.x as u16;
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.add_with_carry(value);
                }

                P6502::INS_ADC_ABY => {
                    let addr: u16 = self.next_word();
                    let final_addr: u16 = addr + self.y as u16;
                    if addr & 0xFF00 != final_addr & 0xFF00 {
                        self.clock_tick(1);
                    }
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.add_with_carry(value);
                }

                P6502::INS_ADC_IDX => {
                    let indirect: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indexed_indirect_addr(indirect);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.add_with_carry(value);
                }

                P6502::INS_ADC_IDY => {
                    let indirect: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indirect_indexing_addr(indirect);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.add_with_carry(value);
                    self.clock_tick(1);
                }

                P6502::INS_SBC_IMM => {
                    let value: u8 = self.next_byte();
                    self.substract_with_carry(value);
                }

                P6502::INS_SBC_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr as u16);
                    self.substract_with_carry(value);
                }

                P6502::INS_SBC_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr.wrapping_add(self.x) as u16);
                    self.substract_with_carry(value);
                    self.clock_tick(1);
                }

                P6502::INS_SBC_ABS => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr);
                    self.substract_with_carry(value);
                }

                P6502::INS_SBC_ABX => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr + self.x as u16);
                    self.substract_with_carry(value);
                }

                P6502::INS_SBC_ABY => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr + self.y as u16);
                    self.substract_with_carry(value);
                }

                P6502::INS_SBC_IDX => {
                    let indirect: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indexed_indirect_addr(indirect);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.substract_with_carry(value);
                }
                P6502::INS_SBC_IDY => {
                    let indirect_addr: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indirect_indexing_addr(indirect_addr);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.substract_with_carry(value);
                }

                P6502::INS_AND_IMM => {
                    let value: u8 = self.next_byte();
                    self.a &= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_AND_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr as u16);
                    self.a &= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_AND_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr.wrapping_add(self.x) as u16);
                    self.a &= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_AND_ABS => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr);
                    self.a &= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_AND_ABX => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr + self.x as u16);
                    self.a &= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_AND_ABY => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr + self.y as u16);
                    self.a &= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_AND_IDX => {
                    let indirect: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indexed_indirect_addr(indirect);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.a &= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_AND_IDY => {
                    let indirect_addr: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indirect_indexing_addr(indirect_addr);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.a |= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_ORA_IMM => {
                    let value: u8 = self.next_byte();
                    self.a |= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_ORA_ZP0 => {
                    let addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(addr as u16);
                    self.a |= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_ORA_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr.wrapping_add(self.x) as u16);
                    self.a |= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_ORA_ABS => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr);
                    self.a |= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_ORA_ABX => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr + self.x as u16);
                    self.a |= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_ORA_ABY => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr + self.y as u16);
                    self.a |= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_ORA_IDX => {
                    let indirect: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indexed_indirect_addr(indirect);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.a |= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_ORA_IDY => {
                    let indirect_addr: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indirect_indexing_addr(indirect_addr);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.a |= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_EOR_IMM => {
                    let value: u8 = self.next_byte();
                    self.a ^= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_EOR_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr as u16);
                    self.a ^= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_EOR_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr.wrapping_add(self.x) as u16);
                    self.a ^= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_EOR_ABS => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr);
                    self.a ^= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_EOR_ABX => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr + self.x as u16);
                    self.a ^= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_EOR_ABY => {
                    let indirect: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indexed_indirect_addr(indirect);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.a ^= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_EOR_IDX => {
                    let indirect: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indexed_indirect_addr(indirect);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.a ^= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_EOR_IDY => {
                    let indirect_addr: u8 = self.next_byte();
                    let final_addr: u16 = self.parse_indirect_indexing_addr(indirect_addr);
                    let value: u8 = self.read_byte_from_addr(final_addr);
                    self.a ^= value;
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) > 0;
                }

                P6502::INS_BIT_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    let value: u8 = self.read_byte_from_addr(zp_addr as u16);
                    self.flags.z = value & self.a == 0;
                    self.flags.v = (value & 0b01000000) >> 6 != 0;
                    self.flags.n = (value & 0b10000000) >> 7 != 0;
                }

                P6502::INS_BIT_ABS => {
                    let addr: u16 = self.next_word();
                    let value: u8 = self.read_byte_from_addr(addr);
                    self.flags.z = value & self.a == 0;
                    self.flags.v = (value & 0b01000000) >> 6 != 0;
                    self.flags.n = (value & 0b10000000) >> 7 != 0;
                }

                P6502::INS_ASL_ACC => {
                    let msb: u8 = (self.a & 0b10000000) >> 7;
                    self.flags.c = msb != 0;
                    self.a = self.a.wrapping_mul(2);
                    self.flags.z = self.a == 0;
                    self.flags.n = (self.a & 0b10000000) != 0;
                }
                P6502::INS_ASL_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    self.shift_left_memory(zp_addr as u16);
                }
                P6502::INS_ASL_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    self.shift_left_memory(zp_addr.wrapping_add(self.x) as u16);
                    self.clock_tick(1);
                }

                P6502::INS_ASL_ABS => {
                    let addr: u16 = self.next_word();
                    self.shift_left_memory(addr);
                }

                P6502::INS_ASL_ABX => {
                    let addr: u16 = self.next_word();
                    self.shift_left_memory(addr + self.x as u16);
                    self.clock_tick(1);
                }

                P6502::INS_LSR_ACC => {
                    self.flags.c = self.a & 1 != 0;
                    self.a >>= 1;
                    self.a &= !(1 << 7);
                    self.flags.z = self.a == 0;
                    self.flags.n = self.a & (1 << 7) != 0;
                    self.clock_tick(1);
                }

                P6502::INS_LSR_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    self.logical_shift_right_memory(zp_addr as u16);
                }

                P6502::INS_LSR_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    self.logical_shift_right_memory(zp_addr.wrapping_add(self.x) as u16);
                    self.clock_tick(1);
                }

                P6502::INS_LSR_ABS => {
                    let addr: u16 = self.next_word();
                    self.logical_shift_right_memory(addr);
                }

                P6502::INS_LSR_ABX => {
                    let addr: u16 = self.next_word();
                    self.logical_shift_right_memory(addr + self.x as u16);
                    self.clock_tick(1);
                }

                P6502::INS_ROL_ACC => {
                    let old_carry: bool = self.flags.c;
                    let new_carry: bool = (self.a & 0b10000000) >> 7 != 0;
                    self.a = self.a.rotate_left(1);
                    if old_carry {
                        self.a |= 1;
                    } else {
                        self.a &= 0b11111110;
                    }
                    self.flags.c = new_carry;
                    self.flags.z = self.a == 0;
                    self.clock_tick(1);
                }

                P6502::INS_ROL_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    self.rotate_left_memory(zp_addr as u16);
                }

                P6502::INS_ROL_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    self.rotate_left_memory(zp_addr.wrapping_add(self.x) as u16);
                }

                P6502::INS_ROL_ABS => {
                    let addr: u16 = self.next_word();
                    self.rotate_left_memory(addr);
                }

                P6502::INS_ROL_ABX => {
                    let addr: u16 = self.next_word();
                    self.rotate_left_memory(addr + self.x as u16);
                }

                P6502::INS_ROR_ACC => {
                    let old_carry: bool = self.flags.c;
                    let new_carry: bool = self.a & 1 != 0;
                    self.a = self.a.rotate_right(1);
                    if old_carry {
                        self.a |= 1 << 7;
                    } else {
                        self.a &= !(1 << 7);
                    }
                    self.flags.z = self.a == 0;
                    self.flags.c = new_carry;
                    self.flags.n = self.a & 0b10000000 != 0;
                }

                P6502::INS_ROR_ZP0 => {
                    let zp_addr: u8 = self.next_byte();
                    self.rotate_right_memory(zp_addr as u16);
                }

                P6502::INS_ROR_ZPX => {
                    let zp_addr: u8 = self.next_byte();
                    self.rotate_right_memory(zp_addr.wrapping_add(self.x) as u16);
                    self.clock_tick(1);
                }

                P6502::INS_ROR_ABS => {
                    let addr: u16 = self.next_word();
                    self.rotate_right_memory(addr);
                }

                P6502::INS_ROR_ABX => {
                    let addr: u16 = self.next_word();
                    self.rotate_right_memory(addr + self.x as u16);
                    self.clock_tick(1);
                }

                P6502::INS_SEC => {
                    self.flags.c = true;
                    self.clock_tick(1);
                }

                P6502::INS_SED => {
                    self.flags.d = true;
                    self.clock_tick(1);
                }

                P6502::INS_SEI => {
                    self.flags.i = true;
                    self.clock_tick(1);
                }

                P6502::INS_CLC => {
                    self.flags.c = false;
                    self.clock_tick(1);
                }

                P6502::INS_CLD => {
                    self.flags.d = false;
                    self.clock_tick(1);
                }

                P6502::INS_CLI => {
                    self.flags.i = false;
                    self.clock_tick(1);
                }

                P6502::INS_CLV => {
                    self.flags.v = false;
                    self.clock_tick(1);
                }

                _ => {
                    panic!("Unhandled opcode detected ${:02x}", instruction);
                }
            }
        }
    }
    /// (3 C) Increments the value of a specified memory location. Wrapps when necesary.
    fn increment_memory(&mut self, addr: u16) {
        let current_value: u8 = self.read_byte_from_addr(addr as u16);
        let new_value: u8 = current_value.wrapping_add(1);
        self.write_byte_to_addr(addr as u16, new_value);
        self.flags.z = new_value == 0;
        self.flags.n = (new_value & 0b10000000) > 0;
        self.clock_tick(1);
    }

    /// (3 C) Decrements the value of a specified memory location. Wrapps when necesary.
    fn decrement_memory(&mut self, addr: u16) {
        let current_value: u8 = self.read_byte_from_addr(addr as u16);
        let new_value: u8 = current_value.wrapping_sub(1);
        self.write_byte_to_addr(addr as u16, new_value);
        self.flags.z = new_value == 0;
        self.flags.n = (new_value & 0b10000000) > 0;
        self.clock_tick(1);
    }

    /// (0 C) Performs the addition with carry in the accumulator with another value and
    /// sets all flags appropriately.
    fn add_with_carry(&mut self, value: u8) {
        let acc_sign: u8 = self.a & 0b10000000;
        let value_sign: u8 = value & 0b10000000;
        let mut total_addition: u16 = self.a as u16 + value as u16 + self.flags.c as u16;

        if total_addition < 256 {
            self.a = total_addition as u8;
        } else {
            self.flags.c = true;
            total_addition -= 256;
            self.a = total_addition as u8;
        }

        self.flags.z = self.a == 0;
        self.flags.n = (self.a & 0b10000000) > 0;

        if acc_sign == value_sign && acc_sign != total_addition as u8 & 0b10000000 {
            self.flags.v = true;
        }
    }

    /// (0 C) Performs the substraction with carry in the accumulator with another value and
    /// sets all flags appropriately.
    fn substract_with_carry(&mut self, value: u8) {
        let c2value: u8 = !value;
        self.add_with_carry(c2value);
    }

    /// (3 C) Rotates to the left by a factor of one the contents of a memory location.
    fn rotate_left_memory(&mut self, addr: u16) {
        let mut value: u8 = self.read_byte_from_addr(addr);
        let old_carry: bool = self.flags.c;
        let new_carry: bool = (value & 0b10000000) >> 7 != 0;
        value = value.rotate_left(1);
        if old_carry {
            self.a |= 1;
        } else {
            self.a &= !1;
        }
        self.write_byte_to_addr(addr, value);
        self.flags.z = value == 0;
        self.flags.c = new_carry;
        self.flags.n = value & 0b10000000 != 0;
        self.clock_tick(1);
    }

    /// (3 C) Rotates to the right by a factor of one the contents of a memory location.
    fn rotate_right_memory(&mut self, addr: u16) {
        let mut value: u8 = self.read_byte_from_addr(addr);
        let old_carry: bool = self.flags.c;
        let new_carry: bool = value & 1 != 0;
        value = value.rotate_right(1);
        if old_carry {
            value |= 1 << 7;
        } else {
            value &= !(1 << 7);
        }
        self.write_byte_to_addr(addr, value);
        self.flags.z = value == 0;
        self.flags.c = new_carry;
        self.flags.n = value & 0b10000000 != 0;
        self.clock_tick(1);
    }

    /// (3 C) Shifts left by a factor of two the contents of a specified memory location.
    fn shift_left_memory(&mut self, addr: u16) {
        let current_value: u8 = self.read_byte_from_addr(addr);
        let msb: u8 = (current_value & 0b10000000) >> 7;
        self.flags.c = msb != 0;
        let new_value: u8 = current_value.wrapping_mul(2);
        self.write_byte_to_addr(addr, new_value);
        self.flags.z = new_value == 0;
        self.flags.n = (new_value & 0b10000000) != 0;
        self.clock_tick(1);
    }

    /// (3 C) Performs the LSR on a memory location.
    fn logical_shift_right_memory(&mut self, addr: u16) {
        let mut value: u8 = self.read_byte_from_addr(addr);
        self.flags.c = value & 1 != 0;
        value >>= 1;
        value &= !(1 << 7);
        self.flags.z = value == 0;
        self.flags.n = value & (1 << 7) != 0;
        self.write_byte_to_addr(addr, value);
        self.clock_tick(1);
    }

    /// Performs a relative jump from a signed 8-bit address.
    fn relative_jump(&mut self, jump_addr: u8) {
        let positive_sign: bool = jump_addr & 0b10000000 == 0; // & 1 << 7
        let old_pc: u16 = self.pc;

        if positive_sign {
            self.pc += (jump_addr & 0b0111111) as u16;
        } else {
            let mut new_addr: u8 = jump_addr & 0b011111111; // !(1 << 7)
            new_addr = !new_addr + 1;
            self.pc -= new_addr as u16;
        }

        // Check if the old page and the new one match.
        if old_pc & 0xFF00 != self.pc & 0xFF00 {
            self.clock_tick(2);
        } else {
            self.clock_tick(1);
        }
    }
    /// (2 C) Parses a (ADDR),Y indirect indexing operand.
    fn parse_indirect_indexing_addr(&mut self, indirect: u8) -> u16 {
        let indirect_value: u8 = self.read_byte_from_addr(indirect as u16);
        let lsb: u8 = self.y.wrapping_add(indirect_value);
        let carry: bool = self.y as u16 + indirect_value as u16 > 255;
        let msb: u8 = self
            .read_byte_from_addr((indirect as u16).wrapping_add(1))
            .wrapping_add(carry as u8);
        (msb as u16) << 8 | lsb as u16
    }

    /// (3 C) Parses a (ADDR,X) indexed indirect operand.
    fn parse_indexed_indirect_addr(&mut self, indirect_addr: u8) -> u16 {
        self.clock_tick(1);
        let addr: u16 = self.x.wrapping_add(indirect_addr) as u16;
        self.read_word_from_addr(addr)
    }

    /// Keeps track of the CPU's clock ticks.
    fn clock_tick(&mut self, ticks: usize) {
        self.cycles += ticks;
    }

    /// Sets the memory to another
    pub fn set_memory(&mut self, memory: Memory) {
        self.memory = memory;
    }

    /* Stack operations */

    /// Pushes one byte onto the stack and decrements the Stack Pointer.
    fn push_byte(&mut self, data: u8) {
        self.write_byte_to_addr(P6502::STACK_PAGE + self.sp as u16, data);
        self.clock_tick(1);
        self.sp = self.sp.saturating_sub(1);
    }

    /// Pulls one byte from the stack and increments the Stack Pointer.
    fn pull_byte(&mut self) -> u8 {
        self.sp = self.sp.saturating_add(1);
        let value: u8 = self.read_byte_from_addr(P6502::STACK_PAGE + self.sp as u16);
        self.clock_tick(1);
        value
    }

    /// Pushes two bytes onto the stack (the MSB first, then the LSB) and
    /// decrements two times the Stack Pointer.
    fn push_word(&mut self, data: u16) {
        let msb: u8 = ((data & 0xFF00) >> 8) as u8;
        let lsb: u8 = (data & 0x00FF) as u8;

        self.push_byte(msb);
        self.push_byte(lsb);
    }

    /// Pulls two bytes from the stack (the LSB first, then the MSB) and
    /// increments two times the Stack Pointer.
    fn pull_word(&mut self) -> u16 {
        let lsb: u8 = self.pull_byte();
        let msb: u8 = self.pull_byte();

        (msb as u16) << 8 | lsb as u16
    }

    /* Instruction fetching */

    /// Reads the next byte from memory and increments one time the Program Counter.
    fn next_byte(&mut self) -> u8 {
        let data: u8 = self.read_byte_from_addr(self.pc);
        self.pc = self.pc.saturating_add(1);
        data
    }
    /// Reads the next two bytes from memory and increments two times the Program Counter.
    fn next_word(&mut self) -> u16 {
        let data: u16 = self.read_word_from_addr(self.pc);
        self.pc = self.pc.saturating_add(2);
        data
    }

    /* Basic read and write memory functions */

    /// (1 C) Reads and returns the content of the specified memory location.
    fn read_byte_from_addr(&mut self, addr: u16) -> u8 {
        let data: u8 = self.memory.read(addr as usize);
        self.clock_tick(1);
        data
    }

    /// (2 C) Reads and returns as one 16-bit number the contents of two
    /// contiguous memory locations
    fn read_word_from_addr(&mut self, addr: u16) -> u16 {
        let mut data: u16 = self.memory.read(addr as usize) as u16;
        //self.pc = self.pc.saturating_add(1);
        data |= (self.memory.read((addr + 1) as usize) as u16) << 8;

        self.clock_tick(2);
        data
    }

    /// (1 C) Writes one byte to the specified memory location.
    fn write_byte_to_addr(&mut self, addr: u16, data: u8) {
        self.memory.write(addr as usize, data);
        self.clock_tick(1);
    }

    pub fn interactive(&self) {
        let mut copper: Copperline = Copperline::new();

        loop {
            let line = copper.read_line_utf8("]] ");
            match line {
                Ok(contents) => {
                    let line_as_vec: Vec<&str> = contents.split_whitespace().collect();

                    match *line_as_vec.get(0).unwrap_or(&"") {
                        "" => {}
                        "mem" | "memory" => {
                            let start: usize =
                                usize::from_str_radix(line_as_vec.get(1).unwrap_or(&"0"), 16)
                                    .unwrap();
                            let end: usize =
                                usize::from_str_radix(line_as_vec.get(2).unwrap_or(&"0"), 16)
                                    .unwrap();
                            println!("Listing memory from ${:x} to ${:x}:", start, end);

                            self.memory.monitor(start, end);
                        }

                        "clear" => copper.clear_screen().unwrap(),
                        "help" => {
                            println!("Refer to the 'Using the interactive prompt' in the README file for more help. Basic commands are:");
                            println!("\tmonitor START [END]: lists the memory contents of the specified addresses.");
                            println!("\tstatus: outputs the values stored in the processor's registers, the status flags and the program counter.");
                            println!("\tclear: clears the screen.");
                            println!("\texit | quit: terminates this utility.");
                        }
                        "status" | "stat" => println!("{}", self.status()),
                        "exit" | "quit" => break,
                        _ => {}
                    }
                }

                Err(err) => match err {
                    copperline::Error::Cancel | copperline::Error::EndOfFile => {
                        println!("Type \"exit\" or \"quit\" to exit.");
                    }
                    _ => {}
                }
            }
        }
    }

    /// (0 C) Resets the CPU by:
    /// - Erasing the memory (all memory is set to `0_u8`).
    /// - Setting the Program Counter to the default program start address.
    /// - Setting the Stack Pointer to $FF (initial position).
    /// - Setting all flags to zero (boolean false) except for the B-flag which is set to `true`.
    pub fn reset(&mut self) {
        self.pc = Self::PROGRAM_START;
        self.flags = P6502Flags::default();
        self.flags.b = true;
        self.sp = 0xFF;
        self.memory.clear();
    }
}

/// Represents the computer's memory. Consistists only of an `u8` slice of fixed length
/// and equal to 64 K (= 65536 = $10000).
pub struct Memory {
    pub data: [u8; Memory::MAX_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            data: [0_u8; Self::MAX_SIZE],
        }
    }
}

impl Memory {
    const MAX_SIZE: usize = 64 * 1024;

    /// Clears the memory by setting all the elements of the slice to `0_u8`.
    pub fn clear(&mut self) {
        self.data = [0_u8; Memory::MAX_SIZE];
    }

    /// Writes a byte to the specified memory address.
    pub fn write(&mut self, addr: usize, data: u8) {
        if addr < Memory::MAX_SIZE {
            self.data[addr] = data;
        }
    }

    /// Reads a byte from the specified memory address and returns the value.
    pub fn read(&self, addr: usize) -> u8 {
        if addr < Memory::MAX_SIZE {
            self.data[addr]
        } else {
            0
        }
    }

    pub fn from_program_vec(program: Vec<u8>) -> Self {
        let mut memory: Self = Self::default();

        for (index, value) in program.iter().enumerate() {
            memory.write(P6502::PROGRAM_START as usize + index, *value);
        }

        memory
    }

    /// Prints to stdout the contents of an specified memory area in hexadecimal format.
    pub fn monitor(&self, start: usize, end: usize) {
        if end == 0 {
            print!("${:04x}: ", start);
            for j in 0..16 as usize {
                print!("{:02x} ", self.data[start + j]);
            }
        } else {
            for i in (start..end).step_by(16) {
                print!("${:04x}: ", i);
                for j in 0..16 as usize {
                    print!("{:02x} ", self.data[i + j]);
                }
                println!("");
            }
        }

        println!("");
    }
}
