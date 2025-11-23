use dirs::home_dir;
use std::{
    fs,
    io::{self, BufRead},
};

use crate::types::RcInfo;

const RC_FILE_NAME: &str = ".rmer_rc";

//設定ファイルの読み込み
pub fn get_rc() -> io::Result<Vec<RcInfo>> {
    //設定ファイルの取得
    let rc_file = (|| -> io::Result<io::BufReader<fs::File>> {
        let home_dir = home_dir().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "home directory is not found ")
        })?;
        let rc_file = fs::File::open(home_dir.join(RC_FILE_NAME))?;

        Ok(io::BufReader::new(rc_file))
    })()
    .expect("can`t get rc file");

    //中身を整形しVecに変換
    let mut rc_vec: Vec<RcInfo> = Vec::new();
    for rc_line in rc_file.lines() {
        let rc_line = rc_line?;
        let mut splits = rc_line.split(",");
        if let (Some(target_path), Some(uuid)) = (splits.next(), splits.next()) {
            let rc_line = RcInfo {
                target_path: String::from(target_path.trim()),
                uuid_str: String::from(uuid.trim()),
            };
            rc_vec.push(rc_line);
        }
    }

    Ok(rc_vec)
}
