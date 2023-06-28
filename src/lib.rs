// IDEAS
//
// Contexes are spatially cacehable

mod pattern;
mod context;

pub use context::{Context, ContextBuffer, asciify, toggle_accent};

use pattern::{ PatternTable, PATTERN_TABLE_INDEX };

pub fn correct(string: &str) -> String {
    let mut turkish: Vec<char> = string.chars().collect();

    for i in 0..turkish.len() {
        let c = turkish[i];
        let context = Context::of(&turkish, i);

        if need_correction(&context, c) {
            turkish[i] = toggle_accent(c)
        }
    }

    turkish.into_iter().collect()
}

fn need_correction(context: &Context, character: char) -> bool {
    let maybe_ascii = asciify(character);

    let matches = if let Some(table) = PATTERN_TABLE_INDEX.get(&maybe_ascii.to_ascii_lowercase()) {
        match_pattern(context, table)
    } else {
        false
    };

    if maybe_ascii == 'I' {
        if character == maybe_ascii { !matches } else { matches }
    } else {
        if character == maybe_ascii { matches } else { !matches }
    }
}

fn match_pattern(context: &Context, table: &PatternTable) -> bool {
    let context_string = context.as_str();
    let mut rank = table.rank;

    for start in 0..=ContextBuffer::EXTENT {
        for stop in ContextBuffer::EXTENT + 1..=context_string.len() {
            let substring = &context_string[start..stop];

            if let Some(r) = table.inner.get(substring) {
                if r.abs() < rank.abs() {
                    rank = *r;
                }
            }
        }
    }

    rank > 0
}
