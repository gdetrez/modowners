use clap::Parser;
use std::fs;
use std::path::Path;

use rowan::ast::AstNode;

mod codemod;
mod syntax;

use syntax::ast::Codeowners;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// Add a code owner
    Add { pattern: String, owner: String },

    /// Remove a code owner
    Remove { pattern: String, owner: String },
}

const CODEOWNERS: &str = ".github/CODEOWNERS";

fn main() {
    let cli = Cli::parse();

    let path = Path::new(CODEOWNERS);
    let before = if path.exists() {
        let content = fs::read_to_string(path).unwrap();
        Codeowners::parse(&content)
    } else {
        Codeowners::default()
    };

    let mut after = before.clone_for_update();
    match cli.cmd {
        Commands::Add { pattern, owner } => codemod::add_owner(&mut after, &pattern, &owner),
        Commands::Remove { pattern, owner } => codemod::remove_owner(&mut after, &pattern, &owner),
    }

    let dir = path.parent().unwrap();
    if !dir.exists() {
        fs::create_dir(dir).unwrap();
    }
    fs::write(path, after.syntax().to_string()).unwrap();
}
