use std::collections::VecDeque;

use crate::tokens::Instruction::{self};

pub struct GarroteVM {
    instructions: Vec<Instruction>,
    inst_ptr: usize,
    bookmark_stack: Vec<usize>,

    queue: VecDeque<u8>,
}

impl GarroteVM {
    pub fn new(inst: Vec<Instruction>) -> Self {
        Self {
            instructions: inst,
            inst_ptr: 0,
            bookmark_stack: Vec::new(),
            queue: VecDeque::with_capacity(u8::MAX as usize),
        }
    }

    pub fn execute(&mut self) -> Result<(), String> {
        while !self.is_eof() {
            let instruction = &self.instructions[self.inst_ptr];

            match instruction {
                Instruction::Literal(n) => self.enqueue(*n),
                Instruction::Pop => {
                    self.dequeue()?;
                }
                Instruction::Add => {
                    let a = self.dequeue()?;
                    let b = self.dequeue()?;
                    self.enqueue(a.wrapping_add(b));
                    self.queue.make_contiguous().reverse(); // the devil himself 3:)
                }
                Instruction::Sub => {
                    let a = self.dequeue()?;
                    let b = self.dequeue()?;
                    self.enqueue(a.wrapping_sub(b));
                    self.queue.make_contiguous().reverse();
                }
                Instruction::Graft => {
                    let first = match self.peek_first() {
                        Some(v) => *v,
                        None => {
                            return Err(format!(
                                "Graft at instruction [{}] requires the first value in the stack. but none exists.",
                                self.inst_ptr + 1
                            ));
                        }
                    };

                    self.enqueue(first);
                }
                Instruction::Bookmark => {
                    self.bookmark_stack.push(self.inst_ptr + 1); // next instruction after the bookmark itself
                }
                Instruction::JumpIfZero => {
                    if self.bookmark_stack.is_empty() {
                        return Err(format!(
                            "Jump at instruction [{}] requires a bookmark, but none exists.",
                            self.inst_ptr + 1
                        ));
                    }

                    let first_val = match self.peek_first() {
                        Some(v) => Ok(*v),
                        None => Err(format!(
                            "Jump at instruction [{}] requires the first value in the stack. but none exists.",
                            self.inst_ptr + 1
                        )),
                    }?;

                    if first_val != 0 {
                        let bookmark = self.bookmark_stack.last().expect("Bookmark exists here");
                        self.inst_ptr = *bookmark;
                        continue;
                    } else {
                        self.bookmark_stack.pop();
                    }
                }
                Instruction::Display => {
                    let value = match self.peek_first() {
                        Some(v) => *v,
                        None => {
                            return Err(format!(
                                "Display at instruction [{}] requires the first value in the stack. but none exists.",
                                self.inst_ptr + 1
                            ));
                        }
                    } as char;
                    print!("{}", value);
                }
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
    fn dequeue(&mut self) -> Result<u8, String> {
        if self.queue.is_empty() {
            return Err(format!(
                "Queue underflow at instruction [{}]",
                self.inst_ptr + 1
            ));
        }

        Ok(self.queue.pop_front().expect("dequeue should always exist"))
    }

    #[inline(always)]
    pub fn peek_first(&self) -> Option<&u8> {
        self.queue.get(0)
    }
}
