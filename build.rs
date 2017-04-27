extern crate gcc;

fn main() {
    // compile `ctaes` to a static lib.
    gcc::Config::new()
        .file("ctaes/ctaes.c")
        .compile("libctaes.a");
}
