use serde::{Deserialize, Serialize};
use tas_shared::{TasSharedState, TAS_MAX_TICKS};

/// A named input macro: a reusable sequence of input masks.
#[derive(Clone, Serialize, Deserialize)]
pub struct InputMacro {
    pub name: String,
    pub inputs: Vec<u8>,
}

/// Persistent macro library stored as JSON.
#[derive(Default, Serialize, Deserialize)]
pub struct MacroLibrary {
    pub macros: Vec<InputMacro>,
}

impl MacroLibrary {
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, String> {
        let data = std::fs::read_to_string(path).map_err(|e| format!("{}", e))?;
        serde_json::from_str(&data).map_err(|e| format!("{}", e))
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|e| format!("{}", e))?;
        std::fs::write(path, json).map_err(|e| format!("{}", e))
    }

    /// Extract a macro from the current recording's input log (from..to range).
    pub fn extract_from_recording(
        state: &TasSharedState,
        name: &str,
        from: usize,
        to: usize,
    ) -> Option<InputMacro> {
        let count = state.recorded_count as usize;
        if from >= count || to > count || from >= to {
            return None;
        }
        Some(InputMacro {
            name: name.to_string(),
            inputs: state.input_log[from..to].to_vec(),
        })
    }

    /// Paste a macro into the recording at a given tick offset, overwriting existing inputs.
    pub fn paste_into_recording(
        state: &mut TasSharedState,
        macro_data: &InputMacro,
        at_tick: usize,
    ) -> Result<usize, String> {
        let end = at_tick + macro_data.inputs.len();
        if end > TAS_MAX_TICKS {
            return Err(format!(
                "Macro extends past max ticks ({} > {})",
                end, TAS_MAX_TICKS
            ));
        }

        state.input_log[at_tick..end].copy_from_slice(&macro_data.inputs);

        // Extend recorded_count if the paste goes beyond current length
        if end as u32 > state.recorded_count {
            state.recorded_count = end as u32;
        }

        Ok(macro_data.inputs.len())
    }
}

/// UI state for the macro panel.
pub struct MacroState {
    pub library: MacroLibrary,
    pub library_path: Option<std::path::PathBuf>,
    pub extract_name: String,
    pub extract_from: String,
    pub extract_to: String,
    pub paste_at: String,
    pub selected: Option<usize>,
}

impl MacroState {
    pub fn new() -> Self {
        Self {
            library: MacroLibrary::default(),
            library_path: None,
            extract_name: String::new(),
            extract_from: "0".to_string(),
            extract_to: String::new(),
            paste_at: "0".to_string(),
            selected: None,
        }
    }
}

/// Show the macro management panel.
pub fn show_panel(
    ui: &mut egui::Ui,
    macro_state: &mut MacroState,
    state: &mut TasSharedState,
    log: &mut Vec<String>,
) {
    ui.horizontal(|ui| {
        if ui.button("Load Library").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_title("Load Macro Library")
                .add_filter("TAS Macros", &["tasmacro"])
                .pick_file()
            {
                match MacroLibrary::load_from_file(&path) {
                    Ok(lib) => {
                        let count = lib.macros.len();
                        macro_state.library = lib;
                        macro_state.library_path = Some(path.clone());
                        push_log(log, &format!("Loaded {} macros from {}", count, path.display()));
                    }
                    Err(e) => push_log(log, &format!("Macro load error: {}", e)),
                }
            }
        }
        if ui.button("Save Library").clicked() {
            let path = macro_state.library_path.clone().or_else(|| {
                rfd::FileDialog::new()
                    .set_title("Save Macro Library")
                    .add_filter("TAS Macros", &["tasmacro"])
                    .save_file()
            });
            if let Some(path) = path {
                match macro_state.library.save_to_file(&path) {
                    Ok(()) => {
                        macro_state.library_path = Some(path.clone());
                        push_log(log, &format!("Saved macros to {}", path.display()));
                    }
                    Err(e) => push_log(log, &format!("Macro save error: {}", e)),
                }
            }
        }
    });

    ui.separator();

    // Extract from recording
    ui.label(egui::RichText::new("Extract from recording").strong());
    ui.horizontal(|ui| {
        ui.label("Name:");
        ui.text_edit_singleline(&mut macro_state.extract_name);
    });
    ui.horizontal(|ui| {
        ui.label("From tick:");
        ui.add(egui::TextEdit::singleline(&mut macro_state.extract_from).desired_width(60.0));
        ui.label("To tick:");
        ui.add(egui::TextEdit::singleline(&mut macro_state.extract_to).desired_width(60.0));
        if ui.button("Extract").clicked() {
            let from: usize = macro_state.extract_from.parse().unwrap_or(0);
            let to: usize = macro_state
                .extract_to
                .parse()
                .unwrap_or(state.recorded_count as usize);
            let name = if macro_state.extract_name.is_empty() {
                format!("macro_{}", macro_state.library.macros.len() + 1)
            } else {
                macro_state.extract_name.clone()
            };
            if let Some(m) = MacroLibrary::extract_from_recording(state, &name, from, to) {
                let len = m.inputs.len();
                macro_state.library.macros.push(m);
                push_log(log, &format!("Extracted '{}': {} ticks", name, len));
            } else {
                push_log(log, "Extract failed: invalid range");
            }
        }
    });

    ui.separator();

    // Macro list
    ui.label(egui::RichText::new("Library").strong());
    let mut to_delete: Option<usize> = None;
    for (i, m) in macro_state.library.macros.iter().enumerate() {
        let selected = macro_state.selected == Some(i);
        ui.horizontal(|ui| {
            if ui
                .selectable_label(selected, format!("{} ({} ticks)", m.name, m.inputs.len()))
                .clicked()
            {
                macro_state.selected = Some(i);
            }
            if ui.small_button("X").clicked() {
                to_delete = Some(i);
            }
        });
    }
    if let Some(i) = to_delete {
        let name = macro_state.library.macros[i].name.clone();
        macro_state.library.macros.remove(i);
        macro_state.selected = None;
        push_log(log, &format!("Deleted macro '{}'", name));
    }

    // Paste selected macro
    if let Some(idx) = macro_state.selected {
        if idx < macro_state.library.macros.len() {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Paste at tick:");
                ui.add(egui::TextEdit::singleline(&mut macro_state.paste_at).desired_width(60.0));
                if ui.button("Paste").clicked() {
                    let at: usize = macro_state.paste_at.parse().unwrap_or(0);
                    let m = macro_state.library.macros[idx].clone();
                    match MacroLibrary::paste_into_recording(state, &m, at) {
                        Ok(len) => push_log(
                            log,
                            &format!("Pasted '{}' ({} ticks) at tick {}", m.name, len, at),
                        ),
                        Err(e) => push_log(log, &format!("Paste error: {}", e)),
                    }
                }
            });

            // Preview: show first 20 inputs as mask bytes
            let m = &macro_state.library.macros[idx];
            let preview_len = m.inputs.len().min(20);
            let preview: String = m.inputs[..preview_len]
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");
            let suffix = if m.inputs.len() > 20 { " ..." } else { "" };
            ui.label(format!("Preview: {}{}", preview, suffix));
        }
    }
}

fn push_log(log: &mut Vec<String>, msg: &str) {
    let ts = chrono::Local::now().format("%H:%M:%S");
    log.push(format!("[{}] {}", ts, msg));
}
