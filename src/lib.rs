mod pattern;
mod context;
mod kontext;

use pattern::{PatternTable, get_pattern_table, is_index};
pub use context::{Context, ContextBuffer, asciify, toggle_accent};

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
    let context_string = context.as_str();

    // for start in 0..=ContextBuffer::EXTENT {
    //     for stop in ContextBuffer::EXTENT + 1..=context_string.len() {
    //         let substring = &context_string[start..stop];

    //         if let Some(r) = table.get(substring) {
    //             if r.abs() < rank.abs() {
    //                 rank = *r;
    //             }
    //         }
    //     }
    // }

    rank > 0
}
