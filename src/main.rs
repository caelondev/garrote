use garrote::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("+6 781&");

    println!("{:#?}", lexer.tokenize())
}
