// Some generally useful utilities.

// parse a 128 bit hex string into a [u8;16]
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

// parse a 192 bit hex string into a [u8;24]
// without heap allocation.  returns 24 most significant
// bytes if hex is greater than 192, and appends 0 bits if
// hex is less than 192.
pub fn parse_hex_192(hex: &str) -> [u8;24] {
    let mut collector: [u8;24] = [0;24];
    let mut chars = hex.chars();
    let mut next_val = move || { chars.next().unwrap_or('0').to_digit(16).unwrap_or(0) as u8 };
    for i in 0..24 {
        collector[i] = (next_val() << 4) | next_val();
    }
    collector
}

// parse a 256 bit hex string into a [u8;32]
// without heap allocation.  returns 32 most significant
// bytes if hex is greater than 256, and appends 0 bits if
// hex is less than 256.
pub fn parse_hex_256(hex: &str) -> [u8;32] {
    let mut collector: [u8;32] = [0;32];
    let mut chars = hex.chars();
    let mut next_val = move || { chars.next().unwrap_or('0').to_digit(16).unwrap_or(0) as u8 };
    for i in 0..32 {
        collector[i] = (next_val() << 4) | next_val();
    }
    collector
}


#[test]
#[cfg(test)]
fn test_128() {
    let hex = "00ff00ff00ff00ff00ff00ff00ff00ff";
    let val: [u8;16] = [0,255,0,255,0,255,0,255,0,255,0,255,0,255,0,255];
    let rslt = parse_hex_128(hex);
    assert_eq!(val,rslt);
}

#[test]
#[cfg(test)]
fn test_192() {
    let hex = "00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff";
    let val: [u8;24] = [
        0,255,0,255,0,255,0,255,0,255,0,255,
        0,255,0,255,0,255,0,255,0,255,0,255
    ];
    let rslt = parse_hex_192(hex);
    assert_eq!(val,rslt);
}

#[test]
#[cfg(test)]
fn test_256() {
    let hex = "00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff";
    let val: [u8;32] = [
        0,255,0,255,0,255,0,255,0,255,0,255,0,255,0,255,
        0,255,0,255,0,255,0,255,0,255,0,255,0,255,0,255
    ];
    let rslt = parse_hex_256(hex);
    assert_eq!(val,rslt);
}
