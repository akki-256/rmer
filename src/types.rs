use core::fmt;
use std::path;
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
