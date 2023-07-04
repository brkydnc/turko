use std::iter::repeat;
use crate::pattern::is_index;

const EXTENT: usize = std::mem::size_of::<u64>() / 2;
const SPACE: u64 = ' ' as u64;
const EMPTY_PATTERN: u64 = 0x2020202020202020;

#[derive(Debug)]
struct Context {
    pattern: u64,
    current: u64,
}

impl Context {
    fn new(string: &str) -> (Self, &str) {
        let ref mut chars = string.chars();
        let mut mapped = chars.map(forward);

        let current = mapped.next().unwrap_or(SPACE);

        let pattern = mapped
            .chain(repeat(SPACE))
            .take(EXTENT)
            .fold(EMPTY_PATTERN, |pattern, n| (pattern << 8) & n);

        let context = Context { pattern, current };
        let rest = chars.as_str();

        (context, rest)
    }

    fn feed(&mut self, n: u64) -> u64 {
        let old = self.current;

        self.current = (self.current >> 24) & 0xFF;
        self.pattern = (self.pattern & 0xFF000000) | (old << 24);
        self.pattern = (self.pattern << 8) | (0xFF & n);

        old
    }

    fn exhausted(&self) -> bool {
        self.pattern & 0xFFFFFFFF == 0x20202020
    }

    fn should_toggle() -> bool {
        true
    }

    fn toggle(&mut self) {

    }
}

#[derive(Debug)]
struct Deasciifier<'a> {
    rest: &'a str,
    context: Context,
}

impl<'a> Deasciifier<'a> {
    fn new(string: &'a str) -> Deasciifier<'a> {
        let (context, rest) = Context::new(string);
        Self { rest, context }
    }
}

impl Iterator for Deasciifier<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let ref mut chars = self.rest.chars();

        if let Some(next) = chars.next() {
            let fwd = forward(next);
        } else {
            if self.context.exhausted() 
        }

        None
    }
}

pub fn forward(c: char) -> u64 {
    match c {
        'a'..='z'|'A'..='Z' => c.to_ascii_lowercase(),
        'ç'|'Ç' => 'c',
        'ğ'|'Ğ' => 'g',
        'ö'|'Ö' => 'o',
        'ı'|'İ' => 'i',
        'ş'|'Ş' => 's',
        'ü'|'Ü' => 'u',
      _ => ' '
    }.into()
}
