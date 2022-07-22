pub fn get_nth_octal(num: u32, n: usize) -> u8 {
    ((num >> (32 - (8 * n))) & 0xFF) as u8
}

// least-significant byte
pub fn get_lsb(val: u16) -> u8 {
    (val & 0xFF) as u8
}

// most-significant byte
pub fn get_msb(val: u16) -> u8 {
    (val >> 8) as u8
}

// low/least-significant nibble (still u8 because Rust has no inbuilt u4)
pub fn get_lsn(val: u8) -> u8 {
    val & 0x0F
}

// is nth bit set?
pub fn get_flag(flags: u8, pos: u8) -> bool {
    (flags & (1 << pos)) == 1
}
