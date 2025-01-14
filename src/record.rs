use crate::smt::BitVec;

use ruint::aliases::*;
use ruint::Uint;
use z3_ext::ast::Bool;

#[derive(Clone, Debug)]
pub struct MachineRecord<const STACK_ITEM_SZ: u32> {
    pub mem: Option<MemChange>,
    pub stack: Option<StackChange<STACK_ITEM_SZ>>,
    pub pc: (usize, usize),
    pub constraints: Option<Bool<'static>>,
    pub halt: bool,
}

pub type Index = U256;

#[derive(Default, Clone, Debug)]
pub struct MemChange {
    pub touched_slots: Vec<(Index, BitVec<32>)>,
    pub ops_log: Vec<MemOp>,
}
#[derive(Clone, Debug)]
pub enum MemOp {
    Write { idx: Index, val: BitVec<32> },
    Read { idx: Index },
}
#[derive(Clone, Debug)]
pub enum StackOp<const SZ: u32> {
    Push(BitVec<SZ>),
    Pop,
}

#[derive(Default, Clone, Debug)]
pub struct StackChange<const SZ: u32> {
    pub pop_qty: u64,
    pub push_qty: u64,
    pub ops: Vec<StackOp<SZ>>,
}

impl<const SZ: u32> StackChange<SZ> {
    pub fn push(val: BitVec<SZ>) -> Self {
        Self {
            pop_qty: 0,
            push_qty: 1,
            ops: vec![StackOp::Push(val)],
        }
    }

    pub fn with_ops(ops: Vec<StackOp<SZ>>) -> Self {
        let mut pop_qty = 0;
        let mut push_qty = 0;

        ops.iter().for_each(|op| match op {
            StackOp::Push(_) => push_qty += 1,
            StackOp::Pop => pop_qty += 1,
        });

        Self {
            push_qty,
            pop_qty,
            ops,
        }
    }
}

pub fn push<const SZ: u32>(val: BitVec<SZ>) -> StackOp<SZ> {
    StackOp::Push(val)
}
