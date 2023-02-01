use std::fs::read_to_string;
use aoc_2021::{hex_str_to_u8_vec, extract_bits};

const LITERAL_VALUE_TYPE_ID: u8 = 4;

#[derive(Debug, PartialEq)]
pub enum PacketPayload {
    LiteralValue(u64),
    Operator(Vec<TransmissionPacket>),
}

#[derive(Debug)]
struct PayloadParseResult {
    value: PacketPayload,
    bits_read: usize,
}

#[derive(Debug, PartialEq)]
pub struct TransmissionPacket {
    version: u8,
    type_id: u8,
    payload: PacketPayload,
}

#[derive(Debug)]
struct PacketParseResult {
    value: TransmissionPacket,
    bits_read: usize,
}

#[derive(Debug)]
struct PacketParseError {}

fn parse_literal_value_payload(bytes: &[u8], start_bit: usize) -> PayloadParseResult {
    let mut value = 0u64;
    let mut groups_remaining = true;
    let mut group_start_bit = start_bit;
    let mut bits_read = 0usize;
    while groups_remaining {
        let group_bits = extract_bits(bytes, group_start_bit, 5).unwrap();
        bits_read += 5;
        let partial_value = group_bits & 0b01111;
        value <<= 4;
        value += partial_value;
        groups_remaining = group_bits >> 4 == 1;
        group_start_bit += 5;
    }
    PayloadParseResult {
        value: PacketPayload::LiteralValue(value),
        bits_read,
    }
}

fn parse_operator_payload(bytes: &[u8], start_bit: usize) -> PayloadParseResult {
    let mut bits_read = 0usize;
    let mode = extract_bits(bytes, start_bit, 1).unwrap();
    bits_read += 1;
    let mut subpackets: Vec<TransmissionPacket> = Vec::new();

    if mode == 0 {
        let subpackets_bit_length = extract_bits(bytes, start_bit + 1, 15).unwrap() as usize;
        bits_read += 15;
        let mut subpacket_bits_read = 0usize;
        while subpacket_bits_read < subpackets_bit_length {
            let result =
                parse_transmission_packet(bytes, start_bit + bits_read + subpacket_bits_read)
                    .unwrap();
            subpackets.push(result.value);
            subpacket_bits_read += result.bits_read;
        }
        bits_read += subpacket_bits_read;
    } else {
        let subpacket_count = extract_bits(bytes, start_bit + 1, 11).unwrap() as usize;
        bits_read += 11;
        let mut subpackets_read = 0usize;
        let mut subpacket_bits_read = 0usize;
        while subpackets_read < subpacket_count {
            let result =
                parse_transmission_packet(bytes, start_bit + bits_read + subpacket_bits_read)
                    .unwrap();
            subpackets.push(result.value);
            subpackets_read += 1;
            subpacket_bits_read += result.bits_read;
        }
        bits_read += subpacket_bits_read;
    }

    PayloadParseResult {
        value: PacketPayload::Operator(subpackets),
        bits_read,
    }
}

fn parse_transmission_packet(
    bytes: &[u8],
    start_bit: usize,
) -> Result<PacketParseResult, PacketParseError> {
    let version = extract_bits(&bytes, start_bit, 3).unwrap() as u8;
    let type_id = extract_bits(&bytes, start_bit + 3, 3).unwrap() as u8;

    let bits_read = 6usize;

    if type_id == LITERAL_VALUE_TYPE_ID {
        let result = parse_literal_value_payload(bytes, start_bit + 6);
        return Ok(PacketParseResult {
            value: TransmissionPacket {
                version,
                type_id,
                payload: result.value,
            },
            bits_read: bits_read + result.bits_read,
        });
    }

    let result = parse_operator_payload(bytes, start_bit + 6);
    Ok(PacketParseResult {
        value: TransmissionPacket {
            version,
            type_id,
            payload: result.value,
        },
        bits_read: bits_read + result.bits_read,
    })
}

mod part_1 {
    use super::{PacketPayload, TransmissionPacket};

