#[derive(Debug)]
pub struct EltakoFrame {
    pub pre: u16,
    pub length: u8,
    pub rorg: u8,
    pub data: u32,
    pub source: u32,
    pub status: u8,
    pub crc: u8,
}

impl EltakoFrame {
    fn collect_to_u32(data: &[u8]) -> u32 {
        (data[0] as u32) << 24 | (data[1] as u32) << 16 | (data[2] as u32) << 8 | (data[3] as u32)
    }

    fn collect_to_u16(data: &[u8]) -> u16 {
        (data[0] as u16) << 8 | (data[1] as u16)
    }

    pub fn from_vec(frame: &[u8]) -> Result<Self, ()> {
        if frame.len() < 14 {
            return Err(());
        }

        Ok(EltakoFrame {
            pre: EltakoFrame::collect_to_u16(&frame[0..2]),
            length: frame[2],
            rorg: frame[3],
            data: EltakoFrame::collect_to_u32(&frame[4..8]),
            source: EltakoFrame::collect_to_u32(&frame[8..12]),
            status: frame[12],
            crc: frame[13],
        })
    }

    pub fn explain(&self) -> std::string::String {
        let pre_type = match self.pre {
            0xa55a => "OK",
            _ => "Err",
        };

        // Message type apparently?
        let msg_type = match self.rorg {
            0x5 => "Button",
            0x7 => "Dimmer",
            _ => "None",
        };

        // Frame payload
        let msg_data = match self.data.to_be_bytes()[0] {
            0x02 => "Dimming",
            _ => "None",
        };

        format!(
            "{:<3} > rorg:{:<6} | data:{:<10} -> 0x{:08x} | source_addr:0x{:08x} | status:0x{:01x}",
            pre_type, msg_type, msg_data, self.data, self.source, self.status
        )
    }
}
