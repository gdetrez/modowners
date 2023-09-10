use rowan::ast::AstNode;

use super::parser::{self, *};

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

macro_rules! ast_node {
    ($type:ident, $kind:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $type(SyntaxNode);

        impl AstNode for $type {
            type Language = Lang;

            fn can_cast(node: SyntaxKind) -> bool {
                node == SyntaxKind::$kind
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                if Self::can_cast(syntax.kind()) {
                    Some(Self(syntax))
                } else {
                    None
                }
            }

            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
    };
}

ast_node!(Codeowners, ROOT);

impl Codeowners {
    pub fn parse(input: &str) -> Self {
        let syntax = parser::parse(input);
        Self::cast(syntax).unwrap()
    }

    pub fn entries(&self) -> Vec<Entry> {
        self.0.children().filter_map(Entry::cast).collect()
    }
}

impl Default for Codeowners {
    fn default() -> Self {
        Self::parse("")
    }
}

ast_node!(Entry, ENTRY);

impl Entry {
    pub fn pattern(&self) -> Pattern {
        self.0
            .descendants()
            .find_map(Pattern::cast)
            .expect("should have a pattern")
    }

    pub fn owners(&self) -> Vec<Owner> {
        self.0.descendants().filter_map(Owner::cast).collect()
    }
}

ast_node!(Pattern, PATTERN);

impl Pattern {
    pub fn string(&self) -> String {
        for elem in self.0.descendants_with_tokens() {
            if elem.kind() == SyntaxKind::TOKEN {
                return format!("{elem}");
            }
        }
        String::new()
    }
}

ast_node!(Owner, OWNER);

impl Owner {
    pub fn string(&self) -> String {
        for elem in self.0.descendants_with_tokens() {
            if elem.kind() == SyntaxKind::TOKEN {
                return format!("{elem}");
            }
        }
        String::new()
    }
}
