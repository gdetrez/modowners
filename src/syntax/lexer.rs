use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
pub enum Token {
    #[regex("#.*\n?")]
    Comment,

    #[token("\n")]
    Newline,

    #[regex(" +")]
    Whitespace,

    #[regex("[^ \n]+")]
    Token,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lex = Token::lexer("# This is a comment\n*   @ferris @octocat\n");

        assert_eq!(lex.next(), Some(Ok(Token::Comment)));
        assert_eq!(lex.slice(), "# This is a comment\n");


        assert_eq!(lex.next(), Some(Ok(Token::Token)));
        assert_eq!(lex.slice(), "*");

        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.slice(), "   ");

        assert_eq!(lex.next(), Some(Ok(Token::Token)));
        assert_eq!(lex.slice(), "@ferris");

        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.slice(), " ");

        assert_eq!(lex.next(), Some(Ok(Token::Token)));
        assert_eq!(lex.slice(), "@octocat");

        assert_eq!(lex.next(), Some(Ok(Token::Newline)));
        assert_eq!(lex.slice(), "\n");

        assert_eq!(lex.next(), None);
    }
}
