//削除の実行

use std::{fs, io};

use crate::{
    file::{rc_file::read_rc, target_file::read_target_file_line},
    types::Target,
};

pub fn check_target_dir(target: &Target) -> Result<(), io::Error> {
    //削除対象ファイルのすり替わりチェック
    let id = read_target_file_line(&target.path)?;

    //ファイル内idチェック
    if id.eq(&target.uuid) {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "設定ファイルのIDが不正です",
        ))
    }
}

fn remove_in_dir(target: &Target) -> io::Result<()> {
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
        remove_in_dir(&rc_line)?;
    }

    Ok(())
}
