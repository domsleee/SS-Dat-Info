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
                        push_log(
                            log,
                            &format!("Loaded {} macros from {}", count, path.display()),
                        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static MACRO_COUNTER: AtomicU32 = AtomicU32::new(0);

    fn zeroed_state() -> Box<TasSharedState> {
        tas_shared::zeroed_boxed()
    }

    fn unique_temp_path(prefix: &str, ext: &str) -> std::path::PathBuf {
        let id = MACRO_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("{}_{}_{}.{}", prefix, std::process::id(), id, ext))
    }

    // ===== MacroState =====

    #[test]
    fn macro_state_defaults() {
        let ms = MacroState::new();
        assert!(ms.library.macros.is_empty());
        assert!(ms.library_path.is_none());
        assert!(ms.extract_name.is_empty());
        assert_eq!(ms.extract_from, "0");
        assert!(ms.extract_to.is_empty());
        assert_eq!(ms.paste_at, "0");
        assert!(ms.selected.is_none());
    }

    // ===== MacroLibrary =====

    #[test]
    fn macro_library_default_is_empty() {
        let lib = MacroLibrary::default();
        assert!(lib.macros.is_empty());
    }

    #[test]
    fn extract_from_recording_valid_range() {
        let mut state = zeroed_state();
        state.recorded_count = 10;
        for i in 0..10 {
            state.input_log[i] = i as u8;
        }

        let m = MacroLibrary::extract_from_recording(&state, "test", 2, 5).unwrap();
        assert_eq!(m.name, "test");
        assert_eq!(m.inputs, vec![2, 3, 4]);
    }

    #[test]
    fn extract_from_recording_full_range() {
        let mut state = zeroed_state();
        state.recorded_count = 5;
        for i in 0..5 {
            state.input_log[i] = 0x04;
        }

        let m = MacroLibrary::extract_from_recording(&state, "full", 0, 5).unwrap();
        assert_eq!(m.inputs.len(), 5);
    }

    #[test]
    fn extract_from_recording_invalid_ranges() {
        let mut state = zeroed_state();
        state.recorded_count = 5;

        // from >= count
        assert!(MacroLibrary::extract_from_recording(&state, "x", 5, 6).is_none());
        // to > count
        assert!(MacroLibrary::extract_from_recording(&state, "x", 0, 6).is_none());
        // from >= to
        assert!(MacroLibrary::extract_from_recording(&state, "x", 3, 3).is_none());
        assert!(MacroLibrary::extract_from_recording(&state, "x", 4, 2).is_none());
        // empty recording
        let empty = zeroed_state();
        assert!(MacroLibrary::extract_from_recording(&empty, "x", 0, 0).is_none());
    }

    #[test]
    fn paste_into_recording_basic() {
        let mut state = zeroed_state();
        state.recorded_count = 10;

        let m = InputMacro {
            name: "turn".into(),
            inputs: vec![0x01, 0x01, 0x02, 0x02], // L L R R
        };

        let len = MacroLibrary::paste_into_recording(&mut state, &m, 3).unwrap();
        assert_eq!(len, 4);
        assert_eq!(state.input_log[3], 0x01);
        assert_eq!(state.input_log[4], 0x01);
        assert_eq!(state.input_log[5], 0x02);
        assert_eq!(state.input_log[6], 0x02);
        assert_eq!(state.recorded_count, 10); // unchanged, paste within range
    }

    #[test]
    fn paste_extends_recorded_count() {
        let mut state = zeroed_state();
        state.recorded_count = 5;

        let m = InputMacro {
            name: "extend".into(),
            inputs: vec![0x04; 10],
        };

        let len = MacroLibrary::paste_into_recording(&mut state, &m, 3).unwrap();
        assert_eq!(len, 10);
        assert_eq!(state.recorded_count, 13); // 3 + 10
    }

    #[test]
    fn paste_past_max_ticks_errors() {
        let mut state = zeroed_state();
        let m = InputMacro {
            name: "huge".into(),
            inputs: vec![0x04; 100],
        };

        let result = MacroLibrary::paste_into_recording(&mut state, &m, TAS_MAX_TICKS - 50);
        assert!(result.is_err());
    }

    #[test]
    fn paste_at_zero_with_empty_recording() {
        let mut state = zeroed_state();
        assert_eq!(state.recorded_count, 0);

        let m = InputMacro {
            name: "from_zero".into(),
            inputs: vec![0x01, 0x02, 0x04],
        };

        MacroLibrary::paste_into_recording(&mut state, &m, 0).unwrap();
        assert_eq!(state.recorded_count, 3);
        assert_eq!(state.input_log[0], 0x01);
        assert_eq!(state.input_log[1], 0x02);
        assert_eq!(state.input_log[2], 0x04);
    }

    // ===== Library save/load round-trip =====

    #[test]
    fn library_save_load_round_trip() {
        let lib = MacroLibrary {
            macros: vec![
                InputMacro {
                    name: "left_turn".into(),
                    inputs: vec![0x01, 0x01, 0x00],
                },
                InputMacro {
                    name: "right_turn".into(),
                    inputs: vec![0x02, 0x02, 0x00],
                },
            ],
        };

        let path = unique_temp_path("macro_lib", "tasmacro");
        lib.save_to_file(&path).unwrap();

        let loaded = MacroLibrary::load_from_file(&path).unwrap();
        assert_eq!(loaded.macros.len(), 2);
        assert_eq!(loaded.macros[0].name, "left_turn");
        assert_eq!(loaded.macros[0].inputs, vec![0x01, 0x01, 0x00]);
        assert_eq!(loaded.macros[1].name, "right_turn");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn library_load_invalid_json_errors() {
        let path = unique_temp_path("macro_bad", "tasmacro");
        std::fs::write(&path, "not json").unwrap();
        assert!(MacroLibrary::load_from_file(&path).is_err());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn library_load_nonexistent_errors() {
        let path = unique_temp_path("macro_nonexist", "tasmacro");
        assert!(MacroLibrary::load_from_file(&path).is_err());
    }

    // ===== Extract + Paste integration =====

    #[test]
    fn extract_then_paste_round_trip() {
        let mut state = zeroed_state();
        state.recorded_count = 20;
        for i in 0..20 {
            state.input_log[i] = (i * 3) as u8;
        }

        // Extract ticks 5..10
        let m = MacroLibrary::extract_from_recording(&state, "slice", 5, 10).unwrap();
        assert_eq!(m.inputs.len(), 5);

        // Paste at tick 15
        MacroLibrary::paste_into_recording(&mut state, &m, 15).unwrap();
        assert_eq!(state.recorded_count, 20);

        // Verify pasted data matches extracted
        for i in 0..5 {
            assert_eq!(state.input_log[15 + i], state.input_log[5 + i]);
        }
    }
}
