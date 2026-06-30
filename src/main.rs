use garrote::{garrote_vm::GarroteVM, lexer::Lexer, printerr};

fn main() {
    let mut lexer = Lexer::new("+6 781&");
    let mut vm = GarroteVM::new(lexer.tokenize());

    match vm.execute() {
        Ok(()) => {}
        Err(e) => printerr(e),
    }
}
