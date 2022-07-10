use crate::ringbuff::RingBuff;
use log::{debug, error, info};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        {Arc, Mutex},
    },
};

use crate::eldecode::EltakoFrame;

pub struct SerialShared {
    serial_port: std::fs::File,

    pub frame_buffer: RingBuff<EltakoFrame>,
}

pub struct SerialInterface {
    listener_handle: Option<std::thread::JoinHandle<()>>,

    exit: Arc<AtomicBool>,
    pub shared: Arc<Mutex<SerialShared>>,
}

impl SerialInterface {
    /// Constructs a new SerialInterface struct with a default ring buffer size of 1000 frames.
    /// It uses the nix crate to modify the baudrate and change the attributes of the tty to ignore
    /// control characters.
    pub fn new(path: &Path, baudrate: nix::sys::termios::BaudRate) -> Result<Self, ()> {
        use nix::fcntl::open;
        use nix::fcntl::OFlag;
        use nix::sys::stat::Mode;
        use nix::sys::termios::{cfmakeraw, cfsetspeed, tcgetattr, tcsetattr};
        use std::fs::File;
        use std::os::unix::prelude::FromRawFd;

        let fd = match open(path, OFlag::O_RDWR, Mode::empty()) {
            Ok(x) => x,
            Err(_) => {
                return Err(());
            }
        };

        let mut thdata = match tcgetattr(fd) {
            Ok(x) => x,
            Err(_) => {
                return Err(());
            }
        };

        cfmakeraw(&mut thdata);

        if cfsetspeed(&mut thdata, baudrate).is_err() {
            return Err(());
        }

        if tcsetattr(fd, nix::sys::termios::SetArg::TCSANOW, &thdata).is_err() {
            return Err(());
        }

        Ok(SerialInterface {
            shared: Arc::new(Mutex::new(SerialShared {
                serial_port: unsafe { File::from_raw_fd(fd) },
                frame_buffer: RingBuff::new(
                    1000,
                    EltakoFrame {
                        length: 0,
                        rorg: 0,
                        data: 0,
                        source: 0,
                        status: 0,
                    },
                ),
            })),

            exit: Arc::new(AtomicBool::new(false)),
            listener_handle: None,
        })
    }

    /// Starts the listener thread.
    /// The listener thread reads serial data into a temporary buffer. This data is furthermore
    /// decoded using the EltakoFrame struct and saved in a ring buffer which is public.
    pub fn start(&mut self) -> Result<(), ()> {
        use std::io::Read;
        use std::thread;
        use std::time::Duration;

        if self.listener_handle.is_some() {
            return Err(());
        }

        let exit = self.exit.clone();
        let shared_lock = self.shared.clone();

        // Setup the listener thread
        match thread::Builder::new()
            .name("Listener".to_string())
            .spawn(move || {
                debug!("Listener started!");

                let mut temp = RingBuff::new(100, 0);

                loop {
                    // Break if the struct is dropped.
                    if exit.load(Ordering::Relaxed) {
                        break;
                    }

                    let mut read_buff: [u8; 128] = [0; 128];

                    // Read data into a temp array
                    let ret = {
                        let mut shared = shared_lock.lock().unwrap();

                        shared.serial_port.read(&mut read_buff)
                    };

                    if ret.is_ok() {
                        // Write the used portion of the buffer out to the temporary ring buffer.
                        for i in 0..ret.unwrap() {
                            temp.append(read_buff[i]);
                        }
                    }

                    match temp.reduce_search(0xa5) {
                        Ok(()) => {
                            if let Ok(frame_bytes) = temp.reduce_slice(14) {
                                let decoded_frame = EltakoFrame::from_vec(&frame_bytes);

                                if let Ok(frame) = decoded_frame {
                                    let mut shared = shared_lock.lock().unwrap();

                                    info!("{}", frame.explain());
                                    shared.frame_buffer.append(frame);
                                } else {
                                    error!("Decode failed on data: {:x?}", frame_bytes);
                                }
                            }
                        }
                        Err(()) => {}
                    }
                    thread::sleep(Duration::from_micros(100));
                }
            }) {
            Ok(x) => self.listener_handle = Some(x),
            Err(x) => {
                error!("{}", x);
            }
        }

        Ok(())
    }

    pub fn write(&mut self, frame: EltakoFrame) -> Result<(), ()> {
        use std::io::Write;

        let shared_lock = self.shared.clone();
        let mut shared = shared_lock.lock().unwrap();

        match shared.serial_port.write(&frame.to_vec()[..]) {
            Ok(_) => {}
            Err(x) => {
                error!("Failed to write: {:x?} -> {}", frame, x);
                return Err(());
            }
        }
        Ok(())
    }
}

impl Drop for SerialInterface {
    fn drop(&mut self) {
        let exit = self.exit.clone();

        exit.store(true, Ordering::Relaxed);
        self.listener_handle
            .take()
            .unwrap()
            .join()
            .expect("Could not join listener thread!");

        debug!("Listener thread stopped!");
    }
}
