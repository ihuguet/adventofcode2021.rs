use std::num::ParseIntError;

const INPUT: &str = &"60552F100693298A9EF0039D24B129BA56D67282E600A4B5857002439CE580E5E5AEF67803600D2E294B2FCE8AC489BAEF37FEACB31A678548034EA0086253B183F4F6BDDE864B13CBCFBC4C10066508E3F4B4B9965300470026E92DC2960691F7F3AB32CBE834C01A9B7A933E9D241003A520DF316647002E57C1331DFCE16A249802DA009CAD2117993CD2A253B33C8BA00277180390F60E45D30062354598AA4008641A8710FCC01492FB75004850EE5210ACEF68DE2A327B12500327D848028ED0046661A209986896041802DA0098002131621842300043E3C4168B12BCB6835C00B6033F480C493003C40080029F1400B70039808AC30024C009500208064C601674804E870025003AA400BED8024900066272D7A7F56A8FB0044B272B7C0E6F2392E3460094FAA5002512957B98717004A4779DAECC7E9188AB008B93B7B86CB5E47B2B48D7CAD3328FB76B40465243C8018F49CA561C979C182723D769642200412756271FC80460A00CC0401D8211A2270803D10A1645B947B3004A4BA55801494BC330A5BB6E28CCE60BE6012CB2A4A854A13CD34880572523898C7EDE1A9FA7EED53F1F38CD418080461B00440010A845152360803F0FA38C7798413005E4FB102D004E6492649CC017F004A448A44826AB9BFAB5E0AA8053306B0CE4D324BB2149ADDA2904028600021909E0AC7F0004221FC36826200FC3C8EB10940109DED1960CCE9A1008C731CB4FD0B8BD004872BC8C3A432BC8C3A4240231CF1C78028200F41485F100001098EB1F234900505224328612AF33A97367EA00CC4585F315073004E4C2B003530004363847889E200C45985F140C010A005565FD3F06C249F9E3BC8280804B234CA3C962E1F1C64ADED77D10C3002669A0C0109FB47D9EC58BC01391873141197DCBCEA401E2CE80D0052331E95F373798F4AF9B998802D3B64C9AB6617080";
const HDR_LEN: usize = 6;
const MIN_VALID_LIT_LEN: usize = 11;
const MIN_VALID_OP0_LEN: usize = 22 + MIN_VALID_LIT_LEN;
const MIN_VALID_OP1_LEN: usize = 18 + MIN_VALID_LIT_LEN;

#[derive(Clone, PartialEq)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

#[derive(Clone, PartialEq)]
struct LiteralPacket {
    version: u8,
    digits: Vec<u8>,
    bits_len: usize,
}

#[derive(Clone, PartialEq)]
struct OperatorPacket {
    version: u8,
    operator_id: u8,
    subpackets: Vec<Packet>,
    bits_len: usize,
}

fn main() {
    let bin_digits = parse_input(INPUT);
    let packets = parse_all_packets(&bin_digits).unwrap();
    let sum = sum_versions(&packets);
    println!("Part 1: versions sum={}", sum);

    assert_eq!(packets.len(), 1);
    println!("Part 2: result={}", packets[0].value());
}

fn sum_versions(packets: &Vec<Packet>) -> u32 {
    let mut sum = 0;
    for pkt in packets {
        sum += pkt.version() as u32;
        if let Packet::Operator(pkt) = pkt {
            sum += sum_versions(&pkt.subpackets);
        }
    }
    sum
}

fn parse_all_packets(buf: &str) -> Result<Vec<Packet>, ParseError> {
    let mut i = 0;
    let mut packets = Vec::new();
    while let Some(pkt) = parse_next_packet(&buf[i..])? {
        i += pkt.bits_len();
        packets.push(pkt);
        if i == buf.len() {
            break;
        }
    }
    Ok(packets)
}

fn parse_n_packets(buf: &str, num: usize) -> Result<Vec<Packet>, ParseError> {
    let mut i = 0;
    let mut packets = Vec::new();
    while packets.len() < num {
        let pkt = parse_next_packet(&buf[i..])?
                    .ok_or(ParseError { what: "Short packet".to_string() })?;
        i += pkt.bits_len();
        packets.push(pkt);
    }
    Ok(packets)
}

