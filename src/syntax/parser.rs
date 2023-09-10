use logos::Logos;
use rowan::GreenNodeBuilder;
use rowan::SyntaxNode;

use super::lexer::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum SyntaxKind {
    TOKEN = 0,
    COMMENT,
    WHITESPACE,
    NEWLINE,
    ERROR,
    PATTERN,
    OWNER,
    ENTRY,
    ROOT,
}
use SyntaxKind::*;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub fn parse(s: &str) -> SyntaxNode<Lang> {
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    enum State {
        TOP,
        INENTRY,
    }
    use State::*;

    let mut builder = GreenNodeBuilder::new();
    let mut lex = Token::lexer(s);
    let mut state = TOP;
    builder.start_node(ROOT.into());
    let mut checkpoint = builder.checkpoint();
    while let Some(token) = lex.next() {
        state = match (state, token) {
            (INENTRY, Ok(Token::Token)) => {
                builder.start_node_at(checkpoint, OWNER.into());
                builder.token(TOKEN.into(), lex.slice());
                builder.finish_node();
                checkpoint = builder.checkpoint();
                INENTRY
            }
            (_, Ok(Token::Token)) => {
                builder.start_node_at(checkpoint, ENTRY.into());
                builder.start_node(PATTERN.into());
                builder.token(TOKEN.into(), lex.slice());
                builder.finish_node();
                checkpoint = builder.checkpoint();
                INENTRY
            }
            (s, Ok(Token::Whitespace)) => {
                builder.token(WHITESPACE.into(), lex.slice());
                s
            }
            (INENTRY, Ok(Token::Newline)) => {
                builder.token(NEWLINE.into(), lex.slice());
                builder.finish_node();
                checkpoint = builder.checkpoint();
                TOP
            }
            (s, Ok(Token::Newline)) => {
                builder.token(NEWLINE.into(), lex.slice());
                checkpoint = builder.checkpoint();
                s
            }
            (INENTRY, Ok(Token::Comment)) => {
                builder.token(COMMENT.into(), lex.slice());
                builder.finish_node();
                checkpoint = builder.checkpoint();
                TOP
            }
            (s, Ok(Token::Comment)) => {
                builder.token(COMMENT.into(), lex.slice());
                s
            }
            (s, Err(_)) => s,
        }
    }
    if matches!(state, INENTRY) {
        builder.finish_node();
    }
    builder.finish_node();
    let green_node = builder.finish();
    SyntaxNode::new_root(green_node)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let text = "";
        let node = parse(text);
        assert_eq!(format!("{node}"), text);
        insta::assert_debug_snapshot!(node);
    }

    #[test]
    fn single() {
        let text = "* @ferris\n";
        let node = parse(text);
        assert_eq!(format!("{node}"), text);
        insta::assert_debug_snapshot!(node);
    }

    #[test]
    fn comments() {
        let text = "# This comments belongs to the entry\n# so does this one\n* @ferris\n";
        let node = parse(text);
        assert_eq!(format!("{node}"), text);
        insta::assert_debug_snapshot!(node);
    }

    #[test]
    fn inline_comment() {
        let text = "* @foo # inline comment\n* @bar\n";
        let node = parse(text);
        assert_eq!(format!("{node}"), text);
        insta::assert_debug_snapshot!(node);
    }

    #[test]
    fn complex() {
        let text = r#"# Here's a comment!

* @ferris @octocat

# Multiple blank lines!


/some/path @octocat
some/other/path/** @octocat

# The End"#;
        let node = parse(text);
        assert_eq!(format!("{node}"), text);
        insta::assert_debug_snapshot!(node);
    }
}
