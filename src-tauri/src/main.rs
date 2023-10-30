#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serial::{unix::TTYPort, SerialPort};
use serial::{BaudRate, CharSize, FlowControl, Parity, PortSettings, StopBits};
use std::{
    error::Error,
    io::Read,
    thread::{self, JoinHandle},
    time::Duration,
};
use tauri::{App, AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub temperature_1: f32,
}

const SETTINGS: PortSettings = PortSettings {
    baud_rate: BaudRate::BaudOther(921600),
    char_size: CharSize::Bits8,
    parity: Parity::ParityNone,
    stop_bits: StopBits::Stop1,
    flow_control: FlowControl::FlowNone,
};

pub struct StateStorage {
    port: TTYPort,
}

impl StateStorage {
    /// Default baud rate: 115200
    pub fn new(path: &str, config: PortSettings) -> Result<Self, serial::Error> {
        let mut port = serial::open(path)?;
        port.configure(&config)?;
        port.set_timeout(Duration::from_secs(5))?;

        Ok(Self { port })
    }

    pub fn launch_background_task(mut self, handle: AppHandle) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut buf: [u8; 1024] = [0; 1024];

            loop {
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

                log::info!("Payload received: {:?}", payload);

                if let Err(e) = handle.emit_all("data_push", payload) {
                    log::error!("Event emit error: {e}");
                }

                buf = [0; 1024];
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

fn main() {
    let state_manager =
        StateStorage::new("/dev/ttyACM0", SETTINGS).expect("Failed to connect to serial console");

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|app| state_manager.tauri_app_setup(app))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
