include!(concat!(env!("OUT_DIR"), "/table.rs"));

pub type PatternTable = phf::Map<u64, i32>;

pub fn get_pattern_table(c: char) -> (&'static  PatternTable, i32) {
    match c {
        'c'=> (&C, 2 * 2544), 
        'g'=> (&G, 2 *  752), 
        'i'=> (&I, 2 * 2953), 
        'o'=> (&O, 2 * 1621), 
        's'=> (&S, 2 * 3198), 
        'u'=> (&U, 2 * 2394), 
        _ => unreachable!(),
    }
}

pub fn is_index(c: char) -> bool {
    match c {
        'c'|'C'|'g'|'G'|'i'|'I'|'o'|'O'|'s'|'S'|'u'|'U' => true,
        _ => false,
    }
}
