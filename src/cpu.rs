/// cpu.rs

/// References: 
/// (1) <http://devernay.free.fr/hacks/chip8/C8TECH10.HTM>
/// (2) <https://en.wikipedia.org/wiki/CHIP-8>

use std::collections::HashMap;

/// CHIP-8 Memory is 4K bytes in size
const MEM_SIZE: usize = 4096;

/// There are 16 general purpose registers in the CHIP-8,
/// named V0 to VF. VF is used as a flag register in some
/// instructions and it is better to avoid using it for 
/// other purposes.
const NUM_REGS: usize = 16; 

/// The stack has 24 entries, each one 2 bytes long, for a 
/// total size of 48 bytes.
/// 
/// It occupies locations from 0xea0 to 0xeff, both inclusive.
/// 
/// A push operation first increments the stack pointer by 2
/// and then stores a value at the location that is now pointed
/// to by the stack pointer. The first push operation (after reset)
/// should store the value at 0xea0.
const SP_BOTTOM: usize = 0xe9e;

/// The Instruction Pointer type 
type InsnPtr = fn(&mut CPU) -> ();

struct CPU {
    /// 4K Memory. 2 byte objects are stored in big-endian
    /// format.
    mem: [u8; MEM_SIZE],
    
    /// The 16 general purpose registers, 8 bits wide.
    v: [u8; NUM_REGS],

    /// The address register.
    i: u16,

    /// The Program Counter, not directly accessible
    /// from CHIP-8 programs.
    pc: usize,

    /// The Stack Pointer, not directly accessible 
    /// from CHIP-8 programs.
    sp: usize,
} 

impl CPU {

    fn new() -> Self {
        CPU { 
            mem: [0; MEM_SIZE],
            v: [0; NUM_REGS],
            i: 0,
            pc: 0,
            sp: SP_BOTTOM,
        }
    }

    /// Increment the program counter.
    /// 
    /// Each instruction is 2 bytes long, so 
    /// incrementing by 1 will add 2 to the PC.
    fn inc_pc(&mut self, n: usize) {
        self.pc += 2 * n;
    }
    
    /// Return the leftmost nibble of  a 16 bit
    /// instruction stored in big endian order. 
    fn insn_leftmost_nibble(&self) -> u8 {
        (self.mem[self.pc] >> 4) & 0xf
    }

    /// Get the 12 bit memory address encoded as part of 
    /// the instruction.
    /// 
    /// If you have an instruction of the form "1nnn", you
    /// need to get "nnn", the 12 bit address.
    fn get_12bit_nnn(&self) -> usize {
        let address_high_nibble = self.mem[self.pc] & 0xf;
        let address_low_byte = self.mem[self.pc + 1];
        ((address_high_nibble as usize) << 8) + address_low_byte as usize    

    }

    /// Get the 8 bit constant encoded as part of the instruction.
    /// 
    /// If you have an instruction say "7xnn", this function 
    /// returns "nn".
    fn get_8bit_nn(&self) -> u8 {
        self.mem[self.pc + 1] 
    }

    /// Copy 2 bytes from a usize value to top-of-stack.
    /// Value stored on stack is in big endian format.
    fn copy_16bits_to_tos(&mut self, src: usize) {
        self.mem[self.sp] = ((src >> 8) & 0xffusize) as u8;
        self.mem[self.sp + 1] = (src & 0xffusize) as u8;        
    }

    /// Return the 2 byte value taken from top-of-stack.
    fn get_16bits_from_tos(&self) -> usize {
        ((self.mem[self.sp] as usize) << 8) | (self.mem[self.sp + 1] as usize)
    }

    /// Return the lower nibble of the high byte of the 
    /// instruction pointed to by PC.
    fn nibble_x(&self) -> usize {
        (self.mem[self.pc] & 0xf) as usize
    }

