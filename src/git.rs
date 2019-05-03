use std::path::Path;
use std::process::Command;
use std::{env, fs};

pub struct Git;

impl Git {
    pub fn add_remote_origin(url: &str) -> Result<(), String> {
        if Self::exist_in_path() && Self::exist_repo() {
            Command::new("git")
                .arg("remote")
                .arg("add")
                .arg("origin")
                .arg(url)
                .output()
                .map(|_| ())
                .map_err(|_| "failed run git command".to_owned())
        } else {
            Err("git command or git repo not exist".to_owned())
        }
    }
    fn exist_in_path() -> bool {
        if let Ok(path) = env::var("PATH") {
            for p in path.split(":") {
                let p_str = format!("{}/git", p);
                if fs::metadata(p_str).is_ok() {
                    return true;
                }
            }
        }
        false
    }
    fn exist_repo() -> bool {
        Path::new(".git").exists()
    }
}
