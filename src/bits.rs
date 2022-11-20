use bit_vec::BitVec;
use std::str;
use std::collections::HashMap;

pub fn string_to_qr_bits(input: String) -> BitVec {
    // Probably a better way to do this, but whatever.
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

    // Calculate the size of the bit vector
    let d = input.chars().count() as u32;
    let size: u32 = 13 + (11 * (d / 2)) + (6 * (d % 2));

    if size > 154 {
        panic!("URL too long");
    }

    let mut bv = BitVec::from_elem(size as usize, false);

    // Set the first 4 to the QR code mode indicator 0010
    bv.set(2, true);

    // Set the next 9 bits to the size of the string
    let d_bytes: [u8; 4] = d.to_be_bytes();
    if d_bytes[2] == 1 {
        bv.set(4, true)
    }

    for i in 0..8 {
        if d_bytes[3] / 2u8.pow(i) % 2 == 1 {
            bv.set(12 - i as usize, true);
        }
    }
    
    // Loop through string and insert bytes
    let mut b_index: u32 = 24;
    let mut s_index: u32 = 0;
    while s_index < d - 1 {

        // Find the values of the next 2 characters (table 5 on page 34 in ISO standard)
        let mut c_1: u8 = input.as_bytes()[s_index as usize];

        if 48 <= c_1 && c_1 <= 57 {
            c_1 = c_1 - 48;
        }
        else {
            println!("decode: {}", str::from_utf8(&vec![c_1]).unwrap());
            match char_values.get(&str::from_utf8(&vec![c_1]).unwrap()) {
                Some(&val) => c_1 = val,
                _ => panic!("Not encodable as Alpha Numeric!"),
            };
        }

        let mut c_2: u8 = input.as_bytes()[s_index as usize + 1];

        if 48 <= c_2 && c_2 <= 57 {
            c_2 = c_2 - 48;
        }
        else {
            match char_values.get(&str::from_utf8(&vec![c_2]).unwrap()) {
                Some(&val) => c_2 = val,
                _ => panic!("Not encodable as Alpha Numeric!"),
            };
        }

        // Calculate the value we are adding to our bit vec.
        let insert_value: u32 = c_1 as u32 * 45 + c_2 as u32;


        for i in 0..11 {
            println!("{} / {} % {}", insert_value, (2u16.pow(i)), 2);
            if insert_value / 2u32.pow(i) % 2 == 1 {
                bv.set((b_index - i) as usize, true);
            }
        }

        b_index += 11;
        s_index += 2;
    }

    // Deal with the last case of when the input is odd.
    if d % 2 == 1 {
        let mut c: u8 = input.as_bytes()[d as usize - 1];
        if 48 <= c && c <= 57 {
            c = c - 48;
        }
        else {
            match char_values.get(&c.to_string().to_uppercase().as_str()) {
                Some(&val) => c = val,
                _ => panic!("Not encodable as Alpha Numeric!"),
            };
        }

        for i in 0..6 {
            if c / 2u8.pow(i) % 2 == 1 {
                bv.set((d - 1 - i) as usize, true);
            }
        }
    }

    bv
}

