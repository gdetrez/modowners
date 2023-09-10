use rowan::ast::AstNode;

use crate::syntax::ast::Codeowners;
use crate::syntax::ast::SyntaxElement;
use crate::syntax::parser;

pub fn add_owner(tree: &mut Codeowners, pattern: &str, owner: &str) {
    for entry in tree.entries() {
        let pattern_node = entry.pattern();
        if pattern_node.string() == pattern {
            let mut pos = pattern_node.syntax().index();
            for owner_node in entry.owners() {
                if owner_node.string() == owner {
                    return;
                }
                pos = owner_node.syntax().index()
            }
            let syntax = entry.syntax();
            let nodes = make_owner(owner);
            pos += 1;
            syntax.splice_children(pos..pos, nodes);
            return;
        }
    }
    let len = tree.syntax().children_with_tokens().count();
    tree.syntax()
        .splice_children(len..len, make_entry(pattern, owner));
}

fn make_entry(pattern: &str, owner: &str) -> Vec<SyntaxElement> {
    let mut root = parser::parse(&format!("{pattern} {owner}\n"));
    root = root.clone_for_update();
    root.descendants_with_tokens().collect()
}

fn make_owner(owner: &str) -> Vec<SyntaxElement> {
    parser::parse(&format!(" {owner}"))
        .clone_for_update()
        .children_with_tokens()
        .collect()
}

pub fn remove_owner(tree: &mut Codeowners, pattern: &str, owner: &str) {
    for entry in tree.entries() {
        if entry.pattern().string() == pattern {
            let mut has_other_owners = false;
            for entry_owner in entry.owners() {
                if entry_owner.string() == owner {
                    entry_owner.syntax().detach();
                } else {
                    has_other_owners = true
                }
            }
            if !has_other_owners {
                entry.syntax().detach();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_new_entry() {
        let mut ast = Codeowners::parse("").clone_for_update();
        add_owner(&mut ast, "*", "@foo");
        assert_eq!(ast.syntax().to_string(), "* @foo\n");
    }

    #[test]
    fn add_complete_existing_entry() {
        let mut tree = Codeowners::parse("* @foo\n").clone_for_update();
        add_owner(&mut tree, "*", "@bar");
        assert_eq!(tree.syntax().to_string(), "* @foo @bar\n");
    }

    #[test]
    fn add_idempotent() {
        let mut tree = Codeowners::parse("* @foo").clone_for_update();
        add_owner(&mut tree, "*", "@foo");
        assert_eq!(tree.syntax().to_string(), "* @foo");
    }

    #[test]
    fn remove_entry() {
        let mut ast = Codeowners::parse("* @foo").clone_for_update();
        remove_owner(&mut ast, "*", "@foo");
        assert_eq!(ast.syntax().to_string(), "");
    }

    #[test]
    fn remove_one_owner() {
        let mut ast = Codeowners::parse("* @foo @bar").clone_for_update();
        remove_owner(&mut ast, "*", "@bar");
        assert_eq!(ast.syntax().to_string(), "* @foo");
    }

    #[test]
    fn remove_idempotent() {
        let mut ast = Codeowners::parse("* @foo").clone_for_update();
        remove_owner(&mut ast, "X", "Y");
        assert_eq!(ast.syntax().to_string(), "* @foo");
    }
}
