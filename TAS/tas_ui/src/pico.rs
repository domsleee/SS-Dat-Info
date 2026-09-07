use eframe::egui;

use crate::ui_log::UiLog;

fn is_pico_data_port(info: &serialport::SerialPortInfo) -> bool {
    // boot.py enables console (interface 0) and binary data (interface 2).
    matches!(&info.port_type, serialport::SerialPortType::UsbPort(usb)
        if usb.vid == 0x2e8a && usb.pid == 0x000b && usb.interface == Some(2))
}

fn select_data_port<'a>(
    ports: &'a [serialport::SerialPortInfo],
    preferred: &str,
) -> Result<&'a str, String> {
    if let Some(port) = ports
        .iter()
        .find(|port| port.port_name.eq_ignore_ascii_case(preferred))
    {
        return if is_pico_data_port(port) {
            Ok(&port.port_name)
        } else {
            Err(format!(
                "{} is not the expected Pico data interface",
                preferred
            ))
        };
    }
    Err(format!(
        "Pico data port {} not found; select its data COM port",
        preferred
    ))
}

#[cfg(test)]
mod detection_tests {
    use super::*;

    fn usb(name: &str, vid: u16, interface: Option<u8>) -> serialport::SerialPortInfo {
        serialport::SerialPortInfo {
            port_name: name.into(),
            port_type: serialport::SerialPortType::UsbPort(serialport::UsbPortInfo {
                vid,
                pid: 0x000b,
                interface,
                serial_number: None,
                manufacturer: None,
                product: None,
            }),
        }
    }

    #[test]
    fn selects_only_configured_pico_data_interface_without_probing() {
        let ports = vec![
            usb("COM1", 0x1234, Some(2)),
            usb("COM8", 0x2e8a, Some(0)),
            usb("COM7", 0x2e8a, Some(2)),
        ];
        assert_eq!(select_data_port(&ports, "com7").unwrap(), "COM7");
        assert!(select_data_port(&ports, "COM1").is_err());
        assert!(select_data_port(&ports, "COM8").is_err());
        assert!(select_data_port(&ports, "COM9").is_err());
        assert!(select_data_port(&[usb("COM7", 0x2e8a, None)], "COM7").is_err());
    }
}

pub struct PicoState {
    pub port_name: String,
    pub connected: bool,
    pub port: Option<Box<dyn serialport::SerialPort>>,
    pub error: Option<String>,
    scan_attempted: bool,
}

impl PicoState {
    pub fn new() -> Self {
        Self {
            port_name: std::env::var("TAS_PICO_PORT").unwrap_or_else(|_| "COM7".into()),
            connected: false,
            port: None,
            error: None,
            scan_attempted: false,
        }
    }

    /// Identify the configured data interface without sending probe bytes.
    /// Called once on startup. Returns log messages.
    pub fn auto_detect(&mut self) -> Vec<String> {
        if self.scan_attempted || self.connected {
            return Vec::new();
        }
        self.scan_attempted = true;

        let ports = match serialport::available_ports() {
            Ok(p) => p,
            Err(_) => return Vec::new(),
        };

        if let Err(error) = select_data_port(&ports, &self.port_name) {
            return vec![format!("Pico auto-detect: {error}")];
        }
        self.connect();
        if self.connected {
            vec![format!(
                "Pico data interface connected on {}",
                self.port_name
            )]
        } else {
            vec![format!(
                "Pico connection failed: {}",
                self.error.as_deref().unwrap_or("unknown error")
            )]
        }
    }

    pub fn connect(&mut self) {
        self.disconnect();
        let verified = serialport::available_ports()
            .map_err(|error| error.to_string())
            .and_then(|ports| select_data_port(&ports, &self.port_name).map(|_| ()));
        if let Err(error) = verified {
            self.error = Some(error);
            return;
        }
        match serialport::new(&self.port_name, 115200)
            .timeout(std::time::Duration::from_millis(100))
            .open()
        {
            Ok(port) => {
                self.port = Some(port);
                self.connected = true;
                self.error = None;
            }
            Err(e) => {
                self.error = Some(format!("{}", e));
                self.connected = false;
            }
        }
    }

    pub fn disconnect(&mut self) {
        self.port = None;
        self.connected = false;
    }

    pub fn send_mask(&mut self, mask: u8) -> Result<(), String> {
        if let Some(ref mut port) = self.port {
            port.write_all(&[mask]).map_err(|e| {
                self.connected = false;
                self.port = None;
                format!("{}", e)
            })
        } else {
            Err("Not connected".into())
        }
    }

