use std::{
    fs::{self, File},
    io::{self, BufRead},
    path,
    str::FromStr,
};

use uuid::Uuid;

pub fn delete_target_file(path: &path::PathBuf) -> io::Result<()> {
    fs::remove_file(path)?;

    Ok(())
}

pub fn read_target_file_line(path: &path::PathBuf) -> io::Result<Uuid> {
    let target_path = path.join(".rmer_target");
    let target_file = File::open(target_path)?;
    let mut target_file_buf = io::BufReader::new(target_file);

    let mut line = String::new();
    let _ = target_file_buf.read_line(&mut line)?;
    let id_str = line
        .split_once("=")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "設定ファイルが不正です"))?
        .1
        .trim();

    let id = Uuid::from_str(id_str).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(id)
}
