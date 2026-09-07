//! The `.tasrec` recording-file framing shared by the UI serializer
//! (`tas_ui::recording::RecordingFile`) and the harness serializer
//! (`tas_test::replay`).
//!
//! Layout: `[u32le meta_len][JSON metadata][input_log bytes][f32le xyz …]`.
//! This crate owns the byte layout, every length bound, and atomic writing.
//! Metadata *schemas* stay with the callers (the UI stamps identity +
//! segments; the harness preserves unknown fields raw). Whether coordinates
//! are required is an explicit caller decision (`require_coords`), not a
//! property of the format.

use std::path::Path;

use tas_shared::TAS_MAX_TICKS;

/// Bytes per recorded tick: 1 input byte + 3 little-endian f32s.
pub const BLOB_BYTES_PER_TICK: usize = 1 + 3 * 4;
/// Upper bound on the JSON metadata header. Real headers are pure metadata
/// (tens of KB with segments); anything past this is treated as corrupt
/// rather than read into memory.
pub const MAX_TASREC_METADATA_BYTES: usize = 1024 * 1024;

/// Hard upper bound on a whole file: a full-length recording. Any file
pub const MAX_TASREC_BYTES: u64 =
    (4 + MAX_TASREC_METADATA_BYTES + TAS_MAX_TICKS * BLOB_BYTES_PER_TICK) as u64;

/// A decoded file: the raw JSON header plus the body for `count` ticks.
pub struct Decoded {
    /// Byte length of the JSON header (`bytes[4..4 + meta_len]` holds it).
    pub meta_len: usize,
    pub input_log: Vec<u8>,
    pub rec_coords: Vec<[f32; 3]>,
    /// False when the file ends after the input log (a legacy recording):
    /// inputs are usable, coordinates are zeroed.
    pub has_coords: bool,
}

/// Read a whole file with the size cap enforced before allocation.
pub fn read_bounded(path: &Path) -> Result<Vec<u8>, String> {
    let reported = std::fs::metadata(path).map_err(|e| format!("{}", e))?.len();
    if reported > MAX_TASREC_BYTES {
        return Err(format!(
            "Recording file too large: {} bytes (maximum {})",
            reported, MAX_TASREC_BYTES
        ));
    }
    let mut data = Vec::with_capacity(reported as usize);
    use std::io::Read;
    std::fs::File::open(path)
        .map_err(|e| format!("{}", e))?
        .take(MAX_TASREC_BYTES + 1)
        .read_to_end(&mut data)
        .map_err(|e| format!("{}", e))?;
    if data.len() as u64 > MAX_TASREC_BYTES {
        return Err(format!(
            "Recording file too large: more than {} bytes",
            MAX_TASREC_BYTES
        ));
    }
    Ok(data)
}

/// Byte length of the JSON header, bounds-checked. The caller parses
/// `bytes[4..4 + meta_len]` with its own metadata schema.
pub fn header_meta_len(bytes: &[u8]) -> Result<usize, String> {
    if bytes.len() < 4 {
        return Err("File too small".into());
    }
    let meta_len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
    if meta_len > MAX_TASREC_METADATA_BYTES {
        return Err(format!("Recording metadata too large: {} bytes", meta_len));
    }
    if bytes.len() < 4 + meta_len {
        return Err("Truncated metadata".into());
    }
    Ok(meta_len)
}

/// Decode the body for `count` ticks after a `header_meta_len` header.
/// `require_coords` is the caller's rule: the live harness replays
/// coordinates and refuses files without them; the UI loads legacy
/// recordings and zeroes the missing block instead.
pub fn decode_body(
    bytes: &[u8],
    meta_len: usize,
    count: usize,
    require_coords: bool,
) -> Result<Decoded, String> {
    if count > TAS_MAX_TICKS {
        return Err(format!("Recording too long: {} ticks", count));
    }
    let input_start = 4 + meta_len;
    let input_end = input_start
        .checked_add(count)
        .ok_or_else(|| "Recording input length overflow".to_string())?;
    if bytes.len() < input_end {
        return Err("Truncated input log".into());
    }
    let input_log = bytes[input_start..input_end].to_vec();

    let coords_start = input_end;
    let coords_size = count * 3 * 4;
    let coords_end = coords_start
        .checked_add(coords_size)
        .ok_or_else(|| "Recording coordinate length overflow".to_string())?;
    // Zero the FULL coordinate block first: a legacy/minimal file with no
    // coord block must not leave a previous recording's coords behind to
    // corrupt CONT start-matching, drift analysis, or a re-save.
    let mut rec_coords = vec![[0.0f32; 3]; count];
    let has_coords = bytes.len() >= coords_end;
    if has_coords {
        let mut offset = coords_start;
        for coord in rec_coords.iter_mut() {
            for val in coord.iter_mut() {
                *val = f32::from_le_bytes([
                    bytes[offset],
                    bytes[offset + 1],
                    bytes[offset + 2],
                    bytes[offset + 3],
                ]);
                offset += 4;
            }
        }
    } else if require_coords {
        return Err("Truncated rec_coords".into());
    }
    Ok(Decoded {
        meta_len,
        input_log,
        rec_coords,
        has_coords,
    })
}

