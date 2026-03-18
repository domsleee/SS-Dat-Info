use egui_plot::{Bar, BarChart, Plot};
use tas_shared::{input_bits, TasSharedState};

/// Input analysis panel: histogram, transition density, symmetry check.
pub fn show(ui: &mut egui::Ui, state: &TasSharedState) {
    let count = state.recorded_count as usize;
    if count == 0 {
        ui.label("No recording data — record or load a .tasrec first.");
        return;
    }

    let log = &state.input_log[..count];

    egui::CollapsingHeader::new("Input Histogram")
        .default_open(true)
        .show(ui, |ui| {
            show_histogram(ui, log);
        });

    egui::CollapsingHeader::new("Transition Density")
        .default_open(true)
        .show(ui, |ui| {
            show_transition_density(ui, log);
        });

    egui::CollapsingHeader::new("Symmetry Check")
        .default_open(true)
        .show(ui, |ui| {
            show_symmetry(ui, log);
        });

    egui::CollapsingHeader::new("Trajectory Stats")
        .default_open(false)
        .show(ui, |ui| {
            show_trajectory_stats(ui, state, count);
        });
}

/// Bar chart showing ticks held for each input button.
fn show_histogram(ui: &mut egui::Ui, log: &[u8]) {
    let mut counts = [0u32; 6];
    for &mask in log {
        for (i, &(bit, _, _)) in input_bits::ALL.iter().enumerate() {
            if mask & bit != 0 {
                counts[i] += 1;
            }
        }
    }

    let total = log.len() as f32;
    let bars: Vec<Bar> = input_bits::ALL
        .iter()
        .enumerate()
        .map(|(i, &(_, short, _))| {
            Bar::new(i as f64, counts[i] as f64)
                .name(short)
                .width(0.6)
        })
        .collect();

    let chart = BarChart::new(bars).color(egui::Color32::from_rgb(100, 160, 255));

    Plot::new("input_histogram")
        .height(120.0)
        .allow_drag(false)
        .allow_zoom(false)
        .allow_scroll(false)
        .show_axes([false, true])
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(chart);
        });

    // Text summary
    ui.horizontal_wrapped(|ui| {
        for (i, &(_, short, _)) in input_bits::ALL.iter().enumerate() {
            let pct = if total > 0.0 {
                counts[i] as f32 / total * 100.0
            } else {
                0.0
            };
            ui.label(format!("{}: {} ({:.1}%)", short, counts[i], pct));
        }
    });
}

/// Shows where transitions (input changes) cluster in the recording.
fn show_transition_density(ui: &mut egui::Ui, log: &[u8]) {
    // Count transitions
    let mut transitions: Vec<usize> = Vec::new();
    let mut prev: u8 = 0;
    for (i, &mask) in log.iter().enumerate() {
        if mask != prev {
            transitions.push(i);
            prev = mask;
        }
    }

    ui.label(format!(
        "Total transitions: {} across {} ticks",
        transitions.len(),
        log.len()
    ));

    if transitions.is_empty() {
        ui.label("No input transitions detected.");
        return;
    }

    // Compute density: divide into 50 equal bins, count transitions per bin
    let bin_count = 50usize.min(log.len());
    let bin_size = log.len() as f64 / bin_count as f64;
    let mut bins = vec![0u32; bin_count];
    for &t in &transitions {
        let bin = ((t as f64 / bin_size) as usize).min(bin_count - 1);
        bins[bin] += 1;
    }

    let bars: Vec<Bar> = bins
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let tick = (i as f64 * bin_size) as u32;
            Bar::new(tick as f64, c as f64).width(bin_size * 0.8)
        })
        .collect();

    let chart = BarChart::new(bars).color(egui::Color32::from_rgb(255, 180, 80));

    Plot::new("transition_density")
        .height(100.0)
        .allow_drag(false)
        .allow_zoom(false)
        .allow_scroll(false)
        .show_axes([true, true])
        .x_axis_label("Tick")
        .y_axis_label("Transitions")
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(chart);
        });

    // Gap analysis
    if transitions.len() >= 2 {
        let gaps: Vec<usize> = transitions.windows(2).map(|w| w[1] - w[0]).collect();
        let min_gap = gaps.iter().min().copied().unwrap_or(0);
        let max_gap = gaps.iter().max().copied().unwrap_or(0);
        let avg_gap = gaps.iter().sum::<usize>() as f64 / gaps.len() as f64;
        ui.label(format!(
            "Gap stats: min={} max={} avg={:.1} ticks",
            min_gap, max_gap, avg_gap
        ));
    }
}

