use std::str::{ Utf8Error, from_utf8 };

#[derive(Debug)]
pub struct ContextBuffer {
    inner: [u8; Self::SIZE]
}

impl ContextBuffer {
    pub const EXTENT: usize = 4;
    pub const SIZE: usize = 1 + 2 * Self::EXTENT;

    const fn empty() -> Self {
        let mut inner = [b' '; ContextBuffer::SIZE];
        inner[ContextBuffer::EXTENT] = b'X';
        Self { inner }
    }
}

struct ContextBuilder {
    buf: ContextBuffer,
    left: usize,
    right: usize,
}

impl ContextBuilder {
    const fn new() -> Self {
        Self {
            buf: ContextBuffer::empty(),
            left: ContextBuffer::EXTENT,
            right: ContextBuffer::EXTENT,
        }
    }

    fn append(&mut self, c: u8) {
        self.right += 1;
        self.buf.inner[self.right] = c;
    }

    fn prepend(&mut self, byte: u8) {
        self.left -= 1;
        self.buf.inner[self.left] = byte;
    }

    fn build(self) -> Result<Context, Utf8Error> {
        match from_utf8(&self.buf.inner) {
            Ok(_) => Ok(Context { buf: self.buf, end: self.right + 1 }),
            Err(e) => Err(e)
        }
    }
}

// TODO: add current char
#[derive(Debug)]
pub struct Context {
    buf: ContextBuffer,
    end: usize,
}

impl Context {
    pub fn as_str(&self) -> &str {
        // SAFETY: ContextBuilder guarantees that the bytes are
        // valid ASCII.
        let slice = &self.buf.inner[..self.end];
        unsafe { std::str::from_utf8_unchecked(slice) }
    }

    pub fn of(chars: &[char], at: usize) -> Self {
        let mut context = ContextBuilder::new();
        let preceding = chars[..at].iter().rev();
        let following = chars[at + 1..].iter().take(ContextBuffer::EXTENT);

        let mut previous_invalid = false;
        for c in preceding {
            if context.left == 0 { break; }
            if let Some(upcase) = upcase_accent(*c) {
                context.prepend(upcase);
                previous_invalid = false;
            } else if previous_invalid {
                previous_invalid = false;
            } else {
                context.prepend(b' ');
                previous_invalid = true;
            }
        }

        for c in following {
            if let Some(ascii) = downcase_asciify(*c) {
                context.append(ascii);
            } else {
                context.append(b' ');
                break;
            }
        }

        context.build().unwrap()
    }
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

pub fn downcase_asciify(c: char) -> Option<u8> {
    match c {
      'ç' => Some(b'c'),
      'Ç' => Some(b'c'),
      'ğ' => Some(b'g'),
      'Ğ' => Some(b'g'),
      'ö' => Some(b'o'),
      'Ö' => Some(b'o'),
      'ı' => Some(b'i'),
      'İ' => Some(b'i'),
      'ş' => Some(b's'),
      'Ş' => Some(b's'),
      'ü' => Some(b'u'),
      'Ü' => Some(b'u'),
        _ => c.is_ascii_alphabetic().then_some(c.to_ascii_lowercase() as u8),
    }
}

pub fn upcase_accent(c: char) -> Option<u8> {
    match c {
      'ç' => Some(b'C'),
      'Ç' => Some(b'C'),
      'ğ' => Some(b'G'),
      'Ğ' => Some(b'G'),
      'ö' => Some(b'O'),
      'Ö' => Some(b'O'),
      'ı' => Some(b'I'),
      'İ' => Some(b'i'),
      'ş' => Some(b'S'),
      'Ş' => Some(b'S'),
      'ü' => Some(b'U'),
      'Ü' => Some(b'U'),
      _ => c.is_ascii_alphabetic().then_some(c.to_ascii_lowercase() as u8),
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
