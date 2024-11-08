use lame::*;
use std::env::args;
use std::fs::read_to_string;
use std::fs::write;

fn main() {
    let macros = args()
        .skip(1)
        .map(|path| read_to_string(path).unwrap())
        .map(|rawf| {
            let cstr = std::ffi::CString::new(rawf).unwrap();
            split_parenthesis(Arr {
                data: cstr.as_bytes().as_ptr(),
                len: cstr.count_bytes(),
            })
            .as_slice()
            .iter()
            .map(|rawm| {
                parse(Arr {
                    data: rawm.as_slice().as_ptr(),
                    len: rawm.as_slice().len(),
                })
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let flat = macros.concat();
    let carr = flat.iter().map(|m| m.file).collect::<Vec<_>>();
    compile(Arr {
        data: carr.as_ptr(),
        len: carr.len(),
    });
    let temp = expand(Arr {
        data: flat.as_ptr(),
        len: flat.len(),
    });
    let expanded = temp
        .as_slice()
        .iter()
        .map(Arr::as_slice)
        .collect::<Vec<_>>();
    let unflat = macros
        .iter()
        .map(|ms| ms.len())
        .fold((0, vec![]), |acc, n| {
            (
                n + acc.0,
                vec![
                    acc.1,
                    vec![expanded[(acc.0)..(acc.0 + n)].to_vec().concat()],
                ]
                .concat(),
            )
        })
        .1;
    args()
        .skip(1)
        .enumerate()
        .map(|(i, path)| (path, unflat[i].clone()))
        .for_each(|(path, code)| write(path, code.as_slice()).unwrap());
}
