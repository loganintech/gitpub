use std::env::current_dir;
use std::process::Command;

pub fn add_remote(name: &str, url: &str) -> bool {
    if can_create_remote(name) {
        if let Ok(dir) = current_dir() {
            if dir.ancestors().any(|p| p.join(".git").exists()) {
                return Command::new("git")
                    .arg("remote")
                    .arg("add")
                    .arg(name)
                    .arg(url)
                    .output()
                    .is_ok();
            }
        }
    }

    false
}

fn can_create_remote(name: &str) -> bool {
    if let Ok(out) = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg(name)
        .output()
    {
        if let Ok(err) = String::from_utf8(out.stderr) {
            return err.starts_with("fatal"); //If it starts with fatal, that means the remote doesn't exist and we can create one
        }
    }

    false
}
