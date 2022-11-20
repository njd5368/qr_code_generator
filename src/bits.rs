use bit_vec::BitVec;
use std::collections::HashMap;

pub fn string_to_qr_bits(input: String) -> BitVec {
    /// Probably a better way to do this, but whatever.
    let char_values = HashMap::from([
        ("A", 10),
        ("B", 11),
        ("C", 12),
        ("D", 13),
        ("E", 14),
        ("F", 15),
        ("G", 16),
        ("H", 17),
        ("I", 18),
        ("J", 19),
        ("K", 20),
        ("L", 21),
        ("M", 22),
        ("N", 23),
        ("O", 24),
        ("P", 25),
        ("Q", 26),
        ("R", 27),
        ("S", 28),
        ("T", 29),
        ("U", 30),
        ("V", 31),
        ("W", 32),
        ("X", 33),
        ("Y", 34),
        ("Z", 35),
        (" ", 36),
        ("$", 37),
        ("%", 38),
        ("*", 39),
        ("+", 40),
        ("-", 41),
        (".", 42),
        ("/", 43),
        (":", 44),
    ]);

    /// Calculate the size of the bit vector
    let d: u32 = input.chars().count();
    let size: u32 = 13 + (11 * (d / 2)) + (6 * (d % 2));

    if (size > 154) {
        panic!("URL too long");
    }

    let mut bv = BitVec::from_elem(size, false);

    /// Set the first 4 to the QR code mode indicator 0010
    bv.set(2, true);

    /// Set the next 9 bits to the size of the string
    let size_bytes: [u8; 4] = size.to_be_bytes();
    if (size_bytes[2] == 1) {
        bv.set(4, true)
    }

    for i in 0..8 {
        if (size_bytes[3] / pow(2u8, i) % 2) {
            bv.set(13 - i, true);
        }
    }
    
    /// Loop through string and insert bytes
    let mut b_index: u32 = 25;
    let mut s_index: u32 = 0;
    while s_index < size - 1 {
        let mut c_1: u8 = input.as_bytes()[s_index];

        if (c_1 > 9) {
            
            c_1 = char_values.get()
        }

        let mut c_2: u8 = input.as_bytes()[s_index + 1];

        for i in 0..11 {
            if ()
        }
        s_index += 2;
    }
    
}

