#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use crate::assembly::*;
use crate::cpu::*;

mod assembly;
mod cpu;
mod sundries;

fn main() {
    let mut cpu = Cpu::default();
    cpu.execute(fibonocci);
    println!("{:#?}", cpu);
}
