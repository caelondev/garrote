use std::process;

pub mod lexer;
pub mod tokens;

pub fn printerr(msg: String) -> ! {
    eprintln!("error {msg}");
    process::exit(1)
}