/// Frame one file: `[meta_len][meta_json][inputs][coords]`. Length mismatch
/// is the caller's bug, reported here so both serializers share the check.
pub fn encode(
    meta_json: &[u8],
    input_log: &[u8],
    rec_coords: &[[f32; 3]],
) -> Result<Vec<u8>, String> {
    if input_log.len() != rec_coords.len() {
        return Err(format!(
            "input/coord length mismatch: {} inputs vs {} coords",
            input_log.len(),
            rec_coords.len()
        ));
    }
    let mut data =
        Vec::with_capacity(4 + meta_json.len() + input_log.len() + rec_coords.len() * 12);
    data.extend_from_slice(&(meta_json.len() as u32).to_le_bytes());
    data.extend_from_slice(meta_json);
    data.extend_from_slice(input_log);
    for coord in rec_coords {
        for value in coord {
            data.extend_from_slice(&value.to_le_bytes());
        }
    }
    Ok(data)
}
/// Write bytes atomically (temp + fsync + rename): a crash leaves the old
/// file or the new one, never a half-written recording. `create_new` refuses
/// to clobber a temp file a crashed previous save left behind.
pub fn save_atomic(path: &Path, data: &[u8]) -> Result<(), String> {
    use std::io::Write;
    let tmp = path.with_extension("tasrec.tmp");
    let result = (|| {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp)
            .map_err(|e| format!("failed to create {}: {}", tmp.display(), e))?;
        file.write_all(data)
            .map_err(|e| format!("failed to write {}: {}", tmp.display(), e))?;
        file.sync_all()
            .map_err(|e| format!("failed to flush {}: {}", tmp.display(), e))?;
        drop(file);
        std::fs::rename(&tmp, path)
            .map_err(|e| format!("failed to publish recording {}: {}", path.display(), e))
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file_for(count: usize, with_coords: bool) -> Vec<u8> {
        let meta = b"{\"recorded_count\":0}";
        let mut v = (meta.len() as u32).to_le_bytes().to_vec();
        v.extend_from_slice(meta);
        v.extend((0..count).map(|i| i as u8));
        if with_coords {
            for i in 0..count {
                for c in [i as f32, 0.0, 1.0] {
                    v.extend_from_slice(&c.to_le_bytes());
                }
            }
        }
        v
    }

    #[test]
    fn rejects_short_and_oversized_headers() {
        assert!(header_meta_len(&[0, 1, 2]).is_err());
        let mut huge = (MAX_TASREC_METADATA_BYTES as u32 + 1)
            .to_le_bytes()
            .to_vec();
        huge.extend_from_slice(b"{}");
        assert!(header_meta_len(&huge).is_err());
        let mut truncated = (8u32).to_le_bytes().to_vec();
        truncated.extend_from_slice(b"{\"a\"");
        assert!(header_meta_len(&truncated).is_err());
        assert!(header_meta_len(&file_for(1, true)).is_ok());
    }

    #[test]
    fn body_round_trips_and_requires_coords_explicitly() {
        let full = file_for(4, true);
        let len = header_meta_len(&full).unwrap();
        let body = decode_body(&full, len, 4, true).unwrap();
        assert!(body.has_coords);
        assert_eq!(body.input_log, vec![0, 1, 2, 3]);
        assert_eq!(body.rec_coords[3], [3.0, 0.0, 1.0]);

        let legacy = file_for(4, false);
        let len = header_meta_len(&legacy).unwrap();
        assert!(decode_body(&legacy, len, 4, true).is_err());
        let body = decode_body(&legacy, len, 4, false).unwrap();
        assert!(!body.has_coords);
        assert_eq!(body.rec_coords, vec![[0.0; 3]; 4]);
        assert_eq!(body.input_log, vec![0, 1, 2, 3]);
    }

    #[test]
    fn encode_rejects_mismatched_lengths() {
        assert!(encode(b"{}", &[0, 1], &[[0.0; 3]]).is_err());
        let bytes = encode(b"{}", &[0, 1], &[[0.0; 3], [1.0; 3]]).unwrap();
        let len = header_meta_len(&bytes).unwrap();
        let body = decode_body(&bytes, len, 2, true).unwrap();
        assert_eq!(body.input_log, vec![0, 1]);
    }

    #[test]
    fn count_bounds_are_checked() {
        use tas_shared::TAS_MAX_TICKS;
        let bytes = file_for(2, true);
        let len = header_meta_len(&bytes).unwrap();
        assert!(decode_body(&bytes, len, TAS_MAX_TICKS + 1, true).is_err());
        assert!(decode_body(&bytes, len, 999_999_999, true).is_err());
    }

    #[test]
    fn atomic_save_round_trips() {
        let dir = std::env::temp_dir().join(format!(
            "tas_codec_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("a.tasrec");
        let bytes = encode(b"{\"recorded_count\":2}", &[7, 8], &[[0.0; 3], [1.0; 3]]).unwrap();
        save_atomic(&path, &bytes).unwrap();
        let back = read_bounded(&path).unwrap();
        assert_eq!(back, bytes);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
