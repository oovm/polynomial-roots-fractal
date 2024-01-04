use crate::PolynomialRootsDatabase;
use clap::{builder::ValueRange, Args, Parser, Subcommand, ValueEnum};
use std::{
    ops::{Range, RangeInclusive},
    path::Path,
};

mod evaluate;

/// Doc comment
#[derive(Parser)]
pub struct App {
    #[command(subcommand)]
    command: AppCommand,
}

impl App {
    pub fn run(self) -> Result<(), String> {
        match self.command {
            AppCommand::Evaluate(cmd) => cmd.run(),
        }
    }
}

/// Doc comment
#[derive(Debug, Subcommand)]
enum AppCommand {
    /// Doc comment
    Evaluate(EvaluateCommand),
}

#[derive(Debug, Args)]
pub struct EvaluateCommand {
    #[arg(short, long, default_value = "1:4")]
    orders: String,
    #[arg(short, long, default_value = "littlewood")]
    model: String,
}
