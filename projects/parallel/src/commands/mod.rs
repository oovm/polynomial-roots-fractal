use crate::{EvaluateError, PolynomialRootsDatabase};
use clap::{Args, Parser, Subcommand};
use std::{ops::RangeInclusive, path::Path};

mod database;
mod evaluate;

/// Doc comment
#[derive(Parser)]
pub struct App {
    #[command(subcommand)]
    command: AppCommand,
}

impl App {
    pub fn run(self) -> Result<(), EvaluateError> {
        match self.command {
            AppCommand::Evaluate(cmd) => cmd.run(),
            AppCommand::Database(cmd) => cmd.run(),
        }
    }
}

/// Doc comment
#[derive(Debug, Subcommand)]
enum AppCommand {
    /// Doc comment
    Evaluate(EvaluateCommand),
    /// Query the database
    Database(DatabaseCommand),
}

#[derive(Debug, Args)]
pub struct EvaluateCommand {
    #[arg(short, long, default_value = "1:4")]
    orders: String,
    #[arg(short, long, default_value = "littlewood")]
    model: String,
}
#[derive(Debug, Args)]
pub struct DatabaseCommand {
    #[arg(short, long, default_value = "1:4")]
    orders: String,
    #[arg(short, long, default_value = "littlewood")]
    model: String,
}