fn parse_next_packet(buf: &str) -> Result<Option<Packet>, ParseError> {
    if buf.len() < HDR_LEN {
        return Ok(None);
    }

    let type_str = (&buf[3..6], &buf[6..7]);
    let opt = match type_str {
        ("100", _) => {
            let pkt_opt = parse_next_packet_literal(buf)?;
            pkt_opt.map(|pkt| Packet::Literal(pkt))
        },
        (_, "0") => {
            let pkt_opt = parse_next_packet_operator0(buf)?;
            pkt_opt.map(|pkt| Packet::Operator(pkt))
        },
        (_, "1") => {
            let pkt_opt = parse_next_packet_operator1(buf)?;
            pkt_opt.map(|pkt| Packet::Operator(pkt))
        },
        _ => panic!(),
    };

    Ok(opt)
}

fn parse_next_packet_literal(buf: &str) -> Result<Option<LiteralPacket>, ParseError> {
    if buf.len() < MIN_VALID_LIT_LEN {
        return Ok(None);
    }

    let version = &buf[..3];
    let version = u8::from_str_radix(version, 2)?;

    let mut i = 6;
    let mut next = true;
    let mut digits = Vec::new();
    while next {
        digits.push(u8::from_str_radix(&buf[i + 1..i + 5], 2)?);
        next = &buf[i..i+1] == "1";
        i += 5;
    }

    Ok(Some(LiteralPacket {version, digits, bits_len: i}))
}

fn parse_next_packet_operator0(buf: &str) -> Result<Option<OperatorPacket>, ParseError> {
    if buf.len() < MIN_VALID_OP0_LEN {
        return Ok(None);
    }

    let version = &buf[..3];
    let version = u8::from_str_radix(version, 2)?;

    let operator_id = &buf[3..6];
    let operator_id = u8::from_str_radix(operator_id, 2)?;

    let subpackets_len = &buf[7..22];
    let subpackets_len = usize::from_str_radix(subpackets_len, 2)?;
    let subpackets_bits = &buf[22..22 + subpackets_len];
    let subpackets = parse_all_packets(subpackets_bits)?;

    let bits_len = subpackets_len + 22;

    Ok(Some(OperatorPacket { version, operator_id, subpackets, bits_len }))
}

fn parse_next_packet_operator1(buf: &str) -> Result<Option<OperatorPacket>, ParseError> {
    if buf.len() < MIN_VALID_OP1_LEN {
        return Ok(None);
    }

    let version = &buf[..3];
    let version = u8::from_str_radix(version, 2)?;

    let operator_id = &buf[3..6];
    let operator_id = u8::from_str_radix(operator_id, 2)?;

    let subpackets_num = &buf[7..18];
    let subpackets_num = usize::from_str_radix(subpackets_num, 2)?;
    let subpackets = parse_n_packets(&buf[18..], subpackets_num)?;

    let bits_len = 18 + subpackets.iter().map(|p| p.bits_len()).sum::<usize>();

    Ok(Some(OperatorPacket { version, operator_id, subpackets, bits_len }))
}

impl Packet {
    fn version(&self) -> u8 {
        match self {
            Packet::Literal(p) => p.version,
            Packet::Operator(p) => p.version,
        }
    }

    fn bits_len(&self) -> usize {
        match self {
            Packet::Literal(p) => p.bits_len,
            Packet::Operator(p) => p.bits_len,
        }
    }

    fn value(&self) -> u64 {
        match self {
            Packet::Literal(p) => p.value(),
            Packet::Operator(p) => p.value(),
        }
    }
}

impl LiteralPacket {
    fn value(&self) -> u64 {
        self.digits.iter().fold(0, |acc, v| (acc << 4) | *v as u64)
    }
}

impl OperatorPacket {
    fn value(&self) -> u64 {
        assert!(self.subpackets.len() > 0);
        match self.operator_id {
            0 => self.subpackets.iter().map(|p| p.value()).sum(),
            1 => self.subpackets.iter().map(|p| p.value()).product(),
            2 => self.subpackets.iter().map(|p| p.value()).min().unwrap(),
            3 => self.subpackets.iter().map(|p| p.value()).max().unwrap(),
            5 => if self.subpackets[0].value() > self.subpackets[1].value() { 1 } else { 0 },
            6 => if self.subpackets[0].value() < self.subpackets[1].value() { 1 } else { 0 },
            7 => if self.subpackets[0].value() == self.subpackets[1].value() { 1 } else { 0 },
            _ => panic!(),
        }
    }
}

fn parse_input(input: &str) -> String {
    let mut binstr = String::with_capacity(4 * input.len());
    for digitstr in input.bytes().map(hex_ch_to_bin) {
        binstr.push_str(&digitstr);
    }
    binstr
}

fn hex_ch_to_bin(ch: u8) -> String {
    let num = match ch {
        b'0'..=b'9' => ch - b'0',
        b'A'..=b'F' => ch - b'A' + 10,
        _ => panic!(),
    };
    format!("{:04b}", num)
}

