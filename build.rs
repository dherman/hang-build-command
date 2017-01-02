use std::process::Command;

fn main() {
    println!("build.rs: before");
    Command::new("npm.cmd")
        .current_dir("a-node-package")
        .arg("install")
        .status()
        .ok()
        .expect(r#"failed to run "npm install" in build.rs"#);
    println!("build.rs: after");
}