    pub fn add_all_packet_versions(packet: &TransmissionPacket) -> u64 {
        match &packet.payload {
            PacketPayload::LiteralValue(_) => packet.version as u64,
            PacketPayload::Operator(subpackets) => {
                let mut subpacket_sum = packet.version as u64;
                for subpacket in subpackets {
                    subpacket_sum += add_all_packet_versions(&subpacket);
                }
                return subpacket_sum;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;
        use super::*;

        #[test]
        fn test_version_sum_for_single_literal_value_packet() {
            let packet = TransmissionPacket {
                version: 7,
                type_id: LITERAL_VALUE_TYPE_ID,
                payload: PacketPayload::LiteralValue(115),
            };
            assert_eq!(7, add_all_packet_versions(&packet));
        }

        #[test]
        fn test_version_sum_for_operator_packet_with_literal_subpacket() {
            let packet = TransmissionPacket {
                version: 3,
                type_id: 0,
                payload: PacketPayload::Operator(vec![TransmissionPacket {
                    version: 6,
                    type_id: LITERAL_VALUE_TYPE_ID,
                    payload: PacketPayload::LiteralValue(115),
                }]),
            };
            assert_eq!(9, add_all_packet_versions(&packet));
        }

        #[test]
        fn test_version_sum_for_operator_packet_with_operator_packet_with_2_literal_subpackets() {
            let packet = TransmissionPacket {
                version: 3,
                type_id: 0,
                payload: PacketPayload::Operator(vec![TransmissionPacket {
                    version: 6,
                    type_id: 0,
                    payload: PacketPayload::Operator(vec![
                        TransmissionPacket {
                            version: 2,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(10),
                        },
                        TransmissionPacket {
                            version: 4,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(20),
                        },
                    ]),
                }]),
            };
            assert_eq!(15, add_all_packet_versions(&packet));
        }

        #[test]
        fn test_version_sum_for_8a004a801a8002f478_should_be_16() {
            let bytes = hex_str_to_u8_vec("8A004A801A8002F478");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(16, add_all_packet_versions(&packet));
        }

        #[test]
        fn test_version_sum_for_620080001611562c8802118e34_should_be_12() {
            let bytes = hex_str_to_u8_vec("620080001611562C8802118E34");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(12, add_all_packet_versions(&packet));
        }

        #[test]
        fn test_version_sum_for_c0015000016115a2e0802f182340_should_be_23() {
            let bytes = hex_str_to_u8_vec("C0015000016115A2E0802F182340");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(23, add_all_packet_versions(&packet));
        }

        #[test]
        fn test_version_sum_for_a0016c880162017c3686b18a3d4780_should_be_31() {
            let bytes = hex_str_to_u8_vec("A0016C880162017C3686B18A3D4780");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(31, add_all_packet_versions(&packet));
        }
    }
}

mod part_2 {
    use super::{PacketPayload, TransmissionPacket};

    #[derive(Debug, PartialEq)]
    enum Operation {
        Sum,
        Product,
        Minimum,
        Maximum,
        GreaterThan,
        LessThan,
        EqualTo,
    }

    impl TryFrom<u8> for Operation {
        type Error = String;
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Operation::Sum),
                1 => Ok(Operation::Product),
                2 => Ok(Operation::Minimum),
                3 => Ok(Operation::Maximum),
                5 => Ok(Operation::GreaterThan),
                6 => Ok(Operation::LessThan),
                7 => Ok(Operation::EqualTo),
                _ => Err(String::from("Value does not designate a valid Operation")),
            }
        }
    }

    pub fn evaluate_packet(packet: &TransmissionPacket) -> i64 {
        match &packet.payload {
            PacketPayload::LiteralValue(v) => *v as i64,
            PacketPayload::Operator(subpackets) => {
                let operation = Operation::try_from(packet.type_id).unwrap();
                let mut evaluated_subpackets = subpackets.iter().map(evaluate_packet);
                match operation {
                    Operation::Sum => evaluated_subpackets.sum::<i64>(),
                    Operation::Product => evaluated_subpackets.product::<i64>(),
                    Operation::Minimum => evaluated_subpackets.min().unwrap(),
                    Operation::Maximum => evaluated_subpackets.max().unwrap(),
                    Operation::GreaterThan => {
                        if evaluated_subpackets.nth(0).unwrap()
                            > evaluated_subpackets.nth(0).unwrap()
                        {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::LessThan => {
                        if evaluated_subpackets.nth(0).unwrap()
                            < evaluated_subpackets.nth(0).unwrap()
                        {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::EqualTo => {
                        if evaluated_subpackets.nth(0).unwrap()
                            == evaluated_subpackets.nth(0).unwrap()
                        {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;
        use super::*;

        fn make_test_packet(type_id: u8, subpacket_values: &[u64]) -> TransmissionPacket {
            TransmissionPacket {
                version: 0,
                type_id,
                payload: PacketPayload::Operator(
                    subpacket_values
                        .into_iter()
                        .map(|value| TransmissionPacket {
                            version: 0,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(*value),
                        })
                        .collect(),
                ),
            }
        }

        #[test]
        fn test_operation_from_u8() {
            assert_eq!(Ok(Operation::Sum), Operation::try_from(0u8));
            assert_eq!(Ok(Operation::Maximum), Operation::try_from(3u8));
            assert_eq!(
                Err(String::from("Value does not designate a valid Operation")),
                Operation::try_from(8u8)
            );
        }

        #[test]
        fn test_packet_sum_operation() {
            let packet = make_test_packet(0, &[4, 5]);
            assert_eq!(9, evaluate_packet(&packet));
        }

        #[test]
        fn test_packet_product_operation() {
            let packet = make_test_packet(1, &[4, 5]);
            assert_eq!(20, evaluate_packet(&packet));
        }

        #[test]
        fn test_sum_of_1_and_2() {
            let bytes = hex_str_to_u8_vec("C200B40A82");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(3, evaluate_packet(&packet));
        }

        #[test]
        fn test_product_of_6_and_9() {
            let bytes = hex_str_to_u8_vec("04005AC33890");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(54, evaluate_packet(&packet));
        }

        #[test]
        fn test_minimum_of_7_8_and_9() {
            let bytes = hex_str_to_u8_vec("880086C3E88112");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(7, evaluate_packet(&packet));
        }

        #[test]
        fn test_maximum_of_7_8_and_9() {
            let bytes = hex_str_to_u8_vec("CE00C43D881120");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(9, evaluate_packet(&packet));
        }

        #[test]
        fn test_5_less_than_15() {
            let bytes = hex_str_to_u8_vec("D8005AC2A8F0");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(1, evaluate_packet(&packet));
        }

        #[test]
        fn test_5_not_greater_than_15() {
            let bytes = hex_str_to_u8_vec("F600BC2D8F");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(0, evaluate_packet(&packet));
        }

        #[test]
        fn test_5_not_equal_to_15() {
            let bytes = hex_str_to_u8_vec("9C005AC2F8F0");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(0, evaluate_packet(&packet));
        }

        #[test]
        fn test_sum_of_1_and_3_equal_to_product_of_2_and_2() {
            let bytes = hex_str_to_u8_vec("9C0141080250320F1802104A08");
            let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
            assert_eq!(1, evaluate_packet(&packet));
        }
    }
}

fn main() {
    let input = read_to_string("data/day-16.txt").unwrap();
    let bytes = hex_str_to_u8_vec(input.trim_end());

    println!("== PART 1");
    let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
    println!(
        "Sum of version numbers of all packets: {}",
        part_1::add_all_packet_versions(&packet)
    );

    println!();

    println!("== PART 2");
    println!(
        "Evaluation of hexadecimal-encoded BITS transmission: {}",
        part_2::evaluate_packet(&packet)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_of_literal_value_packet() {
        let bytes = hex_str_to_u8_vec("D2FE28");
        let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
        assert_eq!(6, packet.version);
        assert_eq!(PacketPayload::LiteralValue(2021), packet.payload);
    }

    #[test]
    fn test_parsing_of_operator_packet_length_type_0_with_2_literal_values() {
        let bytes = hex_str_to_u8_vec("38006F45291200");
        let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
        assert_eq!(1, packet.version);
        let expected_subpackets = vec![
            TransmissionPacket {
                version: 6,
                type_id: LITERAL_VALUE_TYPE_ID,
                payload: PacketPayload::LiteralValue(10),
            },
            TransmissionPacket {
                version: 2,
                type_id: LITERAL_VALUE_TYPE_ID,
                payload: PacketPayload::LiteralValue(20),
            },
        ];
        assert_eq!(PacketPayload::Operator(expected_subpackets), packet.payload);
    }

    #[test]
    fn test_parsing_of_operator_packet_length_type_1_with_3_literal_values() {
        let bytes = hex_str_to_u8_vec("EE00D40C823060");
        let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
        assert_eq!(7, packet.version);
        let expected_subpackets = vec![
            TransmissionPacket {
                version: 2,
                type_id: LITERAL_VALUE_TYPE_ID,
                payload: PacketPayload::LiteralValue(1),
            },
            TransmissionPacket {
                version: 4,
                type_id: LITERAL_VALUE_TYPE_ID,
                payload: PacketPayload::LiteralValue(2),
            },
            TransmissionPacket {
                version: 1,
                type_id: LITERAL_VALUE_TYPE_ID,
                payload: PacketPayload::LiteralValue(3),
            },
        ];
        assert_eq!(PacketPayload::Operator(expected_subpackets), packet.payload);
    }

    #[test]
    fn test_parsing_of_operator_v4_with_operator_v1_with_operator_v5_with_literal_value() {
        let bytes = hex_str_to_u8_vec("8A004A801A8002F478");
        let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
        let expected = TransmissionPacket {
            version: 4,
            type_id: 2,
            payload: PacketPayload::Operator(vec![TransmissionPacket {
                version: 1,
                type_id: 2,
                payload: PacketPayload::Operator(vec![TransmissionPacket {
                    version: 5,
                    type_id: 2,
                    payload: PacketPayload::Operator(vec![TransmissionPacket {
                        version: 6,
                        type_id: LITERAL_VALUE_TYPE_ID,
                        payload: PacketPayload::LiteralValue(15),
                    }]),
                }]),
            }]),
        };
        assert_eq!(expected, packet);
    }

    #[test]
    fn test_parsing_of_operator_v3_with_2_operator_with_2_literal_values() {
        let bytes = hex_str_to_u8_vec("620080001611562C8802118E34");
        let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
        let expected = TransmissionPacket {
            version: 3,
            type_id: 0,
            payload: PacketPayload::Operator(vec![
                TransmissionPacket {
                    version: 0,
                    type_id: 0,
                    payload: PacketPayload::Operator(vec![
                        TransmissionPacket {
                            version: 0,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(10),
                        },
                        TransmissionPacket {
                            version: 5,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(11),
                        },
                    ]),
                },
                TransmissionPacket {
                    version: 1,
                    type_id: 0,
                    payload: PacketPayload::Operator(vec![
                        TransmissionPacket {
                            version: 0,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(12),
                        },
                        TransmissionPacket {
                            version: 3,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(13),
                        },
                    ]),
                },
            ]),
        };
        assert_eq!(expected, packet);
    }

    #[test]
    fn test_parsing_of_operator_v3_length_type_0_with_2_operator_with_2_literal_values() {
        let bytes = hex_str_to_u8_vec("C0015000016115A2E0802F182340");
        let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
        let expected = TransmissionPacket {
            version: 6,
            type_id: 0,
            payload: PacketPayload::Operator(vec![
                TransmissionPacket {
                    version: 0,
                    type_id: 0,
                    payload: PacketPayload::Operator(vec![
                        TransmissionPacket {
                            version: 0,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(10),
                        },
                        TransmissionPacket {
                            version: 6,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(11),
                        },
                    ]),
                },
                TransmissionPacket {
                    version: 4,
                    type_id: 0,
                    payload: PacketPayload::Operator(vec![
                        TransmissionPacket {
                            version: 7,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(12),
                        },
                        TransmissionPacket {
                            version: 0,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(13),
                        },
                    ]),
                },
            ]),
        };
        assert_eq!(expected, packet);
    }

    #[test]
    fn test_parsing_of_operator_with_operator_with_operator_with_5_literal_values() {
        let bytes = hex_str_to_u8_vec("A0016C880162017C3686B18A3D4780");
        let packet = parse_transmission_packet(&bytes, 0).unwrap().value;
        let expected = TransmissionPacket {
            version: 5,
            type_id: 0,
            payload: PacketPayload::Operator(vec![TransmissionPacket {
                version: 1,
                type_id: 0,
                payload: PacketPayload::Operator(vec![TransmissionPacket {
                    version: 3,
                    type_id: 0,
                    payload: PacketPayload::Operator(vec![
                        TransmissionPacket {
                            version: 7,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(6),
                        },
                        TransmissionPacket {
                            version: 6,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(6),
                        },
                        TransmissionPacket {
                            version: 5,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(12),
                        },
                        TransmissionPacket {
                            version: 2,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(15),
                        },
                        TransmissionPacket {
                            version: 2,
                            type_id: LITERAL_VALUE_TYPE_ID,
                            payload: PacketPayload::LiteralValue(15),
                        },
                    ]),
                }]),
            }]),
        };
        assert_eq!(expected, packet);
    }
}
