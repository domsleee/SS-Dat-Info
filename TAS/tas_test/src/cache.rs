//! Recording cache file I/O.
//!
//! Format: JSON header + binary input log + coordinate data.
//! Compatible with the egui UI save/load format.

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

/// Recording metadata header.
#[derive(Debug, Clone)]
pub struct RecordingHeader {
    pub version: u32,
    pub tick_count: u32,
    pub inject_mode: u32,
    pub force_fixed_tick: u32,
    pub force_direct: u32,
    pub label: String,
}

/// A saved recording: header + input log + coordinates.
#[derive(Debug, Clone)]
pub struct Recording {
    pub header: RecordingHeader,
    pub input_log: Vec<u8>,
    pub rec_coords: Vec<[f32; 3]>,
}

impl Recording {
    /// Save recording to a binary file.
    ///
    /// Format:
    ///   [4 bytes] magic "TAS\0"
    ///   [4 bytes] version (u32 LE)
    ///   [4 bytes] tick_count (u32 LE)
    ///   [4 bytes] inject_mode (u32 LE)
    ///   [4 bytes] force_fixed_tick (u32 LE)
    ///   [4 bytes] force_direct (u32 LE)
    ///   [4 bytes] label_len (u32 LE)
    ///   [label_len bytes] label (UTF-8)
    ///   [tick_count bytes] input_log
    ///   [tick_count * 12 bytes] rec_coords (3xf32 LE per tick)
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut f = fs::File::create(path)?;

        // Magic
        f.write_all(b"TAS\0")?;

        let n = self.header.tick_count;
        let label_bytes = self.header.label.as_bytes();

        for val in [
            self.header.version,
            n,
            self.header.inject_mode,
            self.header.force_fixed_tick,
            self.header.force_direct,
            label_bytes.len() as u32,
        ] {
            f.write_all(&val.to_le_bytes())?;
        }
        f.write_all(label_bytes)?;

        // Input log
        let log_len = n as usize;
        if self.input_log.len() >= log_len {
            f.write_all(&self.input_log[..log_len])?;
        } else {
            f.write_all(&self.input_log)?;
            // Pad with zeros
            let pad = vec![0u8; log_len - self.input_log.len()];
            f.write_all(&pad)?;
        }

        // Coords
        for i in 0..log_len {
            if i < self.rec_coords.len() {
                for &v in &self.rec_coords[i] {
                    f.write_all(&v.to_le_bytes())?;
                }
            } else {
                f.write_all(&[0u8; 12])?;
            }
        }

        f.flush()?;
        Ok(())
    }

    /// Load recording from a binary file.
    pub fn load(path: &Path) -> io::Result<Self> {
        let mut f = fs::File::open(path)?;

        let mut magic = [0u8; 4];
        f.read_exact(&mut magic)?;
        if &magic != b"TAS\0" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "bad magic"));
        }

        let mut buf4 = [0u8; 4];
        let read_u32 = |f: &mut fs::File, buf: &mut [u8; 4]| -> io::Result<u32> {
            f.read_exact(buf)?;
            Ok(u32::from_le_bytes(*buf))
        };

        let version = read_u32(&mut f, &mut buf4)?;
        let tick_count = read_u32(&mut f, &mut buf4)?;
        let inject_mode = read_u32(&mut f, &mut buf4)?;
        let force_fixed_tick = read_u32(&mut f, &mut buf4)?;
        let force_direct = read_u32(&mut f, &mut buf4)?;
        let label_len = read_u32(&mut f, &mut buf4)?;

        let mut label_buf = vec![0u8; label_len as usize];
        f.read_exact(&mut label_buf)?;
        let label = String::from_utf8_lossy(&label_buf).into_owned();

        let n = tick_count as usize;
        let mut input_log = vec![0u8; n];
        f.read_exact(&mut input_log)?;

        let mut rec_coords = vec![[0.0f32; 3]; n];
        for coord in rec_coords.iter_mut() {
            for val in coord.iter_mut() {
                f.read_exact(&mut buf4)?;
                *val = f32::from_le_bytes(buf4);
            }
        }

        Ok(Recording {
            header: RecordingHeader {
                version,
                tick_count,
                inject_mode,
                force_fixed_tick,
                force_direct,
                label,
            },
            input_log,
            rec_coords,
        })
    }
}