/// Checks if L/R steering is roughly symmetric (balanced turns).
fn show_symmetry(ui: &mut egui::Ui, log: &[u8]) {
    let mut left_ticks = 0u32;
    let mut right_ticks = 0u32;
    let mut up_ticks = 0u32;
    let mut down_ticks = 0u32;

    // Track L/R segments for pattern analysis
    let mut segments: Vec<(char, u32)> = Vec::new(); // (direction, duration)
    let mut current_dir: Option<char> = None;
    let mut current_len: u32 = 0;

    for &mask in log {
        let l = mask & input_bits::LEFT != 0;
        let r = mask & input_bits::RIGHT != 0;
        let u = mask & input_bits::UP != 0;
        let d = mask & input_bits::DOWN != 0;

        if l {
            left_ticks += 1;
        }
        if r {
            right_ticks += 1;
        }
        if u {
            up_ticks += 1;
        }
        if d {
            down_ticks += 1;
        }

        // Track L/R segments
        let dir = if l && !r {
            Some('L')
        } else if r && !l {
            Some('R')
        } else {
            None
        };

        if dir == current_dir {
            current_len += 1;
        } else {
            if let Some(d) = current_dir {
                if current_len > 0 {
                    segments.push((d, current_len));
                }
            }
            current_dir = dir;
            current_len = 1;
        }
    }
    if let Some(d) = current_dir {
        if current_len > 0 {
            segments.push((d, current_len));
        }
    }

    // L/R balance
    let lr_total = left_ticks + right_ticks;
    let lr_ratio = if lr_total > 0 {
        left_ticks as f32 / lr_total as f32
    } else {
        0.5
    };
    let lr_balance = (lr_ratio - 0.5).abs() * 200.0; // 0 = perfect, 100 = all one side

    let balance_color = if lr_balance < 10.0 {
        egui::Color32::from_rgb(80, 200, 80)
    } else if lr_balance < 30.0 {
        egui::Color32::YELLOW
    } else {
        egui::Color32::from_rgb(255, 80, 80)
    };

    ui.horizontal(|ui| {
        ui.label("L/R balance:");
        ui.colored_label(
            balance_color,
            format!(
                "L={} R={} ({:.0}% skew)",
                left_ticks, right_ticks, lr_balance
            ),
        );
    });

    // U/D balance
    let ud_total = up_ticks + down_ticks;
    if ud_total > 0 {
        let ud_ratio = up_ticks as f32 / ud_total as f32;
        ui.label(format!(
            "U/D balance: U={} D={} ({:.0}% U)",
            up_ticks,
            down_ticks,
            ud_ratio * 100.0
        ));
    }

    // Segment analysis
    if !segments.is_empty() {
        let left_segs: Vec<u32> = segments.iter().filter(|s| s.0 == 'L').map(|s| s.1).collect();
        let right_segs: Vec<u32> = segments.iter().filter(|s| s.0 == 'R').map(|s| s.1).collect();

        ui.label(format!(
            "Segments: {} L (avg {:.1} ticks), {} R (avg {:.1} ticks)",
            left_segs.len(),
            if left_segs.is_empty() {
                0.0
            } else {
                left_segs.iter().sum::<u32>() as f64 / left_segs.len() as f64
            },
            right_segs.len(),
            if right_segs.is_empty() {
                0.0
            } else {
                right_segs.iter().sum::<u32>() as f64 / right_segs.len() as f64
            },
        ));
    }
}

/// Trajectory statistics: total distance, sector splits, turn radii.
fn show_trajectory_stats(ui: &mut egui::Ui, state: &TasSharedState, count: usize) {
    if count < 2 {
        ui.label("Need at least 2 ticks for trajectory stats.");
        return;
    }

    let coords = &state.rec_coords[..count];

    // Total distance (XZ plane)
    let mut total_dist: f64 = 0.0;
    for i in 1..count {
        let dx = (coords[i][0] - coords[i - 1][0]) as f64;
        let dz = (coords[i][2] - coords[i - 1][2]) as f64;
        total_dist += (dx * dx + dz * dz).sqrt();
    }

    // Start/end displacement
    let start_x = coords[0][0] as f64;
    let start_z = coords[0][2] as f64;
    let end_x = coords[count - 1][0] as f64;
    let end_z = coords[count - 1][2] as f64;
    let displacement = ((end_x - start_x).powi(2) + (end_z - start_z).powi(2)).sqrt();

    // Straightness ratio (1.0 = perfectly straight)
    let straightness = if total_dist > 0.0 {
        displacement / total_dist
    } else {
        0.0
    };

    ui.label(format!("Path length: {:.1} units", total_dist));
    ui.label(format!("Displacement: {:.1} units", displacement));
    ui.label(format!("Straightness: {:.3} (1.0 = straight line)", straightness));

    // Sector splits (divide path into 4 equal parts)
    let sector_count = 4usize.min(count);
    let sector_size = count / sector_count;
    if sector_size > 1 {
        ui.label("Sector splits:");
        for s in 0..sector_count {
            let start = s * sector_size;
            let end = if s == sector_count - 1 {
                count
            } else {
                (s + 1) * sector_size
            };
            let mut sector_dist: f64 = 0.0;
            for i in (start + 1)..end {
                let dx = (coords[i][0] - coords[i - 1][0]) as f64;
                let dz = (coords[i][2] - coords[i - 1][2]) as f64;
                sector_dist += (dx * dx + dz * dz).sqrt();
            }
            ui.label(format!(
                "  S{}: ticks {}-{}, dist {:.1}",
                s + 1,
                start,
                end - 1,
                sector_dist
            ));
        }
    }
}
