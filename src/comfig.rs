//設定ファイル(.rmer_rc)の読み書きの制御

use dirs::home_dir;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path,
    str::FromStr,
};

use crate::types::Target;

const RC_FILE_NAME: &str = ".rmer_rc";

//設定ファイルのアクセス
//設定ファイルがない状態で読み込もうとすると追記ができない
fn open_rc(append: bool) -> io::Result<File> {
    // println!("open_rc");
    let home_dir = home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "home directory is not found"))?;

    let rc_file: File;
    if append {
        rc_file = OpenOptions::new()
            .append(append)
            .create(true)
            .open(home_dir.join(RC_FILE_NAME))?;
    } else {
        rc_file = OpenOptions::new().read(true).open(RC_FILE_NAME)?;
    }

    Ok(rc_file)
}

//設定ファイルの読み込み
pub fn read_rc() -> io::Result<Vec<Target>> {
    // println!("read_rc");
    let rc_file = open_rc(false)?;
    let rc_reader = io::BufReader::new(rc_file);

    //中身を整形しVecに変換
    let mut rc_vec: Vec<Target> = Vec::new();
    for rc_line in rc_reader.lines() {
        // println!("rc_line");
        let rc_line = rc_line?;
        let mut splits = rc_line.split(",");
        if let (Some(path), Some(uuid)) = (splits.next(), splits.next()) {
            let uuid = match uuid::Uuid::from_str(uuid) {
                Ok(uuid) => uuid,
                Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            };

            let rc_line = Target {
                path: path::PathBuf::from(path),
                uuid: uuid,
            };
            rc_vec.push(rc_line);
        }
    }

    Ok(rc_vec)
}

//削除対象ファイルの情報を追記
pub fn write_new_target_rc(target: &Target) -> io::Result<()> {
    let rc_file = open_rc(true)?;
    let mut buf_writer = io::BufWriter::new(rc_file);

    writeln!(buf_writer, "{}", target)?;

    Ok(())
}
