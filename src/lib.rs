mod pattern;
mod context;

use pattern::{PatternTable, get_pattern_table, is_index};
pub use context::{EXTENT, Context, asciify, toggle_accent};

const MASKS: [u64; 25] = [
    0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFF00, 0xFFFFFFFFFFFF0000, 0xFFFFFFFFFF000000, 0xFFFFFFFF00000000,
    0x00FFFFFFFFFFFFFF, 0x00FFFFFFFFFFFF00, 0x00FFFFFFFFFF0000, 0x00FFFFFFFF000000, 0x00FFFFFF00000000,
    0x0000FFFFFFFFFFFF, 0x0000FFFFFFFFFF00, 0x0000FFFFFFFF0000, 0x0000FFFFFF000000, 0x0000FFFF00000000,
    0x000000FFFFFFFFFF, 0x000000FFFFFFFF00, 0x000000FFFFFF0000, 0x000000FFFF000000, 0x000000FF00000000,
    0x00000000FFFFFFFF, 0x00000000FFFFFF00, 0x00000000FFFF0000, 0x00000000FF000000, 0x0000000000000000,
];

pub fn correct(string: &str) -> String {
    let mut turkish: Vec<char> = string.chars().collect();

    for i in 0..turkish.len() {
        let c = turkish[i];

        if !is_index(c) { continue; }

        let context = Context::of(&turkish, i);

        if need_correction(&context, c) {
            turkish[i] = toggle_accent(c)
        }
    }

    turkish.into_iter().collect()
}

fn need_correction(context: &Context, character: char) -> bool {
    let (table, rank) = get_pattern_table(character);
    match_pattern(context, table, rank)
}

fn match_pattern(context: &Context, table: &PatternTable, mut rank: i32) -> bool {
    for mask in MASKS {
        if let Some(r) = table.get(&(context.pattern & mask)) {
            if r.abs() < rank.abs() {
                rank = *r;
            }
        }
    }

    rank > 0
}
