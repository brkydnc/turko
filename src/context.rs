
pub const EXTENT: usize = 4;
// TODO: use 0 as empty (remove spaces=0x20)?
const EMPTY_PATTERN: u64 = 0x2020202020202020;

#[derive(Debug)]
pub struct Context {
    pub pattern: u64,
    pub current: char,
}

impl Context {
    pub fn of(chars: &[char], at: usize) -> Self {
        let current = chars[at];

        let mut pattern = chars[at + 1..]
            .iter()
            .map(forward)
            .take(EXTENT)
            .fold(EMPTY_PATTERN, |p, n| (p << 8) | n);

        let mut i = 4;
        let mut invalid = false;

        for c in chars[..at].iter().rev() {
            let b = backward(c);

            if b == 0 {
                if !invalid {
                    invalid = true;
                    pattern &= !(0xFF << (i * 8));
                    pattern |= 0x20 << (i * 8);
                    i += 1;
                }
            } else {
                invalid = false;
                pattern &= !(0xFF << (i * 8));
                pattern |= b << (i * 8);
                i += 1;
            }

            if i == 8 { break; }
        }

        Self { pattern, current }
    }
}

pub fn forward(c: &char) -> u64 {
    match c {
        'a'..='z'|'A'..='Z' => c.to_ascii_lowercase(),
        'ç'|'Ç' => 'c',
        'ğ'|'Ğ' => 'g',
        'ö'|'Ö' => 'o',
        'ı'|'İ' => 'i',
        'ş'|'Ş' => 's',
        'ü'|'Ü' => 'u',
      _ => '\0',
    }.into()
}

pub fn backward(c: &char) -> u64 {
    match c {
        'a'..='z'|'A'..='Z' => c.to_ascii_lowercase(),
        'ç'|'Ç' => 'C',
        'ğ'|'Ğ' => 'G',
        'ö'|'Ö' => 'O',
        'ş'|'Ş' => 'S',
        'ü'|'Ü' => 'U',
        'ı' => 'I',
        'İ' => 'i',
      _ => '\0',
    }.into()
}

pub fn asciify(c: char) -> char {
    match c {
        'ç' => 'c',
        'Ç' => 'C',
        'ğ' => 'g',
        'Ğ' => 'G',
        'ö' => 'o',
        'Ö' => 'O',
        'ü' => 'u',
        'Ü' => 'U',
        'ı' => 'i',
        'İ' => 'I',
        'ş' => 's',
        'Ş' => 'S',
        _ => c 
    }
}

pub fn toggle_accent(c: char) -> char {
    match c {
        'c' => 'ç',
        'C' => 'Ç',
        'g' => 'ğ',
        'G' => 'Ğ',
        'o' => 'ö',
        'O' => 'Ö',
        'u' => 'ü',
        'U' => 'Ü',
        'i' => 'ı',
        'I' => 'İ',
        's' => 'ş',
        'S' => 'Ş',
        'ç' => 'c',
        'Ç' => 'C',
        'ğ' => 'g',
        'Ğ' => 'G',
        'ö' => 'o',
        'Ö' => 'O',
        'ü' => 'u',
        'Ü' => 'U',
        'ı' => 'i',
        'İ' => 'I',
        'ş' => 's',
        'Ş' => 'S',
        _ => c,
    }
}
