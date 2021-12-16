use aoc::aoc_input::get_input;
use bitreader::{BitReader, BitReaderError};
use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
enum OpType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

#[derive(Debug, Clone)]
enum PacketData {
    Literal { val: u64 },
    Operator { op: OpType, pkts: Vec<Packet> },
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    data: PacketData,
}

impl Packet {
    fn read_literal(reader: &mut BitReader) -> Result<u64, BitReaderError> {
        let mut res = 0u64;
        loop {
            let more = reader.read_bool()?;
            let nibble = reader.read_u64(4)?;
            res = (res << 4) | nibble;
            if !more {
                break Ok(res);
            }
        }
    }

    fn read_operator_packets(reader: &mut BitReader) -> Result<Vec<Packet>, BitReaderError> {
        let length_type_id = reader.read_bool()?;
        let mut res = Vec::new();
        match length_type_id {
            false => {
                let remaining_length = reader.read_u64(15)?;
                let end_pos = reader.position() + remaining_length;
                while reader.position() < end_pos {
                    res.push(Packet::read(reader)?);
                }
            }
            true => {
                let subpackets_count = reader.read_u16(11)?;
                for _ in 0..subpackets_count {
                    res.push(Packet::read(reader)?);
                }
            }
        }
        Ok(res)
    }

    fn read(reader: &mut BitReader) -> Result<Self, BitReaderError> {
        let version = reader.read_u8(3)?;
        let typeid: OpType = reader.read_u8(3)?.try_into().unwrap();
        let data = match typeid {
            OpType::Literal => PacketData::Literal {
                val: Self::read_literal(reader)?,
            },
            _ => PacketData::Operator {
                op: typeid,
                pkts: Self::read_operator_packets(reader)?,
            },
        };
        Ok(Self { version, data })
    }
}

fn version_sum(pkt: &Packet) -> u64 {
    pkt.version as u64
        + match &pkt.data {
            PacketData::Literal { val: _ } => 0u64,
            PacketData::Operator { op: _, pkts } => pkts.iter().map(version_sum).sum(),
        }
}

fn eval_op(op: OpType, pkts: &[Packet]) -> u64 {
    match op {
        OpType::Sum => pkts.iter().map(eval).sum(),
        OpType::Product => pkts.iter().map(eval).product(),
        OpType::Minimum => pkts.iter().map(eval).min().unwrap(),
        OpType::Maximum => pkts.iter().map(eval).max().unwrap(),
        OpType::Literal => unreachable!(),
        OpType::GreaterThan => (eval(&pkts[0]) > eval(&pkts[1])) as u64,
        OpType::LessThan => (eval(&pkts[0]) < eval(&pkts[1])) as u64,
        OpType::EqualTo => (eval(&pkts[0]) == eval(&pkts[1])) as u64,
    }
}

fn eval(pkt: &Packet) -> u64 {
    match &pkt.data {
        PacketData::Literal { val } => *val,
        PacketData::Operator { op, pkts } => eval_op(*op, pkts),
    }
}

fn main() {
    let input = get_input(2021, 16);
    let buf = hex::decode(input.trim()).unwrap();
    let mut reader = BitReader::new(&buf);
    let root = Packet::read(&mut reader).unwrap();

    println!("Version sum: {}", version_sum(&root));
    println!("Evaluation: {}", eval(&root));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
}
