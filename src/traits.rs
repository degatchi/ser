use crate::instruction::Instruction;
use crate::machine::{ExecBranch, ExecutionSummary};
use crate::memory::Memory;
use crate::record::{Index, MachineRecord, MemChange, StackChange};
use crate::smt::BitVec;
use crate::stack::Stack;
use crate::state::evm::EvmState;
use z3_ext::ast::Bool;

pub trait MachineState<const STACK_ITEM_SZ: u32> {
    type PC;

    fn pc(&self) -> Self::PC;
    fn stack(&self) -> &Stack<32>;
    fn stack_push(&mut self, val: BitVec<STACK_ITEM_SZ>);
    fn stack_pop(&mut self) -> BitVec<STACK_ITEM_SZ>;
    fn mem(&self) -> &Memory;
    fn mem_write(&mut self, idx: Index, val: BitVec<32>);
    fn mem_read(&self, idx: Index) -> BitVec<32>;
    fn stack_apply(&mut self, stack_rec: StackChange<STACK_ITEM_SZ>);
    fn mem_apply(&mut self, mem_rec: MemChange);
}

pub trait Machine<const STACK_ITEM_SZ: u32>: MachineComponent {
    type State: MachineState<STACK_ITEM_SZ>;

    // All possible final states
    fn exec(&mut self) -> Vec<ExecBranch>;
    fn pgm(&self) -> Vec<Instruction>;
    fn state(&self) -> Self::State;
    fn state_ref(&self) -> &Self::State;
    fn state_ref_mut(&mut self) -> &mut Self::State;
}

pub trait MachineInstruction<'ctx, const SZ: u32> {
    fn exec(&self, mach: &EvmState) -> MachineRecord<SZ>;
}

pub trait MachineComponent {
    type Record;
    fn apply_change(&mut self, rec: Self::Record);
}
