// Some generally useful utilities.

// safely parse a 128 bit hex string into a [u8;16]
// without heap allocation.  returns 16 most significant
// bytes if hex is greater than 128, and appends 0 bits if
// hex is less than 128.
pub fn parse_hex_128(hex: &str) -> [u8;16] {
    let mut collector: [u8;16] = [0;16];
    let mut chars = hex.chars();
    for i in 0..16 {
        let mut a = '0'; let mut b = '0';
        let mut val: [u8;2] = [0,0];
        if let Some(c) = chars.next() { a = c; }
        if let Some(c) = chars.next() { b = c; }
        if let Some(n) = a.to_digit(16) {val[0] = n as u8;}
        if let Some(n) = b.to_digit(16) {val[1] = n as u8;}
        collector[i] = (val[0] << 4) | val[1];
    }
    collector
}
