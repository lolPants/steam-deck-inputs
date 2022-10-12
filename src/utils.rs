pub fn convert_range(input: i16) -> u8 {
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
