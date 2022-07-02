use log::error;

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

        if frame[0] != 0xa5 || frame[1] != 0x5a {
            error!("Message has an invalid preamble!");
            return Err(());
        }

        let mut crc: u8 = 0;

        for i in 2..(frame.len() - 1) {
            (crc, _) = crc.overflowing_add(frame[i]);
        }

        if frame[13] != crc {
            error!("Message crc check failed!");
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
        let msg_rorg;
        let mut msg_data = "None";
        let mut msg_status = "None";

        // Message type apparently?
        match self.rorg {
            0x5 => {
                msg_rorg = "Button";

                // Frame payload
                msg_data = match self.data.to_be_bytes()[0] {
                    0x70 => "Top Right",
                    0x30 => "Top Left",
                    0x10 => "Bot Left",
                    0x50 => "Bot Right",
                    _ => "None",
                };

                msg_status = match self.status {
                    0x30 => "On",
                    0x20 => "Off",
                    _ => "None",
                };
            }
            0x7 => {
                msg_rorg = "Dimmer";

                // Frame payload
                msg_data = match self.data.to_be_bytes()[0] {
                    0x02 => "Dimming",
                    _ => "None",
                };
            }
            0xf0 => {
                msg_rorg = "Scan";
            }
            0xfe => {
                msg_rorg = "Status";
            }
            _ => msg_rorg = "None",
        }

        format!(
            "rorg:{:<6} -> 0x{:01x} | data:{:<10} -> 0x{:08x} | source_addr:0x{:08x} | status:{:<3} ->0x{:01x}",
            msg_rorg, self.rorg, msg_data, self.data, self.source, msg_status, self.status
        )
    }
}
