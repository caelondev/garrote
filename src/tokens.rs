#[derive(Debug)]
pub enum Instruction {
    Literal(u8), // n number Literal --
    Pop,         // # --
    Add,         // + --
    Sub,         // - --
    Graft,       // ~ --
    Bookmark,    // % --
    JumpIfZero,  // ^ --
    Display,     // & --
}
