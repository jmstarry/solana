use std::{cell::RefCell, rc::Rc};

use solana_sdk::{
    instruction::{CompiledInstruction, Instruction},
    message::Message,
};

#[derive(Default)]
struct RecorderInner {
    instructions_list: Vec<Vec<Instruction>>,
}

/// Record cross-program invoked instructions to save
/// in transaction meta storage
#[derive(Clone, Default)]
pub struct InvokedInstructionRecorder {
    inner: Rc<RefCell<RecorderInner>>,
}

impl InvokedInstructionRecorder {
    pub fn compile_instructions(&self, message: &Message) -> Vec<Vec<CompiledInstruction>> {
        let recorder = self.inner.borrow();
        recorder.instructions_list.iter().map(|instructions| {
            let compiled: Vec<CompiledInstruction> = instructions.iter().map(|ix| {
                message.compile_instruction(ix)
            }).collect();
            compiled
        }).collect()
    }
}

impl InvokedInstructionRecorder {
    pub fn record_instruction(&self, instruction: Instruction) {
        // .expect("record_start was not called")
        let mut recorder = self.inner.borrow_mut();
        let last_index = recorder.instructions_list.len() - 1;
        if let Some(last) = recorder.instructions_list.get_mut(last_index) {
            last.push(instruction)
        }
    }

    pub fn next_instruction(&self) {
        let mut recorder = self.inner.borrow_mut();
        recorder.instructions_list.push(Vec::new());
    }
}
