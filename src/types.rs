use clap::{Parser, Subcommand, ValueHint};
use core::fmt;
use std::{
    fs, io,
    path::{self, PathBuf},
};
use uuid::Uuid;

pub struct Target {
    pub path: path::PathBuf,
    pub uuid: Uuid,
}
impl fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.path.display(), self.uuid)
    }
}

//コマンドのフォーマット
///対象のディレクトリを一括でクリーンアップするツールです
#[derive(Parser, Debug)]
#[command(version, disable_help_subcommand = true)]
pub struct Args {
    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    ///削除対象となるディレクトリを追加
    Add {
        #[arg(value_parser=to_abs_path,value_hint=ValueHint::FilePath)]
        dir_path: path::PathBuf,
    },
    ///一括削除の実行
    Run,
    ///削除対象のディレクトリを対象から除外する
    Drop {
        #[arg(value_parser=to_abs_path,value_hint=ValueHint::FilePath)]
        dir_path: PathBuf,
    },
}

fn to_abs_path(path_str: &str) -> io::Result<path::PathBuf> {
    let path_buf = path::PathBuf::from(path_str);
    let abs_path_buf = fs::canonicalize(path_buf)?;

    Ok(abs_path_buf)
}
