use lame::*;
use std::env::args;
use std::fs::read_to_string;
use std::fs::write;

fn main() {
    let macros = args()
        .skip(1)
        .map(|path| {
            read_to_string(path + ".lame")
                .unwrap()
                .as_bytes()
                .to_vec()
                .into()
        })
        .collect::<Vec<_>>()
        .into();
    args()
        .skip(1)
        .zip(generate(&macros).iter())
        .for_each(|(path, code)| write(path, code.clone().to_vec()).unwrap());
}
