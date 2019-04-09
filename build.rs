extern crate cc;
use std::process::Command;

fn main() {
    Command::new("git")
        .args(&["submodule", "update", "--init"])
        .output()
        .expect("failed to update submodule");
    cc::Build::new()
        .file("src/lava-core/core.c")
        .static_flag(true)
        .compile("libplayer.a");
}
