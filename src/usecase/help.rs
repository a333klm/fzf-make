use crate::usecase::usecase_main::Usecase;
use anyhow::Result;

pub struct Help;

impl Help {
    pub fn new() -> Self {
        Self {}
    }
}

impl Usecase for Help {
    fn command_str(&self) -> Vec<&'static str> {
        vec!["--help", "help"]
    }

    fn run(&self) -> Result<()> {
        println!("{}", get_help());
        Ok(())
    }
}

// TODO: Make each command have the following information as a struct, and just display it here.
// Define the vector of usecases in only one place and refer to it.
pub fn get_help() -> String {
    r#"A command line tool that executes make target using fuzzy finder with preview window.

USAGE:
    Run `fzf-make` in the directory where Makefile exists or `fzf-make [SUBCOMMAND]`.

SUBCOMMANDS:
    repeat, --repeat, -r
        Execute the last executed make target.
    history, --history, -h
        Launch fzf-make with the history pane focused.
    help, --help, -h
        Prints help message.
    version, --version, -v
        Prints version information.
    "#
    .to_string()
}
