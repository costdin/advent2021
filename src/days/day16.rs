use super::get_input_lines;

pub fn day16() {
    let line = get_input_lines(16).into_iter().nth(0).unwrap();
    let buffer = line
        .as_bytes()
        .chunks(2)
        .map(decode_byte)
        .collect::<Vec<_>>();

    let mut bit_array = BitArray::new(buffer);

    let packet = decode_packet(&mut bit_array);

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

fn decode_packet(bit_array: &mut BitArray) -> Packet {
    let version = bit_array.take_bits(3);
    let type_id = bit_array.take_bits(3);

    if type_id == 4 {
        let value = decode_literal(bit_array);
        let packet = Packet {
            version,
            payload: Payload::Literal(value),
        };

        packet
    } else {
        let lenght_type = bit_array.take_bits(1);

        let sub_packets = if lenght_type == 0 {
            decode_sized_operator(bit_array)
        } else {
            decode_count_operator(bit_array)
        };

        let packet = Packet {
            version,
            payload: Payload::Operator(type_id.into(), sub_packets),
        };

        packet
    }
}

fn decode_sized_operator(bit_array: &mut BitArray) -> Vec<Packet> {
    let length = bit_array.take_bits(15);
    let end = bit_array.offset + length as usize;

    let mut packets = vec![];

    while bit_array.offset < end {
        let packet = decode_packet(bit_array);
        packets.push(packet);
    }

    packets
}

fn decode_count_operator(bit_array: &mut BitArray) -> Vec<Packet> {
    let count = bit_array.take_bits(11) as usize;

    (0..count).map(|_| decode_packet(bit_array)).collect()
}

fn decode_literal(bit_array: &mut BitArray) -> u128 {
    let mut value = 0u128;

    loop {
        let block = bit_array.take_bits(5);
        value = (value << 4) + (block & 0x0F) as u128;

        if block & 0x10 == 0 {
            break;
        }
    }

    value
}

static MASKS: &'static [u8] = &[0, 1, 3, 7, 15, 31, 63, 127, 255];

struct BitArray {
    _buffer: Vec<u8>,
    offset: usize,
    ptr: *const u8,
}

impl BitArray {
    fn new(buffer: Vec<u8>) -> BitArray {
        let ptr = buffer.as_ptr();

        BitArray {
            _buffer: buffer,
            offset: 0,
            ptr,
        }
    }

    // I'm using pointers here, just for fun. Or maybe to avoid bounds checking?
    fn take_bits(&mut self, count: usize) -> u16 {
        if count > 8 {
            return self.take_bits(8) << (count - 8) | self.take_bits(count - 8);
        }

        // offset from the first bit of the current byte
        let end_offset = count + (self.offset % 8);

        /*
            A and B are 2 bytes, 'x' are bits to be returned:
                        |aaaa aaaa bbbb bbbb|
            first case: |xxxx xxaa bbbb bbbb| // the next bit will still be from A
            first case: |axxx xxxa bbbb bbbb| // the next bit will still be from A
            second case:|aaxx xxxx bbbb bbbb| // the next bit will be from B
            second case:|xxxx xxxx bbbb bbbb| // the next bit will be from B
            third case: |aaax xxxx xxbb bbbb| // the bits to be returned come from both A and B
        */

        let result = if end_offset < 8 {
            // first case: all the bits are in the current byte, as well as the next bit
            ((unsafe { *self.ptr } >> (8 - end_offset)) & MASKS[count]) as u16
        } else if end_offset == 8 {
            // second case: all the bits are in the current byte, but the next bit is in the next byte
            let r = ((unsafe { *self.ptr } >> (8 - end_offset)) & MASKS[count]) as u16;
            self.ptr = unsafe { self.ptr.offset(1) };

            r
        } else {
            // third case: bits are in the current and in the next byte
            let left_bit = 8 - (self.offset % 8);
            let right_shift = 16 - end_offset;

            let r = (unsafe { *self.ptr } & MASKS[left_bit]) << (count - left_bit);
            self.ptr = unsafe { self.ptr.offset(1) };

            (r | (unsafe { *self.ptr } >> right_shift) & MASKS[count - left_bit]) as u16
        };

        self.offset += count;
        result
    }
}

fn decode_byte(chars: &[u8]) -> u8 {
    let s = String::from_utf8(chars.to_vec()).unwrap();
    u8::from_str_radix(&s, 16).unwrap()
}
