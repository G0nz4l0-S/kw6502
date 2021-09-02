#[cfg(test)]
use crate::p6502;

#[test]
fn adc_idx() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0xfc, 0x85, 0xb4, 0xa9, 0x1c, 0x85, 0xb5, 0xa9, 0xab, 0x8d, 0xfc, 0x1c, 0xa9, 0xba,
        0xa2, 0x04, 0x61, 0xb0,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();

    assert_eq!(cpu.a, 0x65);
    assert_eq!(cpu.flags.as_binary(), 0b01110001);
}

#[test]
fn sbc_idx() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0xfc, 0x85, 0xb4, 0xa9, 0x1c, 0x85, 0xb5, 0xa9, 0xab, 0x8d, 0xfc, 0x1c, 0xa9, 0xba,
        0xa2, 0x04, 0xe1, 0xb0,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x0e);
    assert_eq!(cpu.flags.as_binary(), 0b00110001);
}

#[test]
fn sbc_imm2() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa9, 0xab, 0xe9, 0x43];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x67);
    assert_eq!(cpu.flags.as_binary(), 0b01110001);
}

#[test]
fn sbc_imm3() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa9, 0x43, 0xe9, 0xab];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x97);
    assert_eq!(cpu.flags.as_binary(), 0b11110000);
}

#[test]
fn sbc_imm() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa9, 0xab, 0xe9, 0x0f];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x9b);
    assert_eq!(cpu.flags.as_binary(), 0b10110001);
}
#[test]
fn lsr_imm() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa9, 0xab, 0x4a];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x55);
    assert_eq!(cpu.flags.as_binary(), 0b00110001);
}

#[test]
fn bit_zp() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa9, 0xab, 0x85, 0x00, 0x24, 0x00];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0xab);
    assert_eq!(cpu.flags.as_binary(), 0b10110000);
}

#[test]
fn dex() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xca];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.x, 0xff);
    assert_eq!(cpu.flags.as_binary(), 0b10110000);
}

#[test]
fn cpx_abs_positive() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa2, 0xbc, 0xa9, 0xaa, 0x8d, 0x32, 0x54, 0xec, 0x32, 0x54, 0x10, 0x02, 0x30, 0x05, 0xa9,
        0xcc, 0x4c, 0x18, 0x06, 0xa9, 0xbb, 0x4c, 0x18, 0x06,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0xcc);
    assert_eq!(cpu.flags.as_binary(), 0b10110001);
}
#[test]
fn cpx_abs_negative() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa2, 0xbc, 0xa9, 0xcb, 0x8d, 0x32, 0x54, 0xec, 0x32, 0x54, 0x10, 0x02, 0x30, 0x05, 0xa9,
        0xff, 0x4c, 0x18, 0x06, 0xa9, 0xbb, 0x4c, 0x18, 0x06,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0xbb);
    assert_eq!(cpu.flags.as_binary(), 0b10110000);
}

#[test]
fn ror_acc() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa9, 0x43, 0xe9, 0xab, 0x6a];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x4b);
    assert_eq!(cpu.flags.as_binary(), 0b01110001);
}

#[test]
fn asl_zp0() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa9, 0x43, 0xe9, 0xab, 0x6a, 0x85, 0x06, 0x06, 0x06];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.memory.read(6), 0x96);
    assert_eq!(cpu.a, 0x4b);
    assert_eq!(cpu.flags.as_binary(), 0b11110000);
}

#[test]
fn ldx_aby() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x43, 0xe9, 0xab, 0x6a, 0x85, 0x06, 0x06, 0x06, 0xa0, 0x06, 0xbe, 0x00, 0x00,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.memory.read(6), 0x96);
    assert_eq!(cpu.a, 0x4b);
    assert_eq!(cpu.y, 0x06);
    assert_eq!(cpu.x, 0x96);
    assert_eq!(cpu.flags.as_binary(), 0b11110000);
}

