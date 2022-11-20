mod bits;

fn main() {
    let b = bits::string_to_qr_bits("AC-42".to_string());

    println!("Bits: {:?}", b);
}
