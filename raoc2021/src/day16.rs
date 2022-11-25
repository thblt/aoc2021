use lib::*;
// use std::cmp::{max,min};

struct Packet {
    version: u8,
    payload: PacketData,
}

impl Packet {
    fn eval(&self) -> u128 {
        match &self.payload {
            PacketData::Literal(value) => *value,
            PacketData::Operator(operator) =>
                match operator.op {
                    Operation::Sum => operator.packets.iter().map(|p| p.eval()).sum(),
                    Operation::Product => operator.packets.iter().map(|p| p.eval()).product(),
                    Operation::Min => operator.packets.iter().map(|p| p.eval()).min().unwrap(),
                    Operation::Max => operator.packets.iter().map(|p| p.eval()).max().unwrap(),
                    Operation::Gt => {
                        if operator.packets[0].eval() > operator.packets[1].eval() { 1 } else { 0 }
                    },
                    Operation::Lt => {
                        if operator.packets[0].eval() < operator.packets[1].eval() { 1 } else { 0 }
                    },
                    Operation::Eq => {
                        if operator.packets[0].eval() == operator.packets[1].eval() { 1 } else { 0 }
                    },
                }
        }
    }

    fn dump(&self) -> u128 {
        self.do_dump(0)
}

    fn do_dump(&self, depth: u32) -> u128 {
        let mut vsum = self.version as u128;
        print!("{}", (0..depth * 2).map(|_| " ").collect::<String>());
        print!(
            "{} v. {}: ",
            match self.payload {
                PacketData::Literal(_) => "Literal ",
                PacketData::Operator(_) => "Operator",
            },
            self.version
        );
        match &self.payload {
            PacketData::Literal(value) => println!("v={}", value),
            PacketData::Operator(operator) => {
                println!("({:?})", operator.op);
                for p in &operator.packets {
                    vsum += p.do_dump(depth + 1) as u128;
                }
            }
        }
        vsum
    }
}

enum PacketData {
    Literal(u128),
    Operator(OperatorData),
}

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

struct OperatorData {
    op: Operation,
    packets: Vec<Packet>,
}

impl From<u8> for Operation {
    fn from(v: u8) -> Operation {
        use Operation::*;
        match v {
            0 => Sum,
            1 => Product,
            2 => Min,
            3 => Max,
            5 => Gt,
            6 => Lt,
            7 => Eq,
            _ => panic!("Bad operation"),
        }
    }
}

struct BitStream {
    /// The raw "stream".
    data: Vec<u8>,
    /// The byte index.
    index: usize,
    /// The bit index in data[index], that is, how many bits we've
    /// read at the index.
    bit: usize,
    read_counter: usize
}

impl BitStream {
    fn new(data: Vec<u8>) -> BitStream {
        BitStream {
            data,
            index: 0,
            bit: 0,
            read_counter: 0,
        }
    }

    fn read(&mut self, count: usize) -> u128 {
        self.do_read(count, 0)
    }

    /// @FIXME This should be parametric over the return type.
    fn do_read(&mut self, count: usize, base: u128) -> u128 {
        let avail = 8 - self.bit;
        if count == avail {
            // We need what we have, we just grab it.
            let ret = (base << avail) | (self.data[self.index] as u128);
            self.index += 1;
            self.bit = 0;
            self.read_counter += count;
            return ret;
        } else if count < avail {
            // We need *less*, which will take some cleanup.
            let ret = (base << count) | (self.data[self.index] >> (avail - count)) as u128;
            self.bit += count;
            self.read_counter += count;
            self.data[self.index] &= 0b11111111 >> self.bit;
            return ret;
        } else {
            // count > avail: we need more: we consume what we have,
            // then recurse.
            let base = self.do_read(avail, base);
            return self.do_read(count - avail, base);
        }
    }
}

fn decode(bs: &mut BitStream) -> Packet {
    // All the exemples pad the packets to an even number of hex
    // digits, that is to full bytes.  We shouldn't have to deal with
    // packets that start in the middle of a byte.
    let version = bs.read(3) as u8;
    let typ = bs.read(3) as u8;
    // println!("Protocol version {}, {} packet.", version, typ);
    Packet {
        version,
        payload: match typ {
            4 => decode_literal(bs),
            _ => decode_operator(Operation::from(typ), bs),
        },
    }
}

fn decode_literal(bs: &mut BitStream) -> PacketData {
    let mut ret: u128 = 0;
    let mut last;
    loop {
        last = bs.read(1) == 0;
        ret <<= 4;
        ret |= bs.read(4);
        if last {
            break;
        }
    }
    PacketData::Literal(ret)
}

fn decode_operator(op: Operation, bs: &mut BitStream) -> PacketData {
    let mut packets: Vec<Packet> = vec![];

    if bs.read(1) == 0 {
        // Length is a number of bits
        let length = bs.read(15) as usize;
        let end_count = bs.read_counter + length;
        loop {
            packets.push(decode(bs));
            if bs.read_counter >= end_count {
                break;
            }
        }
    } else {
        // Length is a count of packets.
        for _ in 0..bs.read(11) {
            packets.push(decode(bs));
        }
    }
    return PacketData::Operator(OperatorData { op, packets });
}

fn parse_hex_string(s: &str) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![];
    let mut even = true;
    let mut value: u8 = 0;

    for byte in String::from(s).chars().map(|b| read_digit(b)) {
        if even {
            value = byte << 4
        } else {
            value |= byte;
            ret.push(value);
            // println!("{:02x}", value);
        }
        even = !even;
    }
    ret
}

fn main() {
    let data = parse_hex_string(
        &read_lines("../inputs/16.txt")
            .unwrap()
            .next()
            .unwrap()
            .unwrap(),
    );

    let mut bs = BitStream::new(data);
    let root = decode(&mut bs);
    println!("Stream at {},{} out of {}", bs.index, bs.bit, bs.data.len());
    println!("Part A: version sum = {}", root.dump());
    println!("Part B: total value = {}", root.eval());
}
