#[derive(Debug, PartialEq)]
pub enum Token {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    Write,
    Read,
    JumpIfZero,
    JumpUnlessZero,
}
use Token::*;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    for char in input.chars() {
        let token = match char {
            '+' => Increment,
            '-' => Decrement,
            '<' => MoveLeft,
            '>' => MoveRight,
            '.' => Write,
            ',' => Read,
            '[' => JumpIfZero,
            ']' => JumpUnlessZero,
            _ => continue,
        };
        tokens.push(token);
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_empty_vector_if_no_instructions_were_passed() {
        let tokens = tokenize("");
        assert_eq!(tokens, vec![])
    }

    #[test]
    fn should_ignore_non_operation_characters() {
        let tokens =
            tokenize("I'm a phrase with many characters that aren't considered operators!");
        assert_eq!(tokens, vec![])
    }

    #[test]
    fn should_return_a_vector_with_tokens() {
        let tokens = tokenize("+-,.[]<>");
        assert_eq!(
            tokens,
            vec![
                Increment,
                Decrement,
                Read,
                Write,
                JumpIfZero,
                JumpUnlessZero,
                MoveLeft,
                MoveRight,
            ]
        )
    }
}
