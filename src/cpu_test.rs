
use super::*;


#[test]
fn test_jump(){
    let mut c = CPU::new();
    // Instruction: 0x1055
    // Jump to 0x55
    c.mem[0] = 0x10;
    c.mem[1] = 0x55;   
    c.execute_insn();
    assert_eq!(c.pc, 0x55);
}

#[test]
fn test_call() {
    let mut c = CPU::new();
    // Instruction: 0x2134
    // Call subroutine at 0x134
    c.mem[0] = 0x21;
    c.mem[1] = 0x34;
    c.execute_insn();
    assert_eq!(c.pc, 0x134);
    assert_eq!(c.sp, SP_BOTTOM + 2);
    assert_eq!(c.mem[c.sp], 0x0);
    assert_eq!(c.mem[c.sp + 1], 0x2);
}

#[test]
fn test_ret() {
    let mut c = CPU::new();
    // Instruction: 0x2134
    // Call subroutine at 0x134
    c.mem[0] = 0x21;
    c.mem[1] = 0x34;

    // The subroutine has only one instruction: a "ret".
    c.mem[0x134] = 0x00;
    c.mem[0x135] = 0xee;

    c.execute_insn(); // call 0x134   
    c.execute_insn(); // ret

    assert_eq!(c.sp, SP_BOTTOM);
    assert_eq!(c.pc, 0x2);    
}

#[test]
fn test1_skip_if_vx_eq_nn() {
    let mut c = CPU::new();
    // Instruction: 0x3a24
    // Skip next instruction if self.v[0xa] == 0x24

    c.v[0xa] = 0x24;
    c.mem[0] = 0x3a;
    c.mem[1] = 0x24;

    c.execute_insn();
    assert_eq!(c.pc, 0x4);
}

#[test]
fn test2_skip_if_vx_eq_nn() {
    let mut c = CPU::new();
    // Instruction: 0x3a24
    // Skip next instruction if self.v[0xa] == 0x24

    c.v[0xa] = 0x8;
    c.mem[0] = 0x3a;
    c.mem[1] = 0x24;

    c.execute_insn();
    assert_eq!(c.pc, 0x2);
}

#[test]
fn test1_skip_if_vx_ne_nn() {
    let mut c = CPU::new();
    // Instruction: 0x4a24
    // Skip next instruction if self.v[0xa] != 0x24

    c.v[0xa] = 0x8;
    c.mem[0] = 0x4a;
    c.mem[1] = 0x24;

    c.execute_insn();
    assert_eq!(c.pc, 0x4);
}

#[test]
fn test2_skip_if_vx_ne_nn() {
    let mut c = CPU::new();
    // Instruction: 0x4a24
    // Skip next instruction if self.v[0xa] == 0x24

    c.v[0xa] = 0x24;
    c.mem[0] = 0x4a;
    c.mem[1] = 0x24;

    c.execute_insn();
    assert_eq!(c.pc, 0x2);
}

#[test]
fn test1_skip_if_vx_eq_vy() {
    let mut c = CPU::new();
    // Instruction: 0x52b0
    // Skip next instruction if self.v[0x2] == self.v[0xb]

    c.v[0x2] = 0xff;
    c.v[0xb] = 0xff;
    c.mem[0] = 0x52;
    c.mem[1] = 0xb0;

    c.execute_insn();
    assert_eq!(c.pc, 0x4);
}

#[test]
fn test2_skip_if_vx_eq_vy() {
    let mut c = CPU::new();
    // Instruction: 0x52b0
    // Skip next instruction if self.v[0x2] == self.v[0xb]

    c.v[0x2] = 0xff;
    c.v[0xb] = 0xfe;
    c.mem[0] = 0x52;
    c.mem[1] = 0xb0;

    c.execute_insn();
    assert_eq!(c.pc, 0x2);
}

