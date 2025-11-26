//削除の実行

use std::{
    fs,
    io::{self, BufRead},
};

use crate::types::Target;

fn check_target_dir(target: &Target) -> Result<(), io::Error> {
    //削除対象ファイルのすり替わりチェック
    let target_path = target.path.join("rmer_target");
    let target_file = fs::File::open(target_path)?;
    let mut target_file_buf = io::BufReader::new(target_file);

    //設定ファイルのid取得
    let mut line = String::new();
    let _ = target_file_buf.read_line(&mut line);
    let id = line
        .split_once("=")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "設定ファイルが不正です"))?
        .1
        .trim();

    //ファイル内idチェック
    if id.eq(&String::from(target.uuid)) {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "設定ファイルのIDが不正です",
        ))
    }
}

pub fn remove_in_dir(target: &Target) -> io::Result<()> {
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
