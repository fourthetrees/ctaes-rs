// Some generally useful utilities.

// safely parse a 128 bit hex string into a [u8;16]
// without heap allocation.  returns 16 most significant
// bytes if hex is greater than 128, and appends 0 bits if
// hex is less than 128.
pub fn parse_hex_128(hex: &str) -> [u8;16] {
    let mut collector: [u8;16] = [0;16];
    let mut chars = hex.chars();
    let mut next_val = move || { chars.next().unwrap_or('0').to_digit(16).unwrap_or(0) as u8 };
    for i in 0..16 {
        collector[i] = (next_val() << 4) | next_val();
    }
    collector
}

#[test]
#[cfg(test)]
fn test() {
    let hex = "00ff00ff00ff00ff00ff00ff00ff00ff";
    let values: [u8;16] = [0,255,0,255,0,255,0,255,0,255,0,255,0,255,0,255];
    let converted = parse_hex_128(hex);
    assert_eq!(values,converted);
}