/// Extract a recording from current shared memory state.
pub fn capture_from_state(state: &tas_shared::TasSharedState, label: &str) -> Recording {
    let n = state.recorded_count as usize;
    let input_log = state.input_log[..n].to_vec();
    let rec_coords: Vec<[f32; 3]> = state.rec_coords[..n].to_vec();

    Recording {
        header: RecordingHeader {
            version: state.version,
            tick_count: state.recorded_count,
            inject_mode: state.inject_mode,
            force_fixed_tick: state.force_fixed_tick,
            force_direct: state.force_direct,
            label: label.to_string(),
        },
        input_log,
        rec_coords,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

    fn unique_temp_path(prefix: &str, ext: &str) -> std::path::PathBuf {
        let id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("{}_{}_{}.{}", prefix, std::process::id(), id, ext))
    }

    fn make_recording(tick_count: u32, label: &str) -> Recording {
        let mut input_log = vec![0u8; tick_count as usize];
        let mut rec_coords = vec![[0.0f32; 3]; tick_count as usize];
        for i in 0..tick_count as usize {
            input_log[i] = (i % 4) as u8;
            rec_coords[i] = [i as f32, (i as f32) * 0.5, (i as f32) * 2.0];
        }
        Recording {
            header: RecordingHeader {
                version: 4,
                tick_count,
                inject_mode: 6,
                force_fixed_tick: 0,
                force_direct: 2,
                label: label.to_string(),
            },
            input_log,
            rec_coords,
        }
    }

    #[test]
    fn save_load_round_trip() {
        let rec = make_recording(100, "test_round_trip");
        let path = unique_temp_path("cache_rt", "tas");

        rec.save(&path).expect("save failed");
        let loaded = Recording::load(&path).expect("load failed");

        assert_eq!(loaded.header.version, 4);
        assert_eq!(loaded.header.tick_count, 100);
        assert_eq!(loaded.header.inject_mode, 6);
        assert_eq!(loaded.header.force_fixed_tick, 0);
        assert_eq!(loaded.header.force_direct, 2);
        assert_eq!(loaded.header.label, "test_round_trip");
        assert_eq!(loaded.input_log, rec.input_log);
        assert_eq!(loaded.rec_coords, rec.rec_coords);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_load_empty_recording() {
        let rec = make_recording(0, "empty");
        let path = unique_temp_path("cache_empty", "tas");

        rec.save(&path).expect("save failed");
        let loaded = Recording::load(&path).expect("load failed");

        assert_eq!(loaded.header.tick_count, 0);
        assert!(loaded.input_log.is_empty());
        assert!(loaded.rec_coords.is_empty());

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_load_unicode_label() {
        let rec = make_recording(10, "日本語テスト 🎿");
        let path = unique_temp_path("cache_unicode", "tas");

        rec.save(&path).expect("save failed");
        let loaded = Recording::load(&path).expect("load failed");
        assert_eq!(loaded.header.label, "日本語テスト 🎿");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn load_bad_magic_fails() {
        let path = unique_temp_path("cache_badmagic", "tas");
        std::fs::write(&path, b"BAD\0xxxx").expect("write failed");

        let result = Recording::load(&path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("bad magic"));

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_creates_parent_dirs() {
        let rec = make_recording(5, "nested");
        let base = unique_temp_path("cache_nested", "dir");
        let path = base.join("sub").join("deep.tas");

        rec.save(&path).expect("save failed");
        assert!(path.exists());

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn capture_from_state_extracts_correctly() {
        let mut state = tas_shared::zeroed_boxed();
        state.version = 4;
        state.recorded_count = 3;
        state.inject_mode = 6;
        state.force_fixed_tick = 0;
        state.force_direct = 2;
        state.input_log[0] = 0x01;
        state.input_log[1] = 0x02;
        state.input_log[2] = 0x03;
        state.rec_coords[0] = [1.0, 2.0, 3.0];
        state.rec_coords[1] = [4.0, 5.0, 6.0];
        state.rec_coords[2] = [7.0, 8.0, 9.0];

        let rec = capture_from_state(&state, "captured");
        assert_eq!(rec.header.tick_count, 3);
        assert_eq!(rec.header.label, "captured");
        assert_eq!(rec.input_log, vec![0x01, 0x02, 0x03]);
        assert_eq!(rec.rec_coords[2], [7.0, 8.0, 9.0]);
    }
}
