use std::collections::VecDeque;

use crate::tokens::Instruction;

pub struct GarroteVM {
    instructions: Vec<Instruction>,
    inst_ptr: usize,
    queue: VecDeque<u8>,
}

impl GarroteVM {
    pub fn new(inst: Vec<Instruction>) -> Self {
        Self {
            instructions: inst,
            inst_ptr: 0,
            queue: VecDeque::with_capacity(u8::MAX as usize),
        }
    }

    pub fn execute(&mut self) -> Result<(), String> {
        while !self.is_eof() {
            let instruction = &self.instructions[self.inst_ptr];

            match instruction {
                Instruction::Literal(n) => self.enqueue(*n),
                Instruction::Pop => self.dequeue()?,
                Instruction::Add => {}
                Instruction::Sub => {}
                Instruction::Bookmark => {}
                Instruction::JumpIfZero => {}
                Instruction::Display => {}
            };

            self.inst_ptr += 1;
        }

        Ok(())
    }

    #[inline(always)]
    fn is_eof(&self) -> bool {
        self.inst_ptr >= self.instructions.len()
    }

    #[inline(always)]
    fn enqueue(&mut self, value: u8) {
        self.queue.push_back(value);
    }

    #[inline(always)]
    fn dequeue(&mut self) -> Result<(), String> {
        if self.queue.is_empty() {
            return Err(format!(
                "Queue underflow at instruction [{}]",
                self.inst_ptr + 1
            ));
        }

        self.queue.pop_front().expect("dequeue should always exist");
        Ok(())
    }
}
