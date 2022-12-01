use std::iter::{Product, Sum};

use crate::{utils::collect_n_bits, Solution};

pub struct Day16;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Sum,
    Prod,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

impl Operation {
    fn apply<T, R>(self, values: impl Iterator<Item = T>) -> R
    where
        T: Ord + Into<R>,
        bool: Into<R>,
        R: Sum<T> + Product<T>,
    {
        fn bin_op<T, R>(mut values: impl Iterator<Item = T>, op: impl FnOnce(T, T) -> R) -> R {
            let first = values.next().unwrap();
            let second = values.next().unwrap();
            op(first, second)
        }

        match self {
            Operation::Sum => values.sum(),
            Operation::Prod => values.product(),
            Operation::Min => values.min().unwrap().into(),
            Operation::Max => values.max().unwrap().into(),
            Operation::Gt => bin_op(values, |a, b| (a > b).into()),
            Operation::Lt => bin_op(values, |a, b| (a < b).into()),
            Operation::Eq => bin_op(values, |a, b| (a == b).into()),
        }
    }
}

#[derive(Debug)]
enum PacketValue {
    Value(u64),

    Operator {
        operation: Operation,
        subpackets: Vec<Packet>,
    },
}

impl PacketValue {
    /// Try to extract a packet value from an iterator of bits.
    /// If the bits begins with a complete packet value, only its bits will be popped out of the iterator.
    /// On other cases, the iterator may be removed of an undefined number of bits and None will be returned.
    fn try_extract_from_bits(bits: &mut dyn Iterator<Item = bool>) -> Option<Self> {
        // Parse the packet type id
        let type_id = collect_n_bits(bits, 3)?;

        match type_id {
            4 => Self::parse_value(bits),
            id => Self::parse_operator(bits, id),
        }
    }

    /// Parse a value from the bits.
    /// A value is composed of packets of 5 bits, each one beggining
    /// with a 1, except the last one.
    fn parse_value(bits: &mut dyn Iterator<Item = bool>) -> Option<Self> {
        let mut value = 0;

        loop {
            let last = !bits.next()?;
            let num: u64 = collect_n_bits(bits, 4)?;

            value <<= 4;
            value |= num;

            if last {
                break;
            }
        }

        Some(Self::Value(value))
    }

    /// Parse an operator from the bits.
    /// A value is composed of packets of 5 bits, each one beggining
    /// with a 1, except the last one.
    fn parse_operator(bits: &mut dyn Iterator<Item = bool>, id: u8) -> Option<PacketValue> {
        let length_id = bits.next()?;
        let subpackets = match length_id {
            false => {
                // Subpackets counted by length
                let len_subpackets: u16 = collect_n_bits(bits, 15)?;
                let mut sub_bits = bits.take(len_subpackets as _);
                Day16::parse_packets(&mut sub_bits)
            }
            true => {
                // Subpackets counted by number
                let n_subpackets: u16 = collect_n_bits(bits, 11)?;
                let mut subpackets = Vec::with_capacity(n_subpackets as _);
                for _ in 0..n_subpackets {
                    let packet = Packet::try_extract_from_bits(bits)?;
                    subpackets.push(packet);
                }
                subpackets
            }
        };

        let operation = match id {
            0 => Operation::Sum,
            1 => Operation::Prod,
            2 => Operation::Min,
            3 => Operation::Max,
            5 => Operation::Gt,
            6 => Operation::Lt,
            7 => Operation::Eq,
            _ => unreachable!(),
        };

        Some(Self::Operator {
            operation,
            subpackets,
        })
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    value: PacketValue,
}

impl Packet {
    /// Try to extract a packet from an iterator of bits.
    /// If the bits begins with a complete packet, only its bits will be popped out of the iterator.
    /// On other cases, the iterator may be removed of an undefined number of bits and None will be returned.
    fn try_extract_from_bits(bits: &mut dyn Iterator<Item = bool>) -> Option<Self> {
        // Parse the version
        let version = collect_n_bits(bits, 3)?;

        // Parse the value
        let value = PacketValue::try_extract_from_bits(bits)?;

        Some(Self { version, value })
    }
}

impl Solution for Day16 {
    /// Decode the packets and add up all version numbers.
    fn q1(&self, data: &str) -> String {
        let packet = Self::parse_data(data).expect("Could not parse data into packet");

        fn visit(packet: &Packet) -> u64 {
            packet.version as u64
                + match &packet.value {
                    PacketValue::Value(_) => 0,
                    PacketValue::Operator { subpackets, .. } => subpackets.iter().map(visit).sum(),
                }
        }

        let sum = visit(&packet);
        sum.to_string()
    }

    /// Decode the packets and evaluate the operations.
    fn q2(&self, data: &str) -> String {
        let packet = Self::parse_data(data).expect("Could not parse data into packet");

        fn visit(packet: &Packet) -> u64 {
            match &packet.value {
                PacketValue::Value(v) => *v,
                PacketValue::Operator {
                    operation,
                    subpackets,
                } => operation.apply(subpackets.iter().map(visit)),
            }
        }

        let sum = visit(&packet);
        sum.to_string()
    }
}

impl Day16 {
    /// Decode the hexadecimal string into a single packet.
    /// Additional bits are ignored.
    fn parse_data(data: &str) -> Option<Packet> {
        let mut bits = Self::parse_hex_to_bits(data.trim().chars());
        Packet::try_extract_from_bits(&mut bits)
    }

    /// Parse a iterator of bits to a list of packets.
    fn parse_packets(bits: &mut dyn Iterator<Item = bool>) -> Vec<Packet> {
        let mut packets = vec![];
        while let Some(packet) = Packet::try_extract_from_bits(bits) {
            packets.push(packet);
        }
        packets
    }

    /// Parse hexadecimal characters into their bits representation
    fn parse_hex_to_bits(chars: impl Iterator<Item = char>) -> impl Iterator<Item = bool> {
        chars
            .map(|c| c.to_digit(16).unwrap() as u8)
            .flat_map(|n| (0..4).rev().map(move |i| ((n >> i) & 1) != 0))
    }
}