#[derive(Clone, Debug)]
struct ParseError {
    what: String,
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError {
            what: format!("{:#?}", e.kind()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let hex_digits = parse_input("D2FE28");
        assert_eq!(hex_digits, "110100101111111000101000");
        let hex_digits = parse_input("38006F45291200");
        assert_eq!(hex_digits, "00111000000000000110111101000101001010010001001000000000");
    }

    #[test]
    fn literal_packet() {
        let bin_digits = parse_input("D2FE28AA");
        let packet = parse_next_packet(&bin_digits).unwrap().unwrap();

        let expected_packet = Packet::Literal(LiteralPacket {
            version: 6, digits: vec![7, 14, 5], bits_len: 21,
        });
        assert_eq!(packet, expected_packet);
        assert_eq!(packet.value(), 2021);
    }

    #[test]
    fn operator_0_packet() {
        let bin_digits = parse_input("38006F45291200");
        let packet = parse_next_packet(&bin_digits).unwrap().unwrap();

        let expected_packet = Packet::Operator(OperatorPacket {
            version: 1,
            operator_id: 6,
            subpackets: vec![
                Packet::Literal(LiteralPacket { version: 6, digits: vec![10], bits_len: 11 }),
                Packet::Literal(LiteralPacket { version: 2, digits: vec![1, 4], bits_len: 16 }),
            ],
            bits_len: 49,
        });
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn operator_1_packet() {
        let bin_digits = parse_input("EE00D40C823060");
        let packet = parse_next_packet(&bin_digits).unwrap().unwrap();

        let expected_packet = Packet::Operator(OperatorPacket {
            version: 7,
            operator_id: 3,
            subpackets: vec![
                Packet::Literal(LiteralPacket { version: 2, digits: vec![1], bits_len: 11 }),
                Packet::Literal(LiteralPacket { version: 4, digits: vec![2], bits_len: 11 }),
                Packet::Literal(LiteralPacket { version: 1, digits: vec![3], bits_len: 11 }),
            ],
            bits_len: 51,
        });
        assert_eq!(packet, expected_packet);
    }

    // C200B40A82 finds the sum of 1 and 2, resulting in the value 3.
    // 04005AC33890 finds the product of 6 and 9, resulting in the value 54.
    // 880086C3E88112 finds the minimum of 7, 8, and 9, resulting in the value 7.
    // CE00C43D881120 finds the maximum of 7, 8, and 9, resulting in the value 9.
    // D8005AC2A8F0 produces 1, because 5 is less than 15.
    // F600BC2D8F produces 0, because 5 is not greater than 15.
    // 9C005AC2F8F0 produces 0, because 5 is not equal to 15.
    // 9C0141080250320F1802104A08 produces 1, because 1 + 3 = 2 * 2

    #[test]
    fn sum() {
        let bin_digits = parse_input("C200B40A82");
        let packets = parse_all_packets(&bin_digits).unwrap();
        assert_eq!(packets[0].value(), 3);
    }

    #[test]
    fn mult() {
        let bin_digits = parse_input("04005AC33890");
        let packets = parse_all_packets(&bin_digits).unwrap();
        assert_eq!(packets[0].value(), 54);
    }

    #[test]
    fn min() {
        let bin_digits = parse_input("880086C3E88112");
        let packets = parse_all_packets(&bin_digits).unwrap();
        assert_eq!(packets[0].value(), 7);
    }

    #[test]
    fn max() {
        let bin_digits = parse_input("CE00C43D881120");
        let packets = parse_all_packets(&bin_digits).unwrap();
        assert_eq!(packets[0].value(), 9);
    }

    #[test]
    fn lt() {
        let bin_digits = parse_input("D8005AC2A8F0");
        let packets = parse_all_packets(&bin_digits).unwrap();
        assert_eq!(packets[0].value(), 1);
    }

    #[test]
    fn gt() {
        let bin_digits = parse_input("F600BC2D8F");
        let packets = parse_all_packets(&bin_digits).unwrap();
        assert_eq!(packets[0].value(), 0);
    }

    #[test]
    fn eq() {
        let bin_digits = parse_input("9C005AC2F8F0");
        let packets = parse_all_packets(&bin_digits).unwrap();
        assert_eq!(packets[0].value(), 0);
    }

    #[test]
    fn sum_mult_eq() {
        let bin_digits = parse_input("9C0141080250320F1802104A08");
        let packets = parse_all_packets(&bin_digits).unwrap();
        assert_eq!(packets[0].value(), 1);
    }
}