#[test]
fn iny() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x43, 0xe9, 0xab, 0x6a, 0x85, 0x06, 0x06, 0x06, 0xa0, 0x06, 0xbe, 0x00, 0x00, 0xc8,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.memory.read(6), 0x96);
    assert_eq!(cpu.a, 0x4b);
    assert_eq!(cpu.y, 0x07);
    assert_eq!(cpu.x, 0x96);
    assert_eq!(cpu.flags.as_binary(), 0b01110000);
}

#[test]
fn inc_zp0() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x43, 0xe9, 0xab, 0x6a, 0x85, 0x06, 0x06, 0x06, 0xa0, 0x06, 0xbe, 0x00, 0x00, 0xe6,
        0x06,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.memory.read(6), 0x97);
    assert_eq!(cpu.a, 0x4b);
    assert_eq!(cpu.y, 0x06);
    assert_eq!(cpu.x, 0x96);
    assert_eq!(cpu.flags.as_binary(), 0b11110000);
}

#[test]
fn eor_zpx() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x43, 0xe9, 0xab, 0x6a, 0x85, 0x06, 0x06, 0x06, 0xa0, 0x06, 0xbe, 0x00, 0x00, 0xe6,
        0x06, 0xa2, 0x06, 0x55, 0x00,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0xdc);
    assert_eq!(cpu.y, 0x06);
    assert_eq!(cpu.x, 0x06);
    assert_eq!(cpu.flags.as_binary(), 0b11110000);
}

#[test]
fn and_abx() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x43, 0xe9, 0xab, 0x6a, 0x85, 0x06, 0x06, 0x06, 0xa0, 0x06, 0xbe, 0x00, 0x00, 0xe6,
        0x06, 0xa2, 0x06, 0x55, 0x00, 0x39, 0x00, 0x00,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x94);
    assert_eq!(cpu.y, 0x06);
    assert_eq!(cpu.x, 0x06);
    assert_eq!(cpu.flags.as_binary(), 0b11110000);
}

#[test]
fn jmp_ind() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x01, 0xaa, 0x86, 0x05, 0x69, 0x01, 0x69, 0x01, 0x69, 0x01, 0xaa, 0x86, 0x06, 0x6c,
        0x05, 0x00,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.a, cpu.x);
    assert_eq!(cpu.pc, 0x0401);
}

#[test]
fn cpy_negative() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa0, 0x45, 0xc0, 0xab];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.y, 0x45);
    assert_eq!(cpu.flags.as_binary(), 0b10110000);
}

#[test]
fn cpy_positive() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa0, 0x45, 0xc0, 0x35];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.y, 0x45);
    assert_eq!(cpu.flags.as_binary(), 0b00110001);
}

#[test]
fn cpy_equal() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa0, 0x45, 0xc0, 0x45];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.y, 0x45);
    assert_eq!(cpu.flags.as_binary(), 0b00110011);
}

#[test]
fn clear_flags() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0x18, 0xd8, 0x58, 0xb8];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.flags.as_binary(), 0b00110000);
}

#[test]
fn set_flags() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xf8, 0x38, 0x78];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.flags.as_binary(), 0b00111101);
}

#[test]
fn pha_plp_tsx() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa9, 0xe6, 0x48, 0xba, 0x28];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());
    assert_eq!(cpu.a, 0xe6);
    assert_eq!(cpu.flags.as_binary(), 0b11100110);
    assert_eq!(cpu.x, 0xfe);
}

#[test]
fn jmp_ind_bug() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x40, 0x8d, 0x00, 0x30, 0xa9, 0x80, 0x8d, 0xff, 0x30, 0x6c, 0xff, 0x30,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());
    assert_eq!(cpu.pc, 0x4080);
}

#[test]
fn branch_mul82() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x05, 0x85, 0x00, 0xa9, 0x0a, 0x85, 0x01, 0xa9, 0x00, 0x18, 0x65, 0x00, 0xc6, 0x01,
        0xa6, 0x01, 0xe0, 0x00, 0xd0, 0xf5,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());
    assert_eq!(cpu.a, 0x32);
    assert_eq!(cpu.flags.as_binary(), 0b00110011);
}

