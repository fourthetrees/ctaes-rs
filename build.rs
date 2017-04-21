extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/ctaes/ctaes.c")
        .include("src")
        .compile("libctaes.a");
}
