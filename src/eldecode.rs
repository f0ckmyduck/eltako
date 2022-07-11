use log::error;
use std::{string::String, vec::Vec};

pub mod premaid {
    use crate::eldecode::EltakoFrame;

    pub enum Positions {
        TopRight = 0x70,
        TopLeft = 0x30,
        BotLeft = 0x10,
        BotRight = 0x50,
        Nothing = 0x00,
    }

    pub const fn scan_start() -> EltakoFrame {
        EltakoFrame {
            length: 0xd,
            rorg: 0xf0,
            data: 0x01028708,
            source: 0x04065200,
            status: 0x00,
        }
    }

    pub const fn scan_members(index: u8) -> EltakoFrame {
        EltakoFrame {
            length: 0xd,
            rorg: 0xf0,
            data: 0x00000000,
            source: 0x00000000,
            status: index,
        }
    }

    pub const fn status(index: u8) -> EltakoFrame {
        EltakoFrame {
            length: 0xd,
            rorg: 0xfe,
            data: 0x00000000,
            source: 0x00000000,
            status: index,
        }
    }

    pub const fn acknowledge(index: u8) -> EltakoFrame {
        EltakoFrame {
            length: 0xd,
            rorg: 0xfc,
            data: 0x00000000,
            source: 0x00000000,
            status: index,
        }
    }

    pub const fn button(source_address: u32, status: bool, position: Positions) -> EltakoFrame {
        EltakoFrame {
            length: 0xd,
            rorg: 0x05,
            data: 0x00000000 | (position as u32) << 24,
            source: source_address,
            status: if status { 0x30 } else { 0x20 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EltakoFrame {
    pub length: u8,
    pub rorg: u8,
    pub data: u32,
    pub source: u32,
    pub status: u8,
}

impl EltakoFrame {
    fn collect_to_u32(data: &[u8]) -> u32 {
        (data[0] as u32) << 24 | (data[1] as u32) << 16 | (data[2] as u32) << 8 | (data[3] as u32)
    }

    fn collect_to_u16(data: &[u8]) -> u16 {
        (data[0] as u16) << 8 | (data[1] as u16)
    }

    pub fn crc_from_vec(frame: &[u8]) -> u8 {
        let mut crc: u8 = 0;

        for i in 2..(frame.len() - 1) {
            (crc, _) = crc.overflowing_add(frame[i]);
        }
        return crc;
    }

    pub fn crc_from_frame(self) -> u8 {
        let mut crc: u8 = 0;

        (crc, _) = crc.overflowing_add(self.length);
        (crc, _) = crc.overflowing_add(self.rorg);

        for i in 0..4 {
            (crc, _) = crc.overflowing_add(self.data.to_be_bytes()[i]);
        }

        for i in 0..4 {
            (crc, _) = crc.overflowing_add(self.source.to_be_bytes()[i]);
        }

        (crc, _) = crc.overflowing_add(self.status);

        return crc;
    }

    pub fn from_vec(frame: &[u8]) -> Result<Self, ()> {
        if frame.len() == 0xd {
            return Err(());
        }

        if frame[0] != 0xa5 || frame[1] != 0x5a {
            error!("Message has an invalid preamble!");
            return Err(());
        }

        if frame[13] != EltakoFrame::crc_from_vec(frame) {
            error!("Message crc check failed!");
            return Err(());
        }

        Ok(EltakoFrame {
            length: frame[2],
            rorg: frame[3],
            data: EltakoFrame::collect_to_u32(&frame[4..8]),
            source: EltakoFrame::collect_to_u32(&frame[8..12]),
            status: frame[12],
        })
    }

    pub fn to_vec(self) -> Vec<u8> {
        let mut frame = Vec::new();

        frame.push(0xa5);
        frame.push(0x5a);
        frame.push(self.length);
        frame.push(self.rorg);

        for i in 0..4 {
            frame.push(self.data.to_be_bytes()[i]);
        }

        for i in 0..4 {
            frame.push(self.source.to_be_bytes()[i]);
        }

        frame.push(self.status);
        frame.push(EltakoFrame::crc_from_frame(self));

        return frame;
    }

    pub fn explain(&self) -> String {
        let msg_rorg;
        let mut msg_data = String::new();
        let mut msg_status = "";

        // Message type apparently?
        match self.rorg {
            0x05 => {
                // Message type
                msg_rorg = "Button";

                // Button location
                msg_data = match self.data.to_be_bytes()[0] {
                    0x70 => "Top Right",
                    0x30 => "Top Left",
                    0x10 => "Bot Left",
                    0x50 => "Bot Right",
                    _ => "",
                }
                .to_string();

                // Button state
                msg_status = match self.status {
                    0x30 => "On",
                    0x20 => "Off",
                    _ => "",
                };
            }
            0x07 => {
                // Message type
                msg_rorg = "Dimmer";

                let data = self.data.to_be_bytes();

                msg_data += &format!("On={} ", data[0] & 0x1);
                msg_data += &format!("Lock={} ", data[0] & 0x4);
                msg_data += &format!("Speed={} ", data[1]);
                msg_data += &format!("Val={} ", data[2]);

                msg_data += match data[3] {
                    0x02 => "Dimming",
                    _ => "",
                };
            }
            0xf0 => {
                msg_rorg = "Scan";
            }
            0xfe => {
                msg_rorg = "Status";
            }
            0xfc => {
                msg_rorg = "Acknowledge";
            }
            _ => msg_rorg = "",
        }

        format!(
            "rorg:{:<12} -> 0x{:02x} | source_addr:0x{:08x} | status:{:<3} -> 0x{:02x} | data:{} -> 0x{:08x}",
            msg_rorg, self.rorg,  self.source, msg_status, self.status, msg_data, self.data,
        )
    }
}
