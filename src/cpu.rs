
// we have some code in other files so it's not distracting, but we need to
// include it here
use crate::sundries::*;

/// For our simplified purposes, a Cpu contains some memory, and some registers
#[derive(Default, Debug)]
pub struct Cpu {
    /// Memory is effectively just a map from addresses to bytes
    memory: HashMap<int64, u8>,

    /// the registers are 16 64-bit memory locations that instructions can work
    /// on directly
    registers: [int64; 16],
}

impl Cpu {

    /// this function executes a program, input as an array of `Instruction`s
    /// (in a real computer, the program would be stored in the same memory as
    /// the computation, but we'll hand-wave that for now)
    pub fn execute(&mut self, program: &[Instruction]) {

        // we always start execution at instruction 0
        // we keep a index pointing at the current instruction, in real CPUs
        // this is often called the "program counter", and is usually stored in
        // a register
        let mut current_instruction: int64 = 0.into();

        // the main loop reads one instruction at a time and executes it
        loop {

            // if the current instruction would be at a location less than 0,
            // or larger than the amount of instructions we have, that's a bug.
            // in a real CPU wild and wacky things could happen, but we'll just
            // error
            if current_instruction < 0 || current_instruction >= program.len() {
                panic!(
                    "tried to execute instruction {} in a program of length {}.",
                    current_instruction,
                    program.len()
                )
            }

            // the main body of the function, we read one function at a time
            match program[current_instruction] {

                // and execute it!
                // there's one branch for each instruction, we'll annotate Add
                // in more detail
                Add{destination, source1, source2} => {
                    // the arithmetic instructions take three operands, the first
                    // is the destination register, that's where the results are
                    // stored
                    self.registers[destination] =
                        // the other two are source registers, whose values are
                        // used to calculate the results
                        self.registers[source1] + self.registers[source2];
                    // (three-operand instructions happen to be convenient for
                    // compilation purposes)

                    // after most instructions, the current_instruction will be
                    // incremented, so we continue executing at the next
                    // instriction
                    current_instruction += 1;
                }

                // The next annotated instruction will be the loads

                Sub{destination, source1, source2} => {
                    self.registers[destination] = self.registers[source1] - self.registers[source2];
                    current_instruction += 1;
                }

                Mul{destination, source1, source2} => {
                    self.registers[destination] = self.registers[source1] * self.registers[source2];
                    current_instruction += 1;
                }

                Div{destination, source1, source2} => {
                    self.registers[destination] = self.registers[source1] / self.registers[source2];
                    current_instruction += 1;
                }

                // LoadImmediate is almost trivial, it takes a constant value
                // (usually stored as part of the assembly instruction itself)
                // and loads it into a register
                LoadImmediate{destination, value} => {
                    self.registers[destination] = value;
                    current_instruction += 1;
                }

                // Load reads an 8-byte value from memory, and stores it in a
                // register
                Load{destination, source} => {
                    // the value in the register is interperted as an _address_
                    // that the CPU should read from
                    let source_address = self.registers[source];

                    // as far as the cpu is concerned it's just reading 8 bytes,
                    // it's really up to the instructions to decide how these
                    // bytes should be interpreted
                    let mut bytes = [0; 8];
                    for i in 0..8 {
                        // logically, for each byte referred to by the 8-byte
                        // read, the CPU tries to fetch that byte from memory
                        bytes[i] = self.memory.get(&(source_address + i))
                            .cloned()
                            // if the byte hasn't been written to, it could
                            // contain any value
                            .unwrap_or_else(|| random());
                    }

                    // since by convention we're treating values in registers as
                    // signed 64-bit integers, we'll covert the bytes to an
                    // integer here
                    let value = i64::from_le_bytes(bytes).into();
                    self.registers[destination] = value;

                    // as usual, once we're done, increment the instruction
                    // counter!
                    current_instruction += 1;
                }

                // Store is the inverse of load, it stores 8 bytes into memory
                Store{destination, source} => {
                    let value = self.registers[source];
                    let bytes = value.to_bytes();
                    let destination_address = self.registers[destination];
                    for i in 0..8 {
                        self.memory.insert(destination_address + i, bytes[i]);
                    }
                    current_instruction += 1;
                }

                // Our only branching instruction: JumpIfLessThanZero, if this
                // was properly implemented, it could be used to implement any
                // form of branching
                JumpIfLessThanZero{offset, value} => {
                    let val = self.registers[value];
                    // for the most part, it works like it's name, it compares
                    // a value to zero
                    if val < 0 {
                        // If it's less than 0, we add offset to the
                        // current_instruction to get the next instruction we
                        // should execute. If offset is negative, we'll jump
                        // backwards in the instruction-stream, like in a loop!
                        // We can also jump forward if we want to skip some code
                        // like in a if-statement
                        current_instruction += offset
                    } else {
                        // if the value is not less than zero, we go on to the
                        // next instruction, just like any other
                        current_instruction += 1
                    };

                }

                // pseudo-instructions

                PutChar(r) => {
                    print!("{}", self.registers[r].to_char());
                    current_instruction += 1;
                }

                PutInt(r) => {
                    print!("{}", self.registers[r]);
                    current_instruction += 1;
                }

                Halt => {
                    return
                }
            }
        }
    }
}
