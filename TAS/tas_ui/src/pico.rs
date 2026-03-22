use eframe::egui;

pub struct PicoState {
    pub port_name: String,
    pub connected: bool,
    pub port: Option<Box<dyn serialport::SerialPort>>,
    pub error: Option<String>,
    pub auto_detected: bool,
    scan_attempted: bool,
}

impl PicoState {
    pub fn new() -> Self {
        Self {
            port_name: "COM7".to_string(),
            connected: false,
            port: None,
            error: None,
            auto_detected: false,
            scan_attempted: false,
        }
    }

    /// Scan COM ports and auto-connect to the first responsive one.
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

        let mut logs = Vec::new();
        let names: Vec<&str> = ports.iter().map(|p| p.port_name.as_str()).collect();
        if names.is_empty() {
            return Vec::new();
        }
        logs.push(format!("Pico auto-detect: scanning {}", names.join(", ")));

        for info in &ports {
            // Try to open and do a quick write test (send 0xFF = release all, harmless)
            let result = serialport::new(&info.port_name, 115200)
                .timeout(std::time::Duration::from_millis(100))
                .open();

            if let Ok(mut port) = result {
                if port.write_all(&[0xFF]).is_ok() {
                    self.port_name = info.port_name.clone();
                    self.port = Some(port);
                    self.connected = true;
                    self.auto_detected = true;
                    self.error = None;
                    logs.push(format!("Pico auto-detected on {}", self.port_name));
                    return logs;
                }
            }
        }
        logs.push("Pico auto-detect: no responsive port found".to_string());
        logs
    }

    pub fn connect(&mut self) {
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

pub fn show_panel(ui: &mut egui::Ui, pico: &mut PicoState, log: &mut Vec<String>) {
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
                log.push(format!(
                    "[{}] Pico disconnected",
                    chrono::Local::now().format("%H:%M:%S")
                ));
            }
        } else {
            ui.colored_label(egui::Color32::GRAY, "Disconnected");
            if ui.button("Connect").clicked() {
                pico.connect();
                if pico.connected {
                    log.push(format!(
                        "[{}] Pico connected on {}",
                        chrono::Local::now().format("%H:%M:%S"),
                        pico.port_name
                    ));
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
                    Ok(()) => log.push(format!(
                        "[{}] Pico: sent F5",
                        chrono::Local::now().format("%H:%M:%S")
                    )),
                    Err(e) => log.push(format!(
                        "[{}] Pico F5 error: {}",
                        chrono::Local::now().format("%H:%M:%S"),
                        e
                    )),
                }
            }
            if ui.button("Soft Reconnect").clicked() {
                match pico.soft_reconnect() {
                    Ok(()) => log.push(format!(
                        "[{}] Pico: soft reconnect sent",
                        chrono::Local::now().format("%H:%M:%S")
                    )),
                    Err(e) => log.push(format!(
                        "[{}] Pico reconnect error: {}",
                        chrono::Local::now().format("%H:%M:%S"),
                        e
                    )),
                }
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Health Check").clicked() {
                let ok = pico.health_check();
                if ok {
                    log.push(format!(
                        "[{}] Pico: health OK",
                        chrono::Local::now().format("%H:%M:%S")
                    ));
                } else {
                    log.push(format!(
                        "[{}] Pico: health FAIL (disconnected)",
                        chrono::Local::now().format("%H:%M:%S")
                    ));
                }
            }
        });

        // Available COM ports
        if ui.button("Scan Ports").clicked() {
            match serialport::available_ports() {
                Ok(ports) => {
                    let names: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();
                    log.push(format!(
                        "[{}] Ports: {}",
                        chrono::Local::now().format("%H:%M:%S"),
                        names.join(", ")
                    ));
                }
                Err(e) => {
                    log.push(format!(
                        "[{}] Port scan error: {}",
                        chrono::Local::now().format("%H:%M:%S"),
                        e
                    ));
                }
            }
        }
    } else {
        // Show scan button even when disconnected
        if ui.button("Scan Ports").clicked() {
            match serialport::available_ports() {
                Ok(ports) => {
                    let names: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();
                    log.push(format!(
                        "[{}] Ports: {}",
                        chrono::Local::now().format("%H:%M:%S"),
                        names.join(", ")
                    ));
                }
                Err(e) => {
                    log.push(format!(
                        "[{}] Port scan error: {}",
                        chrono::Local::now().format("%H:%M:%S"),
                        e
                    ));
                }
            }
        }
    }

    // Keyboard shortcut hints
    ui.separator();
    ui.label(egui::RichText::new("Shortcuts").strong().small());
    ui.label(egui::RichText::new("F5 Restart game  F9 REC  F10 PLAY").small());
    ui.label(egui::RichText::new("F11 STOP  F12 CONT").small());
    ui.label(egui::RichText::new("Space Stop  . Step  , Step back").small());
    ui.label(egui::RichText::new("Ctrl+Z Undo  Ctrl+S Save  Ctrl+O Open").small());
    ui.label(egui::RichText::new("+/- Timeline zoom").small());
}
