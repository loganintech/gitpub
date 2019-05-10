use std::env::current_dir;
use std::process::Command;

pub fn add_remote_origin(name: &str, url: &str) -> bool {
    match (current_dir(), can_create_remote(name)) {
        (Ok(dir), true) if dir.ancestors().any(|p| p.join(".git").exists()) => Command::new("git")
            .arg("remote")
            .arg("add")
            .arg(name)
            .arg(url)
            .output()
            .is_ok(),
        _ => false,
    }
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