#[test]
fn jsr_rts_sum16() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x8a, 0x48, 0xa9, 0x3f, 0x48, 0xa9, 0x24, 0x48, 0xa9, 0xb3, 0x48, 0x20, 0x12, 0x06,
        0x4c, 0x63, 0x06, 0x68, 0x85, 0x00, 0x68, 0x85, 0x01, 0x68, 0x85, 0x02, 0x68, 0x85, 0x03,
        0x68, 0x85, 0x04, 0x68, 0x85, 0x05, 0x18, 0xa5, 0x03, 0x65, 0x05, 0x48, 0xa5, 0x02, 0x65,
        0x04, 0x48, 0xa5, 0x01, 0x48, 0xa5, 0x00, 0x48, 0x60, 0xa9, 0x0f, 0x48, 0xa9, 0x0f, 0x48,
        0x20, 0x44, 0x06, 0xa2, 0x60, 0x4c, 0x63, 0x06, 0x68, 0x85, 0x00, 0x68, 0x85, 0x01, 0x68,
        0xaa, 0x68, 0x85, 0x02, 0xa9, 0x00, 0x18, 0x65, 0x02, 0xca, 0xc8, 0xe0, 0x00, 0xd0, 0xf7,
        0xaa, 0xa5, 0x01, 0x48, 0xa5, 0x00, 0x48, 0x8a, 0x40, 0xa2, 0x50,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());
    assert_eq!(cpu.memory.read(0x01fe), 0xf2);
    assert_eq!(cpu.memory.read(0x01ff), 0xae);
    assert_eq!(cpu.x, 0x50);
    assert_eq!(cpu.flags.as_binary(), 0b00110000);
}

#[test]
fn txy_txs() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xa2, 0x50, 0x8a, 0x9a, 0xa8];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, cpu.x);
    assert_eq!(cpu.x, cpu.y);
    assert_eq!(cpu.a, cpu.sp);
    assert_eq!(cpu.y, 0x50);
}

#[test]
fn tsx_txy() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![0xba, 0x8a, 0xa8];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, cpu.x);
    assert_eq!(cpu.x, cpu.y);
    assert_eq!(cpu.a, cpu.sp);
    assert_eq!(cpu.y, 0xff);
}

#[test]
fn cmp_grater() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x50, 0xc9, 0x40, 0x10, 0x04, 0x30, 0x07, 0xf0, 0x0a, 0xa2, 0xaa, 0x4c, 0x19, 0x06,
        0xa2, 0xbb, 0x4c, 0x19, 0x06, 0xa2, 0xcc, 0x4c, 0x19, 0x06, 0xa0, 0x99,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.x, 0xaa);
    assert_eq!(cpu.y, 0x99);
}

#[test]
fn cmp_smaller() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x50, 0xc9, 0x60, 0x10, 0x04, 0x30, 0x07, 0xf0, 0x0a, 0xa2, 0xaa, 0x4c, 0x19, 0x06,
        0xa2, 0xbb, 0x4c, 0x19, 0x06, 0xa2, 0xcc, 0x4c, 0x19, 0x06, 0xa0, 0x99,
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.x, 0xbb);
    assert_eq!(cpu.y, 0x99);
}

#[test]
fn cmp_equal() {
    let mut cpu: p6502::P6502 = p6502::P6502::default();
    cpu.reset();

    let program: Vec<u8> = vec![
        0xa9, 0x50, 0xc9, 0x50, 0xf0, 0x0e, 0x30, 0x07, 0x10, 0x00, 0xa2, 0xaa, 0x4c, 0x19, 0x06, 
0xa2, 0xbb, 0x4c, 0x19, 0x06, 0xa2, 0xcc, 0x4c, 0x19, 0x06, 0xa0, 0x99
    ];

    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);
    cpu.execute();
    println!("{}", cpu.status());

    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.x, 0xcc);
    assert_eq!(cpu.y, 0x99);
}
