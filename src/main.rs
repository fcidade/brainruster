mod tokenizer;

fn main() {
    let input = "
    ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    ";
    println!("{:?}", tokenizer::tokenize(input));
}