    pub fn send_f5(&mut self) -> Result<(), String> {
        self.send_mask(0x40)?; // bit 6 = F5
        std::thread::sleep(std::time::Duration::from_millis(50));
        self.send_mask(0xFF)?; // release all
        Ok(())
    }

    pub fn soft_reconnect(&mut self) -> Result<(), String> {
        self.send_mask(0xFD)
    }

    /// Quick health check: try writing 0xFF (release all). Returns true if alive.
    pub fn health_check(&mut self) -> bool {
        if !self.connected {
            return false;
        }
        match self.send_mask(0xFF) {
            Ok(()) => true,
            Err(_) => {
                self.connected = false;
                self.port = None;
                false
            }
        }
    }
}

pub fn show_panel(ui: &mut egui::Ui, pico: &mut PicoState, log: &mut UiLog) {
    ui.heading("Pico HID");

    ui.horizontal(|ui| {
        ui.label("Port:");
        ui.text_edit_singleline(&mut pico.port_name);
    });

    ui.horizontal(|ui| {
        if pico.connected {
            ui.colored_label(egui::Color32::from_rgb(80, 200, 80), "Connected");
            if ui.button("Disconnect").clicked() {
                pico.disconnect();
                log.push("Pico disconnected");
            }
        } else {
            ui.colored_label(egui::Color32::GRAY, "Disconnected");
            if ui.button("Connect").clicked() {
                pico.connect();
                if pico.connected {
                    log.push(format!("Pico connected on {}", pico.port_name));
                }
            }
        }
    });

    if let Some(ref err) = pico.error {
        ui.colored_label(egui::Color32::from_rgb(255, 100, 100), err);
    }

    if pico.connected {
        ui.horizontal(|ui| {
            if ui.button("F5 Restart").clicked() {
                match pico.send_f5() {
                    Ok(()) => log.push("Pico: sent F5"),
                    Err(e) => log.push(format!("Pico F5 error: {}", e)),
                }
            }
            if ui.button("Soft Reconnect").clicked() {
                match pico.soft_reconnect() {
                    Ok(()) => log.push("Pico: soft reconnect sent"),
                    Err(e) => log.push(format!("Pico reconnect error: {}", e)),
                }
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Health Check").clicked() {
                if pico.health_check() {
                    log.push("Pico: health OK");
                } else {
                    log.push("Pico: health FAIL (disconnected)");
                }
            }
        });
    }

    if ui.button("Scan Ports").clicked() {
        match serialport::available_ports() {
            Ok(ports) => {
                let names: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();
                log.push(format!("Ports: {}", names.join(", ")));
            }
            Err(e) => log.push(format!("Port scan error: {}", e)),
        }
    }

    ui.separator();
    ui.label(egui::RichText::new("Shortcuts").strong().small());
    ui.label(egui::RichText::new("F5 Restart game  F9 REC  F10 PLAY").small());
    ui.label(egui::RichText::new("F11 STOP  F12 CONT").small());
    ui.label(egui::RichText::new("Space Stop").small());
    ui.label(egui::RichText::new("Ctrl+Z Undo  Ctrl+S Save  Ctrl+O Open").small());
    ui.label(egui::RichText::new("+/- Timeline zoom").small());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disconnect_clears_connected() {
        let mut pico = PicoState::new();
        pico.connected = true;
        pico.disconnect();
        assert!(!pico.connected);
        assert!(pico.port.is_none());
    }

    #[test]
    fn send_mask_without_port_errors() {
        let mut pico = PicoState::new();
        let result = pico.send_mask(0xFF);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Not connected");
    }

    #[test]
    fn health_check_disconnected_returns_false() {
        let mut pico = PicoState::new();
        assert!(!pico.health_check());
    }

    #[test]
    fn auto_detect_skips_when_already_scanned() {
        let mut pico = PicoState::new();
        pico.scan_attempted = true;
        let logs = pico.auto_detect();
        assert!(logs.is_empty());
    }

    #[test]
    fn auto_detect_skips_when_connected() {
        let mut pico = PicoState::new();
        pico.connected = true;
        let logs = pico.auto_detect();
        assert!(logs.is_empty());
    }

    #[test]
    fn send_f5_without_port_errors() {
        let mut pico = PicoState::new();
        assert!(pico.send_f5().is_err());
    }

    #[test]
    fn soft_reconnect_without_port_errors() {
        let mut pico = PicoState::new();
        assert!(pico.soft_reconnect().is_err());
    }
}
