
use std::{
    collections::HashMap,
    num::Wrapping,
};

use crate::assembly::*;

#[derive(Default, Debug)]
pub struct Cpu {
    memory: HashMap<i64, u8>,
    registers: [Wrapping<i64>; 16],
}

impl Cpu {
    pub fn execute(&mut self, instructions: &[Instruction]) {
        let mut current_instruction: isize = 0;
        loop {
            if current_instruction < 0 || current_instruction >= instructions.len() as _ {
                panic!("tried to execute instruction {} out of {} instructions", current_instruction, instructions.len())
            }

            match instructions[current_instruction as usize] {
                Add{destination, source1, source2} => {
                    self.registers[destination] = self.registers[source1] + self.registers[source2];
                    current_instruction += 1;
                }
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

                Load{destination, source} => {
                    let source_address = self.registers[source].0;
                    let mut bytes = [0; 8];
                    for i in 0..8 {
                        bytes[i] = self.memory.get(&(source_address + i as i64))
                            .cloned()
                            .unwrap_or(0);
                    }
                    let value = Wrapping(i64::from_le_bytes(bytes));
                    self.registers[destination] = value;
                    current_instruction += 1;
                }

                LoadImmediate{destination, value} => {
                    self.registers[destination] = Wrapping(value);
                    current_instruction += 1;
                }

                Store{destination, source} => {
                    let value: i64 = self.registers[source].0;
                    let bytes = value.to_le_bytes();
                    let destination_address = self.registers[destination].0;
                    for i in 0..8 {
                        self.memory.insert(destination_address + i as i64, bytes[i]);
                    }
                    current_instruction += 1;
                }

                JumpIfLessThanZero{offset, value} => {
                    let val = self.registers[value];
                    current_instruction += if val.0 < 0 { offset as isize } else { 1 };
                }

                PutChar(r) => {
                    println!("{}", self.registers[r].0 as u8 as char);
                    current_instruction += 1;
                }

                PutInt(r) => {
                    println!("{}", self.registers[r].0);
                    current_instruction += 1;
                }

                Halt => {
                    return
                }
            }
        }
    }
}