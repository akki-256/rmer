// #![allow(unused)]
use std::io;

use clap::Parser;

use crate::{
    add::add_target,
    types::{Args, SubCommand},
};

mod add;
mod comfig;
mod run;
mod types;

fn main() -> io::Result<()> {
    let args = Args::parse();
    match args.sub_command {
        SubCommand::Add { dir } => println!("add:{:?}", dir),
        SubCommand::Run => println!("run"),
        SubCommand::Exclude { dir } => println!("exclude:{:?}", dir),
    }

    Ok(())
}
