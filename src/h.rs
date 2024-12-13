#[cfg(not(feature = "h"))]
fn main() {}

#[cfg(feature = "h")]
fn main() {
    lame::gen_headers();
}
