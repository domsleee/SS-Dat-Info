//! The physics-mode stamp recordings carry: which renderer plugin sr.dll
//! loaded and the x87 precision it leaves the game thread at.

/// Renderer plugin ids published in `TasSharedState::renderer_id` (v41):
/// which `srDD_*.dll` sr.dll loaded.
pub const TAS_RENDERER_UNKNOWN: u32 = 0;
pub const TAS_RENDERER_DIRECTX6: u32 = 1;
pub const TAS_RENDERER_DIRECTX7: u32 = 2;
pub const TAS_RENDERER_OPENGL: u32 = 3;
pub const TAS_RENDERER_GLIDE3X: u32 = 4;
pub const TAS_RENDERER_SOFTWARE2: u32 = 5;

pub fn renderer_name(id: u32) -> &'static str {
    match id {
        TAS_RENDERER_DIRECTX6 => "DirectX6",
        TAS_RENDERER_DIRECTX7 => "DirectX7",
        TAS_RENDERER_OPENGL => "OpenGL",
        TAS_RENDERER_GLIDE3X => "Glide3x",
        TAS_RENDERER_SOFTWARE2 => "Software2",
        _ => "unknown",
    }
}

pub fn renderer_id_from_name(name: &str) -> u32 {
    match name {
        "DirectX6" => TAS_RENDERER_DIRECTX6,
        "DirectX7" => TAS_RENDERER_DIRECTX7,
        "OpenGL" => TAS_RENDERER_OPENGL,
        "Glide3x" => TAS_RENDERER_GLIDE3X,
        "Software2" => TAS_RENDERER_SOFTWARE2,
        _ => TAS_RENDERER_UNKNOWN,
    }
}

/// Precision-control field (bits 8-9) of an x87 control word: 24, 53 or 64.
/// 0 for an unsampled (zero) word or the reserved encoding.
pub fn fpu_precision_bits(control_word: u32) -> u32 {
    if control_word == 0 {
        return 0;
    }
    match (control_word >> 8) & 3 {
        0 => 24,
        2 => 53,
        3 => 64,
        _ => 0,
    }
}

/// Canonical physics-mode stamp, e.g. `OpenGL/53-bit` or `DirectX6/24-bit`.
///
/// Supreme.exe asks for 24-bit x87 precision, DirectX 6/7 keep it and the
/// OpenGL/Software2 path runs at 53-bit, so identical inputs give different
/// trajectories per renderer (wiki: "Why are replays sometimes 0.01s shorter
/// than expected?"). Recordings and history entries carry this stamp; a
/// mismatch with the live mode means a replay cannot be bit-exact. `None`
/// until the DLL has sampled the game thread.
pub fn physics_mode_label(renderer_id: u32, fpu_control_word: u32) -> Option<String> {
    let bits = fpu_precision_bits(fpu_control_word);
    if bits == 0 && renderer_id == TAS_RENDERER_UNKNOWN {
        return None;
    }
    let renderer = renderer_name(renderer_id);
    Some(if bits == 0 {
        renderer.to_string()
    } else {
        format!("{}/{}-bit", renderer, bits)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two control words the wiki documents: DirectX 6/7 leave the game
    /// at 24-bit, OpenGL/Software2 at 53-bit. The stamp must tell them apart
    /// and stay `None` until the game thread has been sampled.
    #[test]
    fn physics_mode_stamp_distinguishes_renderers() {
        assert_eq!(fpu_precision_bits(0x007F), 24);
        assert_eq!(fpu_precision_bits(0x027F), 53);
        assert_eq!(fpu_precision_bits(0x037F), 64);
        assert_eq!(fpu_precision_bits(0), 0);
        assert_eq!(
            physics_mode_label(TAS_RENDERER_DIRECTX6, 0x007F).as_deref(),
            Some("DirectX6/24-bit")
        );
        assert_eq!(
            physics_mode_label(TAS_RENDERER_OPENGL, 0x027F).as_deref(),
            Some("OpenGL/53-bit")
        );
        assert_ne!(
            physics_mode_label(TAS_RENDERER_DIRECTX6, 0x007F),
            physics_mode_label(TAS_RENDERER_OPENGL, 0x027F)
        );
        // Renderer known but no cycle yet: name only, so a stamp still exists.
        assert_eq!(
            physics_mode_label(TAS_RENDERER_OPENGL, 0).as_deref(),
            Some("OpenGL")
        );
        assert_eq!(physics_mode_label(TAS_RENDERER_UNKNOWN, 0), None);
        for id in [1, 2, 3, 4, 5] {
            assert_eq!(renderer_id_from_name(renderer_name(id)), id);
        }
    }
}
