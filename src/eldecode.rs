use std::vec::Vec;

struct EltakoFrame {
    length: u8,
    rorg: u8,
    data: u32,
    source: u32,
    status: u8,
    crc: u8,
}

impl EltakoFrame {
    fn collect_to_u32(data: &[u8]) -> u32 {
        (data[0] as u32) << 24 | (data[1] as u32) << 16 | (data[2] as u32) << 8 | (data[3] as u32)
    }
    fn from_vec(frame: [u8; 14]) -> Self {
        EltakoFrame {
            length: frame[2],
            rorg: frame[3],
            data: EltakoFrame::collect_to_u32(&frame[4..7]),
            source: EltakoFrame::collect_to_u32(&frame[8..11]),
            status: frame[12],
            crc: frame[13],
        }
    }
}
