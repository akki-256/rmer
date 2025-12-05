// #![allow(unused)]
use std::io;

use clap::Parser;

use crate::types::{Args, SubCommand};

mod commands;
mod file;
mod types;

fn main() -> io::Result<()> {
    let args = Args::parse();
    match args.sub_command {
        SubCommand::Add { dir_path } => commands::add::add_target(dir_path)?,
        SubCommand::Run => commands::run::run()?,
        SubCommand::Drop { dir_path } => commands::exclude::exclude(dir_path)?,
    }

    Ok(())
}
