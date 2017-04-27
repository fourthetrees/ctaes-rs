extern crate gcc;
use std::process::Command;
use std::path::Path;

fn main() {
    if !Path::new("dep/ctaes/.git").exists() {
        let cmd = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status()
            .unwrap();
        if !cmd.success() {
            panic!("failed to initialize git submodule: `ctaes`")
        }
    }

    // compile `ctaes` to a static lib.
    gcc::Config::new()
        .file("dep/ctaes/ctaes.c")
        .compile("libctaes.a");
}
