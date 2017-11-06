
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