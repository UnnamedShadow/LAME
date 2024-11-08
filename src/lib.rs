use core::slice;
use libc::{dlclose, dlopen, dlsym};
use std::{collections::HashMap, ffi, process::Command};

#[repr(C)]
#[derive(Clone)]
pub struct Arr<T> {
    pub data: *const T,
    pub len: usize,
}

impl<T> Arr<T> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data, self.len) }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct Macro {
    pub name: *const ffi::c_char,
    pub file: *const ffi::c_char,
    pub data: Arr<u8>,
}

#[no_mangle]
pub extern "C" fn parse(raw: Arr<u8>) -> Macro {
    // a simple parser that splits the file into the following pattern:
    // symbol@path/to/file!input some code or something
    let data = raw.as_slice();
    let (name, data) = data.split_at(data.iter().position(|c| *c == b'@').unwrap());
    let (file, data) = data.split_at(data.iter().position(|c| *c == b'!').unwrap());
    // adding the null byte at the end
    let mut name = name.to_vec();
    name.push(0);
    let mut file = file.to_vec();
    file.push(0);
    Macro {
        name: name.as_slice().as_ptr() as *const _,
        file: file.as_slice().as_ptr() as *const _,
        data: Arr {
            data: data.as_ptr(),
            len: data.len(),
        },
    }
}

#[no_mangle]
pub extern "C" fn expand(raw: Arr<Macro>) -> Arr<Arr<u8>> {
    let data = raw.as_slice();
    let mut files = HashMap::new();
    let mut functions = HashMap::new();
    let mut acc = vec![];
    for datum in data {
        if !functions.contains_key(&datum.name) {
            if !files.contains_key(&datum.file) {
                files.insert(datum.file, unsafe { dlopen(datum.file as *const i8, 0) });
            }
            functions.insert(datum.name, unsafe {
                dlsym(files[&datum.file], datum.name as *const i8)
            });
        }
        let func: *const fn(data: Arr<u8>) -> Arr<u8> =
            unsafe { *functions.get(&datum.name).unwrap().cast() };
        acc.push(unsafe { (*func)(datum.data.clone()) });
    }
    for (name, handle) in files {
        let err = unsafe { dlclose(handle) };
        if err != 0 {
            println!("failed to close '{:#?}' error code: {}", name, err);
        }
    }
    let acc = acc.as_slice();
    Arr {
        data: acc.as_ptr(),
        len: acc.len(),
    }
}

#[no_mangle]
pub extern "C" fn compile(raw: Arr<*const ffi::c_char>) {
    raw.as_slice()
        .iter()
        .map(|ptr| unsafe { std::ffi::CStr::from_ptr(*ptr) })
        .map(|origin_link| vec![origin_link.to_str().unwrap(), ".lame"].concat())
        .for_each(|link| {
            Command::new(link).output().unwrap();
        });
}

#[no_mangle]
pub extern "C" fn split_parenthesis(raw: Arr<u8>) -> Arr<Arr<u8>> {
    let mut nest_counter = 0;
    let mut start = 0;
    let mut acc = vec![];
    for (n, c) in raw.as_slice().iter().enumerate() {
        if *c == b'(' {
            nest_counter += 1;
            if nest_counter == 1 {
                start = n
            }
        }
        if *c == b')' {
            if nest_counter == 1 {
                let s = &raw.as_slice()[start..n];
                acc.push(Arr {
                    data: s.as_ptr(),
                    len: s.len(),
                });
            }
            nest_counter -= 1;
        }
    }
    Arr {
        data: acc.as_ptr(),
        len: acc.len(),
    }
}
