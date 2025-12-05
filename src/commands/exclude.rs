//削除対象のファイルを対象から外すコマンド

use std::{io, path};

use crate::file::{rc_file, target_file};

fn remove_target_file(path: &path::PathBuf) -> io::Result<()> {
    //ターゲットファイルの削除
    let path = path.join(".rmer_target");
    target_file::delete_target_file(&path)?;

    Ok(())
}

pub fn exclude(path: path::PathBuf) -> io::Result<()> {
    remove_target_file(&path)?;
    rc_file::delete_target_line(&path)?;

    Ok(())
}
