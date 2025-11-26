#![allow(unused)]
use std::{io, path};

mod add;
mod comfig;
mod run;
mod types;

fn main() -> io::Result<()> {
    let path = path::PathBuf::from("/Users/k23003kk/src/rmer/target/debug/sample2dir");
    add::add_target(path)?;

    Ok(())
}
