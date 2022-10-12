pub fn remap_linear(input: i16) -> u8 {
    if input.is_negative() {
        panic!("should never be negative")
    }

    if input == 0 {
        return 0;
    }

    let absolute = input.unsigned_abs();
    let mapped = absolute / 128;

    u8::try_from(mapped).expect("value should always fit")
}

pub fn remap_axis(input: i16) -> i8 {
    if input == 0 {
        return 0;
    }

    let mapped = input / 256;
    i8::try_from(mapped).expect("value should always fit")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remap_linear() {
        assert_eq!(remap_linear(0), 0);
        assert_eq!(remap_linear(16384), 128);
        assert_eq!(remap_linear(32767), 255);
    }

    #[test]
    fn test_remap_axis() {
        assert_eq!(remap_axis(0), 0);
        assert_eq!(remap_axis(32767), 127);
        assert_eq!(remap_axis(-32768), -128);
    }
}
