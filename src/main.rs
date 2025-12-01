#![allow(unused)]
use std::io;

use clap::Parser;

use crate::types::{Args, SubCommand};

mod comfig;
mod commands;
mod types;

fn main() -> io::Result<()> {
    let args = Args::parse();
    match args.sub_command {
        SubCommand::Add { path } => commands::add::add_target(path)?,
        SubCommand::Run => commands::run::run()?,
        SubCommand::Exclude { path } => println!("exclude:{:?}", path),
    }

    Ok(())
}
