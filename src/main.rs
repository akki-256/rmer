use std::io;

mod comfig;
mod run;
mod types;
fn main() -> io::Result<()> {
    let rc_vec = comfig::get_rc()?;

    for rc_line in rc_vec.iter() {
        run::remove_in_dir(rc_line)?;
    }

    Ok(())
}
