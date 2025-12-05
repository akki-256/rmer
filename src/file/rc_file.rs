//設定ファイル(.rmer_rc)の読み書きの制御

use dirs::home_dir;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Seek, Write},
    path,
    str::FromStr,
};

use crate::types::Target;

const RC_FILE_NAME: &str = ".rmer_rc";

//設定ファイルのアクセス
//設定ファイルがない状態で読み込もうとすると追記ができない
fn get_rc_path() -> io::Result<path::PathBuf> {
    Ok(home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "home directory is not found"))?
        .join(RC_FILE_NAME))
}

fn get_rc_vec(rc_buf_reader: BufReader<&File>) -> io::Result<Vec<Target>> {
    //中身を整形しVecに変換
    let mut rc_vec: Vec<Target> = Vec::new();
    for rc_line in rc_buf_reader.lines() {
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

//設定ファイルの読み込み
pub fn read_rc() -> io::Result<Vec<Target>> {
    let rc_path = get_rc_path()?;
    let rc_file = OpenOptions::new().read(true).open(rc_path)?;
    let rc_reader = io::BufReader::new(&rc_file);

    let rc_vec = get_rc_vec(rc_reader)?;

    Ok(rc_vec)
}

//削除対象ファイルの情報を追記
pub fn write_new_target_rc(target: &Target) -> io::Result<()> {
    let rc_path = get_rc_path()?;
    let rc_file = OpenOptions::new().append(true).create(true).open(rc_path)?;
    let mut buf_writer = io::BufWriter::new(rc_file);

    writeln!(buf_writer, "{}", target)?;

    Ok(())
}

pub fn delete_target_line(path: &path::PathBuf) -> io::Result<()> {
    let rc_path = get_rc_path()?;
    let mut rc_file = OpenOptions::new().read(true).write(true).open(rc_path)?;
    let mut buf_reader = io::BufReader::new(&rc_file);

    let mut rc_data_str = String::new();
    let _ = buf_reader.read_to_string(&mut rc_data_str)?;

    let path_str = path.to_str().expect("");
    let update_data: Vec<&str> = rc_data_str
        .lines()
        .filter(|line| !line.starts_with(path_str))
        .collect();

    rc_file.seek(io::SeekFrom::Start(0))?;

    {
        let mut writer = io::BufWriter::new(&rc_file);
        for line in update_data {
            writeln!(writer, "{}", line)?;
        }
        writer.flush()?;
    }

    let rc_pos = rc_file.stream_position()?;
    rc_file.set_len(rc_pos)?;

    Ok(())
}
