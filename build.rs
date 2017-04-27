extern crate gcc;
use std::process::Command;
use std::path::Path;

fn main() {
    // check if `ctaes` has been downloaded.
    if !Path::new("dep/ctaes/.git").exists() {
        // if not, tell git to initialize submodules.
        let cmd = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status()
            .unwrap();
        // if we got a nonzero exit code, don't continue.
        if !cmd.success() {
            panic!("failed to initialize git submodule: `ctaes`")
        }
    }

    // compile `ctaes` into a static lib.
    gcc::compile_library("libctaes.a",&["dep/ctaes/ctaes.c"]);
}
