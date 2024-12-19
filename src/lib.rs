use libloading::Library;
use safer_ffi::prelude::*;
use std::{collections::HashMap, process::Command};

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Macro {
    pub name: repr_c::String,
    pub file: repr_c::String,
    pub data: repr_c::Vec<u8>,
}

#[ffi_export]
pub fn parse(data: &repr_c::Vec<u8>) -> Macro {
    let (name, data) = data.split_at(data.iter().position(|c| *c == b'@').unwrap());
    let (file, data) = data.split_at(data.iter().position(|c| *c == b'!').unwrap());
    Macro {
        name: repr_c::String::from(String::from_utf8(name.to_vec()).unwrap()),
        file: repr_c::String::from(String::from_utf8(file[1..].to_vec()).unwrap()),
        data: repr_c::Vec::from(data[1..].to_vec()),
    }
}

#[ffi_export]
pub fn expand(data: &repr_c::Vec<Macro>) -> repr_c::Vec<repr_c::Vec<u8>> {
    let mut files = HashMap::new();
    let mut functions = HashMap::new();
    let mut acc = vec![];
    for datum in data.iter() {
        if !functions.contains_key(&datum.name.clone().to_string()) {
            if !files.contains_key(&datum.file.clone().to_string()) {
                let library =
                    unsafe { Library::new(datum.file.clone().to_string().as_str()).unwrap() };
                files.insert(datum.file.clone().to_string(), library);
            }
            let lib = { &files[&datum.file.clone().to_string()] };
            functions.insert(datum.name.clone().to_string(), unsafe {
                *lib.get::<unsafe extern "C" fn(data: repr_c::Vec<u8>) -> repr_c::Vec<u8>>(
                    &datum.name.to_string().as_bytes(),
                )
                .unwrap()
            });
        }
        acc.push(unsafe { functions[&datum.name.clone().to_string()](datum.data.clone()) });
    }
    for (_, lib) in files.drain() {
        lib.close().unwrap();
    }
    acc.into()
}

#[ffi_export]
pub fn compile(link: &repr_c::String) {
    Command::new(vec![link.to_string().as_str(), ".build"].concat())
        .output()
        .unwrap_or_else(|status| panic!("Failed to compile '{:#?}'", status));
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Data {
    pub parenthesized: bool,
    pub raw: repr_c::Vec<u8>,
}

#[ffi_export]
pub fn split_parenthesis(data: &repr_c::Vec<u8>) -> repr_c::Vec<Data> {
    let mut nest_counter = 0;
    let mut start = 0;
    let mut acc = vec![];
    for (n, c) in data.iter().enumerate() {
        if *c == b'(' {
            nest_counter += 1;
            if nest_counter == 1 {
                acc.push(Data {
                    parenthesized: false,
                    raw: data[start..n].to_vec().into(),
                });
                start = n + 1;
            }
        }
        if *c == b')' {
            if nest_counter == 1 {
                acc.push(Data {
                    parenthesized: true,
                    raw: data[start..n].to_vec().into(),
                });
                start = n + 1;
            }
            nest_counter -= 1;
        }
        nest_counter = nest_counter.clamp(0, i32::MAX);
    }
    acc.into()
}

#[ffi_export]
pub fn generate(data: &repr_c::Vec<repr_c::Vec<u8>>) -> repr_c::Vec<repr_c::Vec<u8>> {
    let split = data
        .iter()
        .map(|a| {
            if a.first().is_some_and(|b| *b == b'#') {
                vec![Data {
                    parenthesized: true,
                    raw: a[1..].to_vec().into(),
                }]
                .into()
            } else {
                split_parenthesis(a)
            }
        })
        .collect::<Vec<_>>();
    let macros = split
        .iter()
        .flat_map(|a| a.iter().filter(|b| b.parenthesized).map(|b| parse(&b.raw)))
        .collect::<Vec<_>>();
    macros.iter().for_each(|a| compile(&a.file));
    let results = expand(&macros.into());
    let mut i: usize = 0;
    let mut joined = vec![];
    for a in split {
        let mut acc = vec![];
        for b in a.to_vec() {
            if b.parenthesized {
                acc.append(&mut results[i].to_vec());
                i += 1;
            } else {
                acc.append(&mut b.raw.to_vec());
            }
        }
        joined.push(acc.into());
    }
    joined.into()
}

#[cfg(feature = "h")]
pub fn gen_headers() {
    safer_ffi::headers::builder()
        .to_file("lame.h")
        .unwrap()
        .generate()
        .unwrap();
}
