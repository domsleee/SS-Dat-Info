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
pub fn capture_from_state(
    state: &tas_shared::TasSharedState,
    label: &str,
) -> Recording {
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
