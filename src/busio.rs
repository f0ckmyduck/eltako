use crate::ringbuff::RingBuff;
use log::{debug, error, info};
use std::string::String;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use crate::eldecode::EltakoFrame;

pub struct SerialShared {
    refresh_rate: u64,
    port: Box<dyn serialport::SerialPort>,

    pub buff: RingBuff<EltakoFrame>,
}

pub struct SerialInterface {
    listener: Option<std::thread::JoinHandle<()>>,

    exit: Arc<AtomicBool>,
    pub shared: Arc<Mutex<SerialShared>>,
}

impl SerialInterface {
    /// Constructs a new SerialInterface struct with a default ring buffer size of 1000 frames.
    pub fn new(path: String, baudrate: u32, refresh_rate: u64) -> Self {
        SerialInterface {
            shared: Arc::new(Mutex::new(SerialShared {
                refresh_rate,
                port: serialport::new(path, baudrate)
                    .open()
                    .expect("Failed to open port!"),

                buff: RingBuff::new(
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
            listener: None,
        }
    }

    /// Starts the listener thread.
    /// The listener thread reads serial data into a temporary buffer. This data is furthermore
    /// decoded using the EltakoFrame struct and saved in a ring buffer which is public.
    pub fn start(&mut self) -> Result<(), ()> {
        use std::thread::{sleep, spawn};
        use std::time::Duration;

        if self.listener.is_some() {
            return Err(());
        }

        let exit = self.exit.clone();
        let shared_lock = self.shared.clone();

        // Setup the listener thread
        self.listener = Some(spawn(move || {
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

                    shared.port.read(&mut read_buff)
                };

                if ret.is_ok() {
                    // Write the used portion of the buffer out to the temporary ring buffer.
                    for i in 0..ret.unwrap() {
                        debug!("Received: {:#2x}", read_buff[i]);
                        temp.append(read_buff[i]);
                    }
                } else {
                    let refresh_rate = shared_lock.lock().unwrap().refresh_rate;

                    sleep(Duration::from_millis(refresh_rate));
                }

                match temp.reduce_search(0xa5) {
                    Ok(()) => {
                        if let Ok(frame_bytes) = temp.reduce_slice(14) {
                            let decoded_frame = EltakoFrame::from_vec(&frame_bytes);

                            if let Ok(frame) = decoded_frame {
                                let mut shared = shared_lock.lock().unwrap();

                                info!("{}", frame.explain());
                                shared.buff.append(frame);
                            } else {
                                error!("Decode failed on data: {:x?}", frame_bytes);
                            }
                        }
                    }
                    Err(()) => {}
                }
            }
        }));

        Ok(())
    }

    pub fn write(&mut self, frame: EltakoFrame) -> Result<(), ()> {
        let shared_lock = self.shared.clone();
        let mut shared = shared_lock.lock().unwrap();

        if shared.port.write_all(&frame.to_vec()[..]).is_err() {
            error!("Failed to write: {:x?}", frame);
            return Err(());
        }
        Ok(())
    }
}

impl Drop for SerialInterface {
    fn drop(&mut self) {
        let exit = self.exit.clone();

        exit.store(true, Ordering::Relaxed);
        self.listener
            .take()
            .unwrap()
            .join()
            .expect("Could not join listener thread!");

        debug!("Listener thread stopped!");
    }
}