#[test]
fn test_set_vx_to_nn() {
    let mut c = CPU::new();
    // Instruction: 0x6c2b
    // Set self.v[0xc] to 0x2b

    c.mem[0] = 0x6c;
    c.mem[1] = 0x2b;
    
    c.execute_insn();
    assert_eq!(c.v[0xc], 0x2b);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_add_nn_to_vx() {
    let mut c = CPU::new();
    // Instruction: 0x7405
    // Add 0x5 to self.v[0x4] without changing carry.

    let t = c.v[0xf]; 
    c.v[0x4] = 255;
    c.mem[0] = 0x74;
    c.mem[1] = 0x05;

    c.execute_insn();
    assert_eq!(t, c.v[0xf]);
    assert_eq!(c.v[0x4], 4);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_assign_vy_to_vx() {
    let mut c = CPU::new();
    // Instruction: 0x82b0
    // v[2] = v[0xb]

    c.v[2] = 10;
    c.v[0xb] = 20;
    c.mem[0] = 0x82;
    c.mem[1] = 0xb0;

    c.execute_insn();
    assert_eq!(c.v[2], c.v[0xb]);    
    assert_eq!(c.v[2], 20);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_assign_vx_or_vy_to_vx() {
    let mut c = CPU::new();
    // Instruction: 0x85c1
    // v[5] = v[5] | v[0xc]

    c.v[5] = 0x8;
    c.v[0xc] = 0x1;
    c.mem[0] = 0x85;
    c.mem[1] = 0xc1;

    c.execute_insn();
    assert_eq!(c.v[5], 0x9);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_assign_vx_and_vy_to_vx() {
    let mut c = CPU::new();
    // Insruction: 0x85c2
    // v[5] = v[5] & v[0xc] 

    c.v[5] = 0x8;
    c.v[0xc] = 0x1;
    c.mem[0] = 0x85;
    c.mem[1] = 0xc2;

    c.execute_insn();
    assert_eq!(c.v[5], 0);
    assert_eq!(c.pc, 2);   
}

#[test]
fn test_assign_vx_xor_vy_to_vx() {
    let mut c = CPU::new();
    // Instruction: 0x85c3
    // v[5] = v[5] ^ v[0xc]

    c.v[5] = 0x9;
    c.v[0xc] = 0x9;
    c.mem[0] = 0x85;
    c.mem[1] = 0xc3;

    c.execute_insn();
    assert_eq!(c.v[5], 0);
    assert_eq!(c.pc, 2);
}

#[test]
fn test1_assign_vx_plus_vy_to_vx() {
    let mut c = CPU::new();
    // Instruction: 0x85c4
    // v[5] = v[5] + v[0xc]

    c.v[5] = 255;
    c.v[0xc] = 10; // Generate overflow
    c.mem[0] = 0x85;
    c.mem[1] = 0xc4;

    c.execute_insn();
    assert_eq!(c.v[5], 9);
    assert_eq!(c.v[0xf], 1);
    assert_eq!(c.pc, 2);
}

#[test]
fn test2_assign_vx_plus_vy_to_vx() {
    let mut c = CPU::new();
    // Instruction: 0x85c4
    // v[5] = v[5] + v[0xc]

    c.v[5] = 245;
    c.v[0xc] = 10; // No overflow
    c.mem[0] = 0x85;
    c.mem[1] = 0xc4;

    c.execute_insn();
    assert_eq!(c.v[5], 255);
    assert_eq!(c.v[0xf], 0);
    assert_eq!(c.pc, 2);
}

#[test] 
fn test1_assign_vx_minus_vy_to_vx() {
    let mut c = CPU::new();
    // Instruction: 0x89d5
    // v[9] = v[9] - v[0xd]

    c.v[9] = 100;
    c.v[0xd] = 90; // No borrow
    c.mem[0] = 0x89;
    c.mem[1] = 0xd5;

    c.execute_insn();
    assert_eq!(c.v[9], 10); 
    assert_eq!(c.v[0xf], 1); 
    assert_eq!(c.pc, 2);
}

#[test] 
fn test2_assign_vx_minus_vy_to_vx() {
    let mut c = CPU::new();
    // Instruction: 0x89d5
    // v[9] = v[9] - v[0xd]

    c.v[9] = 100;
    c.v[0xd] = 110; // Borrow
    c.mem[0] = 0x89;
    c.mem[1] = 0xd5;

    c.execute_insn();
    assert_eq!(c.v[9], 246); 
    assert_eq!(c.v[0xf], 0); 
    assert_eq!(c.pc, 2);
}

#[test]
fn test_shr_vx() {
    let mut c = CPU::new();
    // Instruction: 0x8706
    // v[7] = v[7] >> 1

    c.v[7] = 3;
    c.mem[0] = 0x87;
    c.mem[1] = 0x06;

    c.execute_insn();
    assert_eq!(c.v[7], 1);
    assert_eq!(c.v[0xf], 1);
    assert_eq!(c.pc, 2);
}

#[test]
fn test1_assign_vy_minus_vx_to_vx() {
    let mut c = CPU::new();
    // Instruction: 0x89e7
    // v[9] = v[0xe] - v[0x9]

    c.v[9] = 10;
    c.v[0xe] = 13;
    c.mem[0] = 0x89;
    c.mem[1] = 0xe7;

    c.execute_insn();
    assert_eq!(c.v[9], 3);
    assert_eq!(c.v[0xf], 1);
    assert_eq!(c.pc, 2);

}

#[test]
fn test2_assign_vy_minus_vx_to_vx() {
    let mut c = CPU::new();
    // Instruction: 0x89e7
    // v[9] = v[0xe] - v[0x9]

    c.v[9] = 13;
    c.v[0xe] = 10;
    c.mem[0] = 0x89;
    c.mem[1] = 0xe7;

    c.execute_insn();
    assert_eq!(c.v[9], 253);
    assert_eq!(c.v[0xf], 0);
    assert_eq!(c.pc, 2);

}

#[test]
fn test_shl_vx() {
    let mut c = CPU::new();
    // Instruction: 0x870e
    // v[7] = v[7] << 1

    c.v[7] = 2;
    c.mem[0] = 0x87;
    c.mem[1] = 0x0e;

    c.execute_insn();
    assert_eq!(c.v[7], 4);
    assert_eq!(c.pc, 2);
}