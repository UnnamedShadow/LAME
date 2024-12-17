use lame::*;
use std::env::args;
use std::fs::read_to_string;
use std::fs::write;

fn main() {
    let macros = args()
        .skip(1)
        .map(|path| read_to_string(path + ".lame").unwrap().as_bytes().to_vec())
        .map(|rawf| {
            if rawf.first().is_some_and(|c| *c == b'#') {
                vec![parse(rawf.into())]
            } else {
                split_parenthesis(rawf.into())
                    .iter()
                    .map(|rawm| parse(rawm.clone()))
                    .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();
    let flat = macros.concat();
    let carr = flat.iter().map(|m| m.file.clone()).collect::<Vec<_>>();
    compile(carr.into());
    let expanded = expand(flat.into());
    let points = macros.iter().map(|ms| ms.len()).fold(vec![], |acc, l| {
        let start = acc.last().map_or(0, |(_, e)| *e);
        let end = start + l;
        vec![acc, vec![(start, end)]].concat()
    });
    let unflat = points
        .iter()
        .map(|(s, e)| expanded[*s..*e].to_vec())
        .map(|v| v.iter().map(|a| a.to_vec()).collect::<Vec<_>>().concat())
        .collect::<Vec<_>>();
    args()
        .skip(1)
        .enumerate()
        .map(|(i, path)| (path, unflat[i].clone()))
        .for_each(|(path, code)| write(path, code).unwrap());
}
