

pub use crate::{asm, asm_stmt};
pub use Instruction::*;
pub use Register::*;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    /* Add the values in the two source registers and store them in destination.
     * In pseudo-C `destination = source1 + source2`
     */
    Add{destination: Register, source1: Register, source2: Register  },

    /* Subtract the value in the second source register from the one in the first.
     * In pseudo-C `destination = source1 - source2`
     */
    Sub{destination: Register, source1: Register, source2: Register},

    /* Multiply the values in the two source registers and store them in destination.
     * In pseudo-C `destination = source1 + source2`
     */
    Mul{destination: Register, source1: Register, source2: Register},

    /* Divide the value in the first source register by the one in the second.
     * In pseudo-C `destination = source1 / source2`
     */
    Div{destination: Register, source1: Register, source2: Register},

    /* load 8 bytes of memory pointed at by `source` to `destination`.
     * In pseudo-C `destination = *(uint64)source`
     */
    Load{destination: Register, source: Register},

    /* load a 64bit constant value into `destination`.
     * In pseudo-C `destination = value`
     */
    LoadImmediate{destination: Register, value: i64 },

    /* load 8 bytes contained in `source` to the memory pointed at by `destination`.
     * In pseudo-C `*(uint64 *)destination = source`
     */
    Store{destination: Register, source: Register},

    /* if value < 0, jump to current instruction + offset. In pseudo-C
     * `next_instruction = value < 0?
     *     current_instruction + offset: current_instruction + 1`
     */
    JumpIfLessThanZero{ offset: i16, value: Register},

    /* Pseudo-Instructions. In a real computer these would be implemented in
     * terms of more generic primitive operations, but it's convenient to have
     * them available for writing programs, so we'll build them into our CPU
     */

    /* write the register to output as a character (as if we were writing it to
     * a serial port)
     */
    PutChar(Register),

    /* write the register to output as a signed integer */
    PutInt(Register),

    /* ends execution */
    Halt
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    r0 = 0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15
}


pub static sample1: &[Instruction] = asm!{
    r1 <- 100;
    r2 <- 10;
    *r2 <- r1
    r2 <- *r2
    r1 <- r0 + r2
    r1 <- r1 + r1
    r1 <- r1 - r0
    r1 <- r1 * r2
    r1 <- r1 / r3
};

pub static fibonocci: &[Instruction]  = asm! {
    r1 <- 0; // prev
    r2 <- 1; // curr
    r3 <- 0; // i
    r4 <- 6;  // N
    r6 <- r1 + r2 // temp = prev + curr
    r1 <- r2 + r0 // prev = curr
    r2 <- r6 + r0 // curr = temp
    r6 <- 1; // i++
    r3 <- r3 + r6
    r6 <- r3 - r4 // if i > N
    jlz r6 -7;
    puti r2
    r7 <- 1;
    *r7 <- r2 // *(int64 *)1 = curr
    halt
};