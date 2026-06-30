use std::io::{self, Write};

use garrote::{garrote_vm::GarroteVM, lexer::Lexer, printerr};

fn main() {
    loop {
        let mut buf = String::new();
        print!("> ");
        io::stdout().flush().expect("Cannot flush stdout");
        io::stdin().read_line(&mut buf).expect("Cannot read line");

        if buf == "exit" {
            return;
        }

        let mut lexer = Lexer::new(&buf);
        let mut vm = GarroteVM::new(lexer.tokenize());

        match vm.execute() {
            Ok(()) => {
                println!(""); // extra line
            }
            Err(e) => printerr(e),
        }
    }
}
