/// This file contains sundry definitions that are needed to help write the code for
/// this example, but are not necessarily useful for understanding how ASM works

pub use std::{
    collections::HashMap,
    num::Wrapping,
};

use std::{
    ops::{
        Add as Plus,
        AddAssign,
        Sub as Minus,
        Mul as Times,
        Div as Division,
        Index,
        IndexMut,
    },
};

pub use crate::{
    assembly::{
        *,
        Instruction::*,
        Register::*,
    },
    assembly,
    asm_stmt,
};

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
            LoadImmediate{destination: $d, value: $s1.into()}; => $($stmt)*)
    };
    ($($instructions:expr;)* => *$d:ident <- $s1:ident $($stmt:tt)*) => {
        $crate::asm_stmt!($($instructions;)*
            Store{destination: $d, source: $s1}; => $($stmt)*)
    };

}

#[macro_export]
macro_rules! assembly {
    ($($stmt:tt)*) => {
        $crate::asm_stmt!(=> $($stmt)*)
    };
}


impl Index<Register> for [int64; 16] {
    type Output = int64;
    fn index(&self, r: Register) -> &int64 {
        let slice: &[int64] = &*self;
        match r {
            r0 => &int64(Wrapping(0)),
            _ => &slice[r as usize],
        }
    }
}

impl IndexMut<Register> for [int64; 16] {
    fn index_mut(&mut self, r: Register) -> &mut int64 {
        let slice: &mut [int64] = &mut *self;
        &mut slice[r as usize]
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub struct int64(Wrapping<i64>);

impl int64 {
    pub fn to_bytes(self) -> [u8; 8] {
        (self.0).0.to_le_bytes()
    }

    pub fn to_char(self) -> char {
        (self.0).0 as u8 as char
    }
}

impl Index<int64> for [Instruction] {
    type Output = Instruction;
    fn index(&self, i: int64) -> &Instruction {
        if i < 0i64 {
            panic!("cannot index with negative index {}", i);
        }
        if i >= self.len() {
            panic!("index {} to large for len {}", i, self.len());
        }
        &self[(i.0).0 as usize]
    }
}

impl AddAssign<i16> for int64 {
    fn add_assign(&mut self, rhs: i16) {
        let res = *self + (rhs as i64).into();
        *self = res
    }
}

impl Plus<int64> for int64 {
    type Output = int64;
    fn add(self, rhs: int64) -> int64 {
        int64(self.0 + rhs.0)
    }
}

impl Plus<usize> for int64 {
    type Output = int64;
    fn add(self, rhs: usize) -> int64 {
        use std::convert::TryInto;
        let rhs: i64 = rhs.try_into().expect("overflow");
        let rhs: int64 = rhs.into();
        self + rhs
    }
}

impl Minus<int64> for int64 {
    type Output = int64;
    fn sub(self, rhs: int64) -> int64 {
        int64(self.0 - rhs.0)
    }
}

impl Times<int64> for int64 {
    type Output = int64;
    fn mul(self, rhs: int64) -> int64 {
        int64(self.0 * rhs.0)
    }
}

impl Division<int64> for int64 {
    type Output = int64;
    fn div(self, rhs: int64) -> int64 {
        int64(self.0 / rhs.0)
    }
}

impl From<i64> for int64 {
    fn from(i: i64) -> Self {
        int64(Wrapping(i))
    }
}

impl PartialEq<i64> for int64 {
    fn eq(&self, rhs :&i64) -> bool {
        &(self.0).0 == rhs
    }
}

impl PartialOrd<i64> for int64 {
    fn partial_cmp(&self, rhs: &i64) -> Option<std::cmp::Ordering> {
        (self.0).0.partial_cmp(rhs)
    }
}

impl PartialEq<usize> for int64 {
    fn eq(&self, rhs :&usize) -> bool {
        use std::convert::TryInto;
        &(self.0).0 == rhs.try_into().expect("overflow")
    }
}

impl PartialOrd<usize> for int64 {
    fn partial_cmp(&self, rhs: &usize) -> Option<std::cmp::Ordering> {
        use std::convert::TryInto;
        (self.0).0.partial_cmp(&(*rhs).try_into().expect("overflow"))
    }
}

impl PartialEq<i32> for int64 {
    fn eq(&self, rhs :&i32) -> bool {
        self == &(*rhs as i64)
    }
}

impl PartialOrd<i32> for int64 {
    fn partial_cmp(&self, rhs: &i32) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&(*rhs as i64))
    }
}

impl std::fmt::Display for int64 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", (self.0).0)
    }
}

static mut wyhash16_x: u16 = 0xdeaf;

pub fn random() -> u8 {
    unsafe {
        wyhash16_x += 0xfc15;
        let hash = (wyhash16_x as u32).wrapping_mul(0x2ab);
        (((hash >> 16) ^ hash) & 0xffff) as u8
    }
}
