use crate::ringbuff::RingBuff;
use std::string::String;
use std::sync::{Arc, Mutex};

pub struct SerialShared {
    path: String,
    baudrate: u32,
    refresh_rate: u64,

    pub buff: RingBuff<u8>,
}

pub struct SerialInterface {
    listener: Option<std::thread::JoinHandle<()>>,

    pub shared: Arc<Mutex<SerialShared>>,
}

impl SerialInterface {
    /// Constructs a new SerialInterface struct with a default ring buffer size of 1000 bytes.
    pub fn new(path: String, baudrate: u32, refresh_rate: u64) -> Self {
        SerialInterface {
            listener: None,

            shared: Arc::new(Mutex::new(SerialShared {
                path,
                baudrate,
                refresh_rate,

                buff: RingBuff::new(1000),
            })),
        }
    }

    /// Starts the listener thread.
    /// The listener thread reads serial data into the ring buffer at
    /// a specific size per iteration.
    pub fn start(&mut self) {
        use std::thread::{sleep, spawn};
        use std::time::Duration;

        let shared_lock = self.shared.clone();

        self.listener = Some(spawn(move || {
            // Get the thread configuration variables out of the shared struct
            let (mut port, refresh_rate) = {
                let mut shared = shared_lock.lock().unwrap();

                // Initialize the buffer
                for _ in 0..1000 {
                    shared.buff.data.push(0);
                }
                (
                    serialport::new(shared.path.clone(), shared.baudrate)
                        .open()
                        .expect("Failed to open port!"),
                    shared.refresh_rate,
                )
            };

            loop {
                let mut temp_read_buff: [u8; 32] = [0; 32];
                let ret = port.read(&mut temp_read_buff);

                if ret.is_ok() {
                    // Write the used portion of the buffer out to the ring buffer
                    {
                        let mut shared = shared_lock.lock().unwrap();

                        for i in 0..ret.unwrap() {
                            shared.buff.append(temp_read_buff[i]).unwrap();
                            // println!(" !{:#2x}! ", temp_read_buff[i]);
                        }
                    }
                }
                sleep(Duration::from_millis(refresh_rate));
            }
        }));
    }
}
