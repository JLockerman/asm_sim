/// This file contains sundry definitions that are needed to help write the code for
/// this example, but are not necessarily useful for understanding how ASM works

pub use std::num::Wrapping;

use crate::assembly::*;

#[macro_export]
macro_rules! asm_stmt {
    ($($instructions:expr;)* =>) => {
        &[
            $($instructions),*
        ]
    };
    ($($instructions:expr;)* => jlz $v:ident $o:expr; $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            JumpIfLessThanZero{offset: $o, value: $v}; => $($stmt)*)
    };
    ($($instructions:expr;)* => puti $v:ident $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            PutInt($v); => $($stmt)*)
    };
    ($($instructions:expr;)* => halt $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            Halt; => $($stmt)*)
    };
    ($($instructions:expr;)* => $d:ident <- $s1:ident + $s2:ident $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            Add{destination: $d, source1: $s1, source2: $s2}; => $($stmt)*)
    };
    ($($instructions:expr;)* => $d:ident <- $s1:ident - $s2:ident $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            Sub{destination: $d, source1: $s1, source2: $s2}; => $($stmt)*)
    };
    ($($instructions:expr;)* => $d:ident <- $s1:ident * $s2:ident $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            Mul{destination: $d, source1: $s1, source2: $s2}; => $($stmt)*)
    };
    ($($instructions:expr;)* => $d:ident <- $s1:ident / $s2:ident $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            Div{destination: $d, source1: $s1, source2: $s2}; => $($stmt)*)
    };
    ($($instructions:expr;)* => $d:ident <- *$s1:ident $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            Load{destination: $d, source: $s1}; => $($stmt)*)
    };
    ($($instructions:expr;)* => $d:ident <- $s1:expr; $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            LoadImmediate{destination: $d, value: $s1}; => $($stmt)*)
    };
    ($($instructions:expr;)* => *$d:ident <- $s1:ident $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            Store{destination: $d, source: $s1}; => $($stmt)*)
    };

}

#[macro_export]
macro_rules! asm {
    ($($stmt:tt)*) => {
        $crate::asm_stmt!(=> $($stmt)*)
    };
}


impl std::ops::Index<Register> for [Wrapping<i64>; 16] {
    type Output = Wrapping<i64>;
    fn index(&self, r: Register) -> &Wrapping<i64> {
        let slice: &[Wrapping<i64>] = &*self;
        match r {
            r0 => &Wrapping(0),
            _ => &slice[r as usize],
        }
    }
}

impl std::ops::IndexMut<Register> for [Wrapping<i64>; 16] {
    fn index_mut(&mut self, r: Register) -> &mut Wrapping<i64> {
        let slice: &mut [Wrapping<i64>] = &mut *self;
        &mut slice[r as usize]
    }
}