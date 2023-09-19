use crate::messaging::Payload;
use serialport::SerialPort;
use std::{
    error::Error,
    thread::{self, JoinHandle},
    time::Duration,
};
use tauri::{App, AppHandle, Manager};

pub struct StateMachine {
    port: Box<dyn SerialPort>,
    cache: Vec<Option<Payload>>,
}

impl StateMachine {
    // Default baud rate: 115200
    pub fn new(path: &str, baud_rate: u32) -> Result<Self, serialport::Error> {
        let port = serialport::new(path, baud_rate)
            .timeout(Duration::from_millis(2000))
            .open()?;

        Ok(Self {
            port,
            cache: Vec::new(),
        })
    }

    pub fn launch_background_task(mut self, handle: AppHandle) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut buf: [u8; 1024];

            loop {
                buf = [0; 1024];

                if let Err(e) = self.port.read(&mut buf) {
                    log::error!("Serial read error: {e}");
                    continue;
                }
                let nbuf = eliminate_trailing_characters(&buf);

                let payload: Payload = match serde_json::from_slice(nbuf) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Payload deserialization error: {e}");
                        continue;
                    }
                };

                log::info!("Payload received: {:?}", payload.0);

                if let Err(e) = handle.emit_all("data_push", payload) {
                    log::error!("Event emit error: {e}");
                }
            }
        })
    }

    pub fn tauri_app_setup(self, app: &mut App) -> Result<(), Box<dyn Error>> {
        self.launch_background_task(app.handle());

        Ok(())
    }
}

fn eliminate_trailing_characters(buf: &[u8]) -> &[u8] {
    let mut pos = 0;

    for (i, ele) in buf.iter().enumerate() {
        if *ele == 13_u8 {
            pos = i;
        }
    }

    let (nbuf, _) = buf.split_at(pos);
    nbuf
}
