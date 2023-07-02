use std::iter::repeat;

const EXTENT: usize = std::mem::size_of::<u64>() / 2;
const SPACE: u64 = ' ' as u64;

#[derive(Debug)]
#[repr(transparent)]
struct Pattern(u64);

impl Pattern {
    const fn new() -> Self {
        Pattern(0x2020202020202020)
    }
    
    const fn from(n: u64) -> Self {
        Pattern(n)
    }
}

impl std::ops::Shl<u64> for Pattern {
    type Output = Self;
    fn shl(self, rhs: u64) -> Self {
        Pattern((self.0 << 8) | (0xFFFF & rhs))
    }
}

#[derive(Debug)]
struct Context {
    pattern: Pattern,
    // TODO: use u8 instead?
    current: u64,
}

impl Context {
    fn create(string: &str) -> Self {
        let current = string
            .chars()
            .map(forward)
            .next()
            .unwrap_or(SPACE);

        let pattern = string
            .chars()
            .skip(1)
            .map(forward)
            .chain(repeat(SPACE))
            .take(EXTENT)
            .fold(Pattern::new(), |pattern, byte| pattern << byte);

        Self {
            pattern,
            current,
        }
    }
    
    const fn space(&self) -> bool {
        self.current == SPACE
    }
    
    fn feed(&mut self) {
        if self.space() {
            
        } else {
            
        }
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

fn main() {
    let string = "üğ,i ç abc";
    let mut context = Context::create(string);
    println!("{:x?}", context);
    println!("{:x?}", string.chars().map(forward).collect::<Vec<_>>());
}
