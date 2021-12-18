use crate::utils::Input;
use anyhow::{bail, Result};
use std::collections::VecDeque;

pub fn run(input: &Input) -> Result<(u32, i64)> {
    let mut reader = BinaryReader::new(input.all().chars());
    if let Some(packet) = reader.read_packet() {
        Ok((packet.sum_versions(), packet.compute_value()))
    } else {
        bail!("invalid input")
    }
}

#[derive(Debug)]
struct Packet {
    version: u32,
    kind: PacketKind,
    packets: Vec<Packet>,
}

#[derive(Debug)]
enum PacketKind {
    Invalid,
    LiteralValue(i64),
    Sum,
    Product,
    Minimum,
    Maximum,
    Gt,
    Lt,
    Eq,
}

impl Packet {
    fn compute_value(&self) -> i64 {
        match &self.kind {
            PacketKind::Invalid => panic!("invalid packet"),
            PacketKind::LiteralValue(v) => *v,
            PacketKind::Sum | PacketKind::Product | PacketKind::Minimum | PacketKind::Maximum => {
                let values = self.packets.iter().map(|p| p.compute_value());
                match &self.kind {
                    PacketKind::Sum => values.sum(),
                    PacketKind::Product => values.product(),
                    PacketKind::Minimum => values.min().unwrap(),
                    PacketKind::Maximum => values.max().unwrap(),
                    _ => panic!("unexpected"),
                }
            }
            PacketKind::Gt | PacketKind::Lt | PacketKind::Eq => {
                assert_eq!(2, self.packets.len());
                let (left, right) = (
                    self.packets[0].compute_value(),
                    self.packets[1].compute_value(),
                );
                (match &self.kind {
                    PacketKind::Gt => left > right,
                    PacketKind::Lt => left < right,
                    PacketKind::Eq => left == right,
                    _ => panic!("unexpected"),
                }) as i64
            }
        }
    }

    fn sum_versions(&self) -> u32 {
        self.version + self.packets.iter().map(|p| p.sum_versions()).sum::<u32>()
    }
}

struct BinaryReader<T> {
    input: T,
    read_bits: u32,
    buffer: VecDeque<bool>,
}

impl<T> BinaryReader<T>
where
    T: Iterator<Item = char>,
{
    fn new(input: T) -> Self {
        Self {
            input,
            read_bits: 0,
            buffer: VecDeque::with_capacity(4),
        }
    }

    /// Read the next char and send bits to the buffer, return false when reaching end of input
    fn fill_buffer(&mut self) -> bool {
        match self.input.next() {
            None => false,
            Some(c) => {
                let mut bits = if c.is_ascii_digit() {
                    c as u8 - 48
                } else if c.is_ascii_uppercase() {
                    c as u8 - 65 + 10
                } else {
                    panic!("unexpected char {}", c)
                };
                for _i in 0..4 {
                    self.buffer.push_front(bits & 1 == 1);
                    bits >>= 1;
                }
                true
            }
        }
    }

    /// Read N bits from the input and return them as a little-endian integer
    fn read_n(&mut self, length: u32) -> Option<u32> {
        let mut out = 0;
        for _i in 0..length {
            out <<= 1;
            if self.buffer.is_empty() && !self.fill_buffer() {
                return None;
            }
            if let Some(true) = self.buffer.pop_front() {
                out += 1;
            }
        }
        self.read_bits += length;
        Some(out)
    }

    /// Read one packet and its nested sub-packets
    fn read_packet(&mut self) -> Option<Packet> {
        let mut packet = Packet {
            version: 0,
            kind: PacketKind::Invalid,
            packets: vec![],
        };
        packet.version = self.read_n(3)?;
        match self.read_n(3) {
            Some(4) => {
                let mut value: i64 = 0;
                let mut not_end = true;
                while not_end {
                    value <<= 4;
                    not_end = self.read_n(1)? > 0;
                    let part = self.read_n(4)?;
                    value += part as i64;
                }
                packet.kind = PacketKind::LiteralValue(value);
            }
            Some(k) => {
                packet.kind = match k {
                    0 => PacketKind::Sum,
                    1 => PacketKind::Product,
                    2 => PacketKind::Minimum,
                    3 => PacketKind::Maximum,
                    5 => PacketKind::Gt,
                    6 => PacketKind::Lt,
                    7 => PacketKind::Eq,
                    _ => PacketKind::Invalid,
                };
                match self.read_n(1) {
                    Some(0) => {
                        let end = self.read_n(15)? + self.read_bits;
                        while self.read_bits < end {
                            packet.packets.push(self.read_packet()?);
                        }
                    }
                    Some(1) => {
                        let packet_count = self.read_n(11)?;
                        for _i in 0..packet_count {
                            packet.packets.push(self.read_packet()?);
                        }
                    }
                    _ => packet.kind = PacketKind::Invalid,
                }
            }
            None => return None,
        }
        Some(packet)
    }
}

#[test]
fn test_sum_versions() -> Result<()> {
    let _ = pretty_env_logger::try_init();
    for (input, output) in vec![
        ("8A004A801A8002F478", 16),
        ("620080001611562C8802118E34", 12),
        ("C0015000016115A2E0802F182340", 23),
        ("A0016C880162017C3686B18A3D4780", 31),
    ] {
        let mut b = BinaryReader::new(input.chars());
        assert_eq!(output, b.read_packet().unwrap().sum_versions());
    }
    Ok(())
}

#[test]
fn test_compute_value() -> Result<()> {
    let _ = pretty_env_logger::try_init();
    for (input, output) in vec![
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ] {
        let mut b = BinaryReader::new(input.chars());
        assert_eq!(output, b.read_packet().unwrap().compute_value());
    }
    Ok(())
}
