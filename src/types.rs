use clap::{Parser, Subcommand, ValueHint};
use core::fmt;
use std::path::{self, PathBuf};
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

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    //削除対象となるディレクトリを追加
    Add {
        #[arg(default_value = "./",value_hint=ValueHint::FilePath)]
        dir_path: path::PathBuf,
    },
    //削除の実行
    Run,
    //指定の削除対象のディレクトリを除外する
    Drop {
        #[arg(default_value = "./",value_hint=ValueHint::FilePath)]
        dir_path: PathBuf,
    },
}
