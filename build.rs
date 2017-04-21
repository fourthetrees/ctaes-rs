extern crate gcc;

fn main() {
    // compile `ctaes` to a static lib.
    gcc::Config::new()
        .file("src/ctaes/ctaes.c")
        .include("src")
        .compile("libctaes.a");
}
