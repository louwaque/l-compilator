extern crate failure;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate lalrpop_util;

extern crate structopt;

mod ast;
mod lexer;
mod parser;
use lexer::Lexer;
use parser::Parser;

use failure::{Error, ResultExt};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author = "Loïc Escales <loic.escales@etu.univ-amu.fr>")]
struct Opt {
    /// Affiche les tokens de l'analyse lexicale
    #[structopt(short = "l")]
    lex: bool,

    /// Affiche l'arbre abstrait
    #[structopt(short = "a")]
    ast: bool,

    /// Affiche la table des symboles
    #[structopt(short = "t")]
    symbol_table: bool,

    /// Affiche le code trois adresses
    #[structopt(short = "3")]
    three_address_code: bool,

    /// Affiche le code nasm (actif par defaut)
    #[structopt(short = "n")]
    nasm: bool,

    /// Le fichier l source
    #[structopt(parse(from_os_str))]
    source_file: PathBuf,
}

pub struct App;

impl App {
    pub fn run() -> Result<(), Error> {
        let opt = Opt::from_args();

        let content = std::fs::read_to_string(&opt.source_file)
            .with_context(|_| format!("could not read file {:?}", opt.source_file))?;

        if opt.lex {
            Self::print_lex(&content)?;
        }

        if opt.ast {
            Self::print_ast(&content)?;
        }

        Ok(())
    }

    fn print_lex(content: &str) -> Result<(), Error> {
        print!("{}", Lexer::new(&content).into_lex()?);

        Ok(())
    }

    fn print_ast(content: &str) -> Result<(), Error> {
        let l = Lexer::new(&content);
        let p = Parser::new();

        print!("{:#?}", p.parse(l)?);

        Ok(())
    }
}
