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
    ((flags >> pos) & 1) == 1
}

#[cfg(test)]
mod tests {
    use super::{get_flag, get_lsb, get_lsn, get_msb, get_nth_octal};

    #[test]
    fn can_get_nth_octal() {
        for i in 1..=4 {
            assert_eq!(get_nth_octal(u32::MAX, i), 255);
            assert_eq!(get_nth_octal(u32::MIN, i), 0);
        }
    }

    #[test]
    fn can_get_least_significant_byte() {
        assert_eq!(get_lsb(u16::MAX), 255);
        assert_eq!(get_lsb(u16::MIN), 0);
    }

    #[test]
    fn can_get_most_significant_byte() {
        assert_eq!(get_msb(u16::MAX), 255);
        assert_eq!(get_msb(u16::MIN), 0);
    }

    #[test]
    fn can_get_low_nibble() {
        assert_eq!(get_lsn(u8::MAX), 15);
        assert_eq!(get_lsn(u8::MIN), 0);
    }

    #[test]
    fn can_get_flag_values_all_set() {
        for i in 0..8 {
            assert_eq!(get_flag(u8::MAX, i), true);
        }
    }

    #[test]
    fn can_get_flag_values_none_set() {
        for i in 0..8 {
            assert_eq!(get_flag(u8::MIN, i), false);
        }
    }
}
