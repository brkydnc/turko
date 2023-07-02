use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const INDICES: [char; 6] = ['c', 'g', 'i', 'o', 's', 'u'];

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    let codegen_path = Path::new(&outdir).join("table.rs");
    let mut codegen_file = BufWriter::new(File::create(&codegen_path).unwrap());

    for index in INDICES {
        let content = std::fs::read_to_string(format!("table/{}", index)).unwrap();
        let mut map = phf_codegen::Map::new();

        for line in content.lines() {
            let (key, value) = line.split_once(',').unwrap();
            map.entry(key, value);
        }

        write!(
            &mut codegen_file,
            "static {}: phf::Map<&'static str, i32> = {};",
            index.to_ascii_uppercase(),
            map.build(),
        ).unwrap();

        write!(&mut codegen_file, "\n").unwrap()
    }
}
