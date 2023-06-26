use std::str::{ Utf8Error, from_utf8 };

// TODO: Maybe the performance of `char` is better than `u8`?
#[derive(Debug)]
struct ContextBuffer {
    inner: [u8; Self::SIZE]
}

impl ContextBuffer {
    const EXTENT: usize = 10;
    const SIZE: usize = 1 + 2 * Self::EXTENT;

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
            left: ContextBuffer::EXTENT - 1,
            right: ContextBuffer::EXTENT + 1
        }
    }

    fn append(&mut self, byte: u8) {
        self.buf.inner[self.right] = byte;
        self.right += 1;
    }

    fn prepend(&mut self, byte: u8) {
        self.buf.inner[self.left] = byte;
        self.left -= 1;
    }

    fn build(self) -> Result<Context, Utf8Error> {
        match from_utf8(&self.buf.inner) {
            Ok(_) => Ok(Context { buf: self.buf, end: self.right }),
            Err(e) => Err(e)
        }
    }
}

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

    pub fn of(string: &str, at: usize) -> Self {
        let mut context = ContextBuilder::new();
        let preceding = &string[..at];
        let following = &string[at + 1..];

        let mut previous_invalid = false;
        for c in preceding.chars().rev() {
            if let Some(ascii) = asciify(c) {
                context.prepend(ascii);
                previous_invalid = false;
            } else if previous_invalid {
                previous_invalid = false;
            } else {
                context.prepend(b' ');
                previous_invalid = true;
            }
        }

        for c in following.chars().take(ContextBuffer::EXTENT) {
            if let Some(ascii) = asciify(c) {
                context.append(ascii);
            } else {
                context.append(b' ');
                break;
            }
        }

        context.build().unwrap()
    }
}

fn asciify(c: char) -> Option<u8> {
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

fn upcase_accent(c: char) -> (bool, u8) {
    match c {
      'ç' => (true, b'C'),
      'Ç' => (true, b'C'),
      'ğ' => (true, b'G'),
      'Ğ' => (true, b'G'),
      'ö' => (true, b'O'),
      'Ö' => (true, b'O'),
      'ı' => (true, b'I'),
      'İ' => (true, b'i'),
      'ş' => (true, b'S'),
      'Ş' => (true, b'S'),
      'ü' => (true, b'U'),
      'Ü' => (true, b'U'),
      _ => (c.is_ascii_alphabetic(), c.to_ascii_lowercase() as u8),
    }
}

fn toggle_accent(c: char) -> char {
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
