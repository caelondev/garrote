use core::{iter::Iterator, option::Option::Some};

use crate::tokens::Instruction;

pub struct Lexer {
    src_char: Vec<char>,
    current_pos: usize,
    previous_pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            src_char: source.chars().collect(),
            current_pos: 0,
            previous_pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();

        while !self.is_at_end() {
            self.previous_pos = self.current_pos;

            if let Some(inst) = self.to_token() {
                instructions.push(inst);
            }
        }

        instructions
    }

    fn to_token(&mut self) -> Option<Instruction> {
        let c = self.advance();

        match c {
            '0'..='9' => Some(self.tokenize_literal(c)),
            '#' => Some(Instruction::Pop),
            '+' => Some(Instruction::Add),
            '-' => Some(Instruction::Sub),
            '~' => Some(Instruction::Graft),
            '^' => Some(Instruction::JumpIfZero),
            '%' => Some(Instruction::Bookmark),
            '&' => Some(Instruction::Display),
            _ => None,
        }
    }

    fn tokenize_literal(&mut self, first: char) -> Instruction {
        let mut value: u8 = first as u8 - b'0';

        while !self.is_at_end() && self.is_number(self.peek()) {
            // NOTE: the purpose of "- b'0'" is to offset the character's ASCII value
            // so that it aligns with '0' and produces the actual numeric value.
            value = value * 10 + (self.advance() as u8 - b'0');
        }

        Instruction::Literal(value)
    }

    fn advance(&mut self) -> char {
        let c = self.current_char();
        self.current_pos += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.src_char[self.current_pos]
        }
    }

    fn current_char(&self) -> char {
        self.src_char[self.current_pos]
    }

    fn is_at_end(&self) -> bool {
        self.current_pos >= self.src_char.len()
    }

    fn is_number(&self, c: char) -> bool {
        matches!(c, '0'..='9')
    }
}
