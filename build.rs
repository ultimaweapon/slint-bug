use std::path::PathBuf;

fn main() {
    slint_build::compile(PathBuf::from_iter(["ui", "main.slint"])).unwrap();
}
