use turko::{correct, asciify};
use std::time::Instant;

fn is_turkish_char(c: char) -> bool {
    match c {
        'ç' | 'Ç' | 'ğ' | 'Ğ' | 'ö' | 'Ö' | 'ü' | 'Ü' | 'ı' | 'İ' | 'ş' | 'Ş' => true,
        _ => false,
    }
} 

fn main() {
    let text = include_str!("../../text/large.txt");
    let ascii = text.chars().map(asciify).collect::<String>();

    let now = Instant::now();
    let output = correct(&ascii);
    let elapsed = now.elapsed();

    let mut num_original_turkish_chars: usize = 0;
    let mut num_true_corrected: usize = 0;
    let mut num_false_corrected: usize = 0;

    for (original, corrected) in text.chars().zip(output.chars()) {
        if is_turkish_char(original) {
            num_original_turkish_chars += 1;

            if original == corrected {
                num_true_corrected += 1;
            }
        } else {
            if original != corrected {
                num_false_corrected += 1;
            }
        }
    }

    println!("elapsed: {:?}", elapsed);
    println!("original: {}, true: {}, false: {}", num_original_turkish_chars, num_true_corrected, num_false_corrected);
    println!("accuracy: {}, correcting error: {}", 100f64 * num_true_corrected as f64 / num_original_turkish_chars as f64, 100f64 * num_false_corrected as f64 / (num_true_corrected + num_false_corrected) as f64);
}
