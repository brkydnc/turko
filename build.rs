use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const INDICES: [char; 6] = ['c', 'g', 'i', 'o', 's', 'u'];
const MAX_EXTENT: usize = 4;
const SPACE: u8 = 0x20;

fn extent(string: &str) -> usize {
    let position = string.bytes().position(|x| x == b'X').unwrap();
    position.max(string.len() - position - 1)
}

fn pattern_to_integer(pattern: &str) -> u64 {
    assert!(pattern.len() <= MAX_EXTENT * 2 + 1);

    let bytes = pattern.as_bytes();
    let position = pattern.bytes().position(|x| x == b'X').unwrap();

    let offset = std::mem::size_of::<u64>() / 2;
    let mut number = [0u8; std::mem::size_of::<u64>()];

    (&mut number[offset - position..]).write(&bytes[..position]).unwrap();
    (&mut number[offset..]).write(&bytes[position + 1..]).unwrap();

    u64::from_be_bytes(number)
}

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    let codegen_path = Path::new(&outdir).join("table.rs");
    let mut codegen_file = BufWriter::new(File::create(&codegen_path).unwrap());

    for index in INDICES {
        let content = std::fs::read_to_string(format!("table/{}", index)).unwrap();
        let mut map = phf_codegen::Map::new();

        for line in content.lines() {
            let (pattern, rank) = line.split_once(',').unwrap();
            if extent(pattern) > MAX_EXTENT { continue; }
            map.entry(pattern_to_integer(pattern), rank);
        }

        write!(
            &mut codegen_file,
            "static {}: phf::Map<u64, i32> = {};",
            index.to_ascii_uppercase(),
            map.build(),
        ).unwrap();

        write!(&mut codegen_file, "\n").unwrap()
    }
}
