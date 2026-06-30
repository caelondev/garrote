use crate::tokens::Instruction;

pub struct GarroteVM {
    instructions: Vec<Instruction>,
    inst_ptr: usize,
}

impl GarroteVM {
    pub fn new(inst: Vec<Instruction>) -> Self {
        Self {
            instructions: inst,
            inst_ptr: 0,
        }
    }

    pub fn execute(&mut self) -> Result<(), String> {
        Ok(())
    }
}
