use std::{
    fs,
    io::{self, BufRead},
};

use crate::types::Target;

pub fn check_target_dir(target: &Target) -> Result<(), io::Error> {
    //削除対象ファイルのすり替わりチェック
    let target_path = target.path.join(".rmer_target");
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