    /// Return the high nibble of the low byte of the
    /// instruction pointed to by PC.
    fn nibble_y(&self) -> usize {
        ((self.mem[self.pc + 1] >> 4) & 0xf) as usize
    }

    /// Execute a jump instruction of the form "1nnn"
    /// where nnn represents a memory address.
    fn jmp(&mut self) {
        self.pc = self.get_12bit_nnn();
    }

    /// Call subroutine.
    /// 
    /// The call instruction is of the form "2nnn".
    /// The instruction first increments
    /// the stack pointer by 2 and copies the address of the next
    /// instruction to the new location on the stack. It then sets
    /// the program counter to "nnn".
    fn call(&mut self) {
        let target_address = self.get_12bit_nnn();
        let next_insn_address = self.pc + 2;
        self.sp += 2;
        self.copy_16bits_to_tos(next_insn_address);
        self.pc = target_address;
    }

    /// Subroutine return. Opcode "0x00ee".
    fn ret(&mut self) {
        self.pc = self.get_16bits_from_tos();
        self.sp -= 2;
    }

    /// Skip next instruction if v[x] == nn.
    /// 
    /// This instruction is of the form "3xnn".
    fn skip_if_vx_eq_nn(&mut self) {
        if self.v[self.nibble_x()] == self.get_8bit_nn() {
            self.inc_pc(2); // skip next instruction
            return;
        }
        self.inc_pc(1);
    }

    /// Skip next instruction if v[x] != nn.
    /// 
    /// This instruction is of the form "4xnn".
    fn skip_if_vx_ne_nn(&mut self) {
        if self.v[self.nibble_x()] != self.get_8bit_nn() {
            self.inc_pc(2); // skip next instruction
            return;
        }
        self.inc_pc(1);
    }

    /// Skip the next instruction if v[x] == v[y].
    /// 
    /// This instruction is of the form "5xy0".
    fn skip_if_vx_eq_vy(&mut self) {
        if self.v[self.nibble_x()] == self.v[self.nibble_y()] {
            self.inc_pc(2); // skip next instruction
            return;
        }
        self.inc_pc(1);
    }

    /// Set v[x] to nn.
    /// 
    /// This instruction is of the form "6xnn".
    fn set_vx_to_nn(&mut self) {
        self.v[self.nibble_x()] = self.get_8bit_nn();
        self.inc_pc(1);
    }

    /// Add nn to v[x] without changing carry.
    /// 
    /// This instruction is of the form "7xnn"
    fn add_nn_to_vx(&mut self) {
        self.v[self.nibble_x()] = self.v[self.nibble_x()]
                                      .wrapping_add(self.get_8bit_nn());
        self.inc_pc(1);
    }

    /// Execute the instruction pointed to by the PC.
    fn execute_insn(&mut self) {
        if (self.mem[self.pc] == 0x0) && (self.mem[self.pc + 1] == 0xee) {
            self.ret();
            return;
        }
        let t = self.insn_leftmost_nibble();
        if (t >= 1) && (t <= 7) {
            let insn = INSN_LUT1.get(&t).expect("Bad Instruction");
            insn(self);            
        }         
    }
} 

/// INSN_LUT1 is an instruction lookup table; used for decoding the 
/// instruction based on its leftmost nibble.
lazy_static! {
     static  ref  INSN_LUT1:HashMap<u8, InsnPtr> = hashmap! {
        1 => CPU::jmp as InsnPtr,
        2 => CPU::call as InsnPtr,
        3 => CPU::skip_if_vx_eq_nn as InsnPtr,
        4 => CPU::skip_if_vx_ne_nn as InsnPtr,
        5 => CPU::skip_if_vx_eq_vy as InsnPtr,
        6 => CPU::set_vx_to_nn as InsnPtr,
        7 => CPU::add_nn_to_vx as InsnPtr,
    };
}

#[cfg(test)]
#[path="./cpu_test.rs"]
mod cpu_test;
    


