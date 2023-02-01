use std::cmp::min;

pub fn parse_input<T: std::str::FromStr>(path: &str) -> Result<Vec<T>, std::io::Error> {
    Ok(std::fs::read_to_string(path)?
        .split("\n")
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            return line.parse::<T>().ok();
        })
        .collect::<Vec<T>>())
}

pub fn bit_str_to_u64(bit_str: &str) -> u64 {
    let bit_str = bit_str.trim();
    if bit_str.len() == 0 {
        return 0;
    }
    let mut value: u64 = 0;
    let mut addition_term: u64 = 1;
    for b in bit_str.chars().rev() {
        if b == '1' {
            value += addition_term;
        }
        addition_term *= 2;
    }
    value
}

pub fn hex_str_to_u8_vec(input: &str) -> Vec<u8> {
    (0..input.len())
        .step_by(2)
        .map(|i| {
            input
                .get(i..i + 2)
                .and_then(|hex_chars| u8::from_str_radix(hex_chars, 16).ok())
                .unwrap()
        })
        .collect()
}

pub fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn extract_bits_from_byte(byte_value: u8, start_bit: u8, bit_count: u8) -> u8 {
    let bit_offset = start_bit % 8;
    let mut extracted = byte_value;

    if bit_offset > 0 {
        let left_trim = (1 << 8 - bit_offset) - 1;
        extracted &= left_trim;
    }

    extracted >> 8 - bit_count - bit_offset
}

pub fn extract_bits(bytes: &[u8], start_bit: usize, bit_count: u8) -> Result<u64, &str> {
    let total_bits = bytes.len() * 8;

    if start_bit >= total_bits {
        return Err("invalid start_bit");
    }
    if bit_count > 64 {
        return Err("max value for bit_count is 64");
    }
    if start_bit + bit_count as usize > total_bits {
        return Err("not enough bits to extract");
    }

    let mut byte_index = start_bit / 8;
    let mut remaining_bits = bit_count;
    let mut bit_offset = (start_bit % 8) as u8;
    let mut extracted = 0u64;

    while remaining_bits > 0 {
        let bit_count_in_byte = min(remaining_bits, 8 - bit_offset);
        let bits = extract_bits_from_byte(bytes[byte_index], bit_offset, bit_count_in_byte);
        remaining_bits -= bit_count_in_byte as u8;

        extracted |= (bits as u64) << remaining_bits;

        byte_index += 1;
        bit_offset = 0;
    }

    Ok(extracted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_str_to_u64_should_return_0_for_empty_bit_str() {
        let s = "";
        assert_eq!(bit_str_to_u64(s), 0);
    }

    #[test]
    fn bit_str_to_u64_should_return_1_for_bit_str_1() {
        let s = "1";
        assert_eq!(bit_str_to_u64(s), 1);
    }

    #[test]
    fn bit_str_to_u64_should_return_2_for_bit_str_10() {
        let s = "10";
        assert_eq!(bit_str_to_u64(s), 2);
    }

    #[test]
    fn bit_str_to_u64_should_return_7_for_bit_str_111() {
        let s = "111";
        assert_eq!(bit_str_to_u64(s), 7);
    }

    #[test]
    fn bit_str_to_u64_should_return_25_for_bit_str_11001() {
        let s = "11001";
        assert_eq!(bit_str_to_u64(s), 25);
    }

    #[test]
    fn test_hex_str_to_u8_vec() {
        assert_eq!(vec![0xd2, 0xfe, 0x2b], hex_str_to_u8_vec("D2FE2B"));
    }

    #[test]
    fn test_bit_extraction_from_single_byte() {
        let data = 0xD2;
        // 1101 0010
        assert_eq!(0b11, extract_bits_from_byte(data, 0, 2));
        assert_eq!(0b1001, extract_bits_from_byte(data, 3, 4));
        assert_eq!(0b10010, extract_bits_from_byte(data, 3, 5));
        assert_eq!(0b11010010, extract_bits_from_byte(data, 0, 8));
    }

    #[test]
    fn test_bit_extraction_from_multiple_bytes() {
        let data = vec![0xD2, 0xFE, 0x28];
        // 1101 0010  1111 1110  0010 1000
        assert_eq!(Ok(0b11), extract_bits(&data, 0, 2));
        assert_eq!(Ok(0b1001), extract_bits(&data, 3, 4));
        assert_eq!(Ok(0b10010), extract_bits(&data, 3, 5));
        assert_eq!(Ok(0b1001011), extract_bits(&data, 3, 7));
        assert_eq!(Ok(0b100101111111000101), extract_bits(&data, 3, 18));
    }

    #[test]
    fn test_bit_extraction_from_multiple_bytes_invalid_start_bit() {
        let data = vec![0xD2, 0xFE, 0x28];
        // 1101 0010  1111 1110  0010 1000
        assert_eq!(Err("invalid start_bit"), extract_bits(&data, 24, 1));
    }

    #[test]
    fn test_bit_extraction_from_multiple_bytes_invalid_bit_count() {
        let data = vec![0xD2, 0xFE, 0x28];
        // 1101 0010  1111 1110  0010 1000
        assert_eq!(
            Err("max value for bit_count is 64"),
            extract_bits(&data, 0, 65)
        );
    }

    #[test]
    fn test_bit_extraction_from_multiple_bytes_not_enough_bits() {
        let data = vec![0xD2, 0xFE, 0x28];
        // 1101 0010  1111 1110  0010 1000
        assert_eq!(
            Err("not enough bits to extract"),
            extract_bits(&data, 0, 25)
        );
    }

}

