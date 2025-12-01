//削除の実行

use std::{
    fs, future,
    io::{self, BufRead},
};

use crate::{
    file::{rc_file::read_rc, target_file::check_target_dir},
    types::Target,
};

fn remove_in_dir(target: &Target) -> io::Result<()> {
    println!("remove_in_dir()");
    check_target_dir(target)?;

    let dir = fs::read_dir(&target.path)?;
    for de in dir {
        let de = de?;
        if de.file_name() == ".rmer_target" {
            println!("ターゲットファイル");
            continue;
        };

        if de.path().is_dir() {
            println!("{:?} is dir", de);
            // fs::remove_dir_all(de.path())?;
        } else {
            println!("{:?} is file", de);
            // fs::remove_file(de.path())?;
        }
    }

    Ok(())
}

pub fn run() -> io::Result<()> {
    let rc_vec = read_rc()?;
    for rc_line in rc_vec {
        println!("{}", rc_line);
        remove_in_dir(&rc_line)?;
    }

    Ok(())
}
