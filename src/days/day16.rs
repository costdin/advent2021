use super::get_input_lines;

pub fn day16() {
    let line = get_input_lines(16).into_iter().nth(0).unwrap();
    let buffer = line
        .as_bytes()
        .chunks(2)
        .map(decode_byte)
        .collect::<Vec<_>>();

    let bitarray = BitArray::new(buffer.clone());

    let (packet, _) = decode_packet(&buffer[..], 0);

    println!(
        "DAY 16\nSolution 1: {}\nSolution 2: {}",
        packet.sum_versions(),
        packet.process()
    );
}

struct Packet {
    version: u16,
    payload: Payload,
}

enum Payload {
    Literal(u128),
    Operator(OperatorType, Vec<Packet>),
}

enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<u16> for OperatorType {
    fn from(n: u16) -> Self {
        match n {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::EqualTo,
            _ => unreachable!(),
        }
    }
}

impl Packet {
    fn sum_versions(&self) -> u32 {
        match &self.payload {
            Payload::Literal(_) => self.version as u32,
            Payload::Operator(_, sub_packets) => {
                self.version as u32 + sub_packets.iter().map(|p| p.sum_versions()).sum::<u32>()
            }
        }
    }

    fn process(&self) -> u128 {
        match &self.payload {
            Payload::Literal(value) => *value as u128,
            Payload::Operator(operator_type, sub_packets) => match operator_type {
                OperatorType::Sum => sub_packets.iter().map(|p| p.process()).sum(),
                OperatorType::Product => sub_packets.iter().map(|p| p.process()).product(),
                OperatorType::Minimum => sub_packets.iter().map(|p| p.process()).min().unwrap(),
                OperatorType::Maximum => sub_packets.iter().map(|p| p.process()).max().unwrap(),
                OperatorType::GreaterThan => {
                    if sub_packets[0].process() > sub_packets[1].process() {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::LessThan => {
                    if sub_packets[0].process() < sub_packets[1].process() {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::EqualTo => {
                    if sub_packets[0].process() == sub_packets[1].process() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

fn decode_packet(buffer: &[u8], offset: usize) -> (Packet, usize) {
    let version = take_bits(buffer, offset, 3);
    let type_id = take_bits(buffer, offset + 3, 3);

    if type_id == 4 {
        let (value, offset) = decode_literal(buffer, offset + 6);
        let packet = Packet {
            version,
            payload: Payload::Literal(value),
        };

        (packet, offset)
    } else {
        let lenght_type = take_bits(buffer, offset + 6, 1);

        let (sub_packets, offset) = if lenght_type == 0 {
            decode_sized_operator(buffer, offset + 7)
        } else {
            decode_count_operator(buffer, offset + 7)
        };

        let packet = Packet {
            version,
            payload: Payload::Operator(type_id.into(), sub_packets),
        };

        (packet, offset)
    }
}

fn decode_sized_operator(buffer: &[u8], mut offset: usize) -> (Vec<Packet>, usize) {
    let length = take_bits(buffer, offset, 15);
    offset += 15;
    let end = offset + length as usize;

    let mut packets = vec![];

    while offset < end {
        let (packet, new_offset) = decode_packet(buffer, offset);
        offset = new_offset;
        packets.push(packet);
    }

    (packets, offset)
}

fn decode_count_operator(buffer: &[u8], mut offset: usize) -> (Vec<Packet>, usize) {
    let count = take_bits(buffer, offset, 11) as usize;
    offset += 11;

    let mut packets = vec![];
    while packets.len() < count {
        let (packet, new_offset) = decode_packet(buffer, offset);
        offset = new_offset;
        packets.push(packet);
    }

    (packets, offset)
}

fn decode_literal(buffer: &[u8], mut offset: usize) -> (u128, usize) {
    let mut value = 0u128;

    loop {
        let block = take_bits(buffer, offset, 5);
        value = (value << 4) + (block & 0x0F) as u128;

        offset += 5;

        if block & 0x10 == 0 {
            break;
        }
    }

    (value, offset)
}

static MASKS: &'static [u8] = &[0, 1, 3, 7, 15, 31, 63, 127, 255];

struct BitArray {
    buffer: Vec<u8>,
    offset: usize,
    ptr: *const u8,
}

impl BitArray {
    fn new(buffer: Vec<u8>) -> BitArray {
        let ptr = buffer.as_ptr();

        BitArray {
            buffer,
            offset: 0,
            ptr,
        }
    }

    fn take_bits(&mut self, count: usize) -> u16 {
        let o = count + self.offset;
        if o < 8 {
            self.offset += count;

            ((unsafe { *self.ptr } >> (8 - o)) & MASKS[count]) as u16
        } else if o == 8 {
            self.offset = (self.offset + count) % 8;

            let r = ((unsafe { *self.ptr } >> (8 - o)) & MASKS[count]) as u16;
            self.ptr = unsafe { self.ptr.offset(1) };

            r
        } else if count <= 8 {
            let left_bit = 8 - self.offset;
            let right_shift = 8 - (count - left_bit);
            self.offset = (self.offset + count) % 8;

            let r = ((unsafe { *self.ptr } & MASKS[left_bit]) << (count - left_bit)
                | (unsafe { *self.ptr.offset(1) } >> right_shift) & MASKS[count - left_bit])
                as u16;

            self.ptr = unsafe { self.ptr.offset(1) };

            r
        } else {
            self.take_bits(8) << (count - 8) | self.take_bits(count - 8)
        }
    }
}

fn take_bits(buffer: &[u8], offset: usize, count: usize) -> u16 {
    let ptr = buffer.as_ptr();
    let off = unsafe { ptr.offset(offset as isize / 8) };
    let o = count + (offset % 8);
    if o <= 8 {
        ((unsafe { *off } >> (8 - o)) & MASKS[count]) as u16
    } else if count <= 8 {
        let left_bit = 8 - offset % 8;
        let right_shift = 8 - (count - left_bit);

        ((unsafe { *off } & MASKS[left_bit]) << (count - left_bit)
            | (unsafe { *off.offset(1) } >> right_shift) & MASKS[count - left_bit]) as u16
    } else {
        take_bits(buffer, offset, 8) << (count - 8) | take_bits(buffer, offset + 8, count - 8)
    }
}

fn decode_byte(chars: &[u8]) -> u8 {
    let s = String::from_utf8(chars.to_vec()).unwrap();
    u8::from_str_radix(&s, 16).unwrap()
}
