//削除対象を追加

use std::fs::OpenOptions;
use std::{
    io::{self, Write},
    path,
};

use crate::file::rc_file::{read_rc, write_new_target_rc};
use crate::types::Target;

//対象ファイルの作成
fn add_target_file(target: &Target) -> io::Result<()> {
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&target.path.join(".rmer_target"))?;

    let mut writer_buf = io::BufWriter::new(file);
    writeln!(writer_buf, "id={}", &target.uuid)?;

    Ok(())
}

//現状，ファイルがすり替わった際にはそのことを検知せず，すでに対象であると認識する
fn check_already_target(path: &path::PathBuf) -> io::Result<bool> {
    let rc_file = read_rc()?;
    let found = match rc_file
        .iter()
        .find(|rc_line: &&Target| &rc_line.path == path)
    {
        Some(_) => true,
        None => false,
    };

    Ok(found)
}

pub fn add_target(path: path::PathBuf) -> io::Result<()> {
    let already_target = check_already_target(&path)?;
    if !already_target {
        let target = Target {
            path: path,
            uuid: uuid::Uuid::new_v4(),
        };

        add_target_file(&target)?;
        write_new_target_rc(&target)?;
    } else {
        eprintln!("{}\n This is already target", &path.display());
    }

    Ok(())
}
