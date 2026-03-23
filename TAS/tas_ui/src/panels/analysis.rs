use egui_plot::{Bar, BarChart, Plot};
use tas_shared::{input_bits, TasSharedState};

/// Cached analysis data, invalidated when recorded_count changes.
#[derive(Default)]
pub struct AnalysisCache {
    last_recorded: u32,
    // Histogram
    pub counts: [u32; 6],
    pub total: f32,
    pub histogram_bars: Vec<Bar>,
    // Transition density
    pub transitions: Vec<usize>,
    pub density_bars: Vec<Bar>,
    pub bin_size: f64,
    pub gap_min: usize,
    pub gap_max: usize,
    pub gap_avg: f64,
    // Symmetry
    pub left_ticks: u32,
    pub right_ticks: u32,
    pub up_ticks: u32,
    pub down_ticks: u32,
    pub lr_balance: f32,
    pub left_segs: Vec<u32>,
    pub right_segs: Vec<u32>,
    // Trajectory stats
    pub total_dist: f64,
    pub displacement: f64,
    pub straightness: f64,
    pub sectors: Vec<(usize, usize, f64)>, // (start, end, dist)
}

impl AnalysisCache {
    /// Update cache if recorded_count changed. Returns true if refreshed.
    pub fn refresh(&mut self, state: &TasSharedState) -> bool {
        let recorded = state.recorded_count;
        if recorded == self.last_recorded {
            return false;
        }
        self.last_recorded = recorded;
        let count = recorded as usize;
        if count == 0 {
            return true;
        }
        let log = &state.input_log[..count];

        // Histogram
        self.counts = [0u32; 6];
        for &mask in log {
            for (i, &(bit, _, _)) in input_bits::ALL.iter().enumerate() {
                if mask & bit != 0 {
                    self.counts[i] += 1;
                }
            }
        }
        self.total = log.len() as f32;
        self.histogram_bars = input_bits::ALL
            .iter()
            .enumerate()
            .map(|(i, &(_, short, _))| Bar::new(i as f64, self.counts[i] as f64).name(short).width(0.6))
            .collect();

        // Transition density
        self.transitions.clear();
        let mut prev: u8 = 0;
        for (i, &mask) in log.iter().enumerate() {
            if mask != prev {
                self.transitions.push(i);
                prev = mask;
            }
        }

        let bin_count = 50usize.min(log.len());
        self.bin_size = log.len() as f64 / bin_count as f64;
        let mut bins = vec![0u32; bin_count];
        for &t in &self.transitions {
            let bin = ((t as f64 / self.bin_size) as usize).min(bin_count - 1);
            bins[bin] += 1;
        }
        self.density_bars = bins
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let tick = (i as f64 * self.bin_size) as u32;
                Bar::new(tick as f64, c as f64).width(self.bin_size * 0.8)
            })
            .collect();

        if self.transitions.len() >= 2 {
            let gaps: Vec<usize> = self.transitions.windows(2).map(|w| w[1] - w[0]).collect();
            self.gap_min = gaps.iter().min().copied().unwrap_or(0);
            self.gap_max = gaps.iter().max().copied().unwrap_or(0);
            self.gap_avg = gaps.iter().sum::<usize>() as f64 / gaps.len() as f64;
        } else {
            self.gap_min = 0;
            self.gap_max = 0;
            self.gap_avg = 0.0;
        }

        // Symmetry
        self.left_ticks = 0;
        self.right_ticks = 0;
        self.up_ticks = 0;
        self.down_ticks = 0;
        let mut segments: Vec<(char, u32)> = Vec::new();
        let mut current_dir: Option<char> = None;
        let mut current_len: u32 = 0;

        for &mask in log {
            let l = mask & input_bits::LEFT != 0;
            let r = mask & input_bits::RIGHT != 0;
            let u = mask & input_bits::UP != 0;
            let d = mask & input_bits::DOWN != 0;
            if l { self.left_ticks += 1; }
            if r { self.right_ticks += 1; }
            if u { self.up_ticks += 1; }
            if d { self.down_ticks += 1; }

            let dir = if l && !r { Some('L') } else if r && !l { Some('R') } else { None };
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

        let lr_total = self.left_ticks + self.right_ticks;
        let lr_ratio = if lr_total > 0 { self.left_ticks as f32 / lr_total as f32 } else { 0.5 };
        self.lr_balance = (lr_ratio - 0.5).abs() * 200.0;

        self.left_segs = segments.iter().filter(|s| s.0 == 'L').map(|s| s.1).collect();
        self.right_segs = segments.iter().filter(|s| s.0 == 'R').map(|s| s.1).collect();

        // Trajectory stats
        let coords = &state.rec_coords[..count];
        self.total_dist = 0.0;
        if count >= 2 {
            for i in 1..count {
                let dx = (coords[i][0] - coords[i - 1][0]) as f64;
                let dz = (coords[i][2] - coords[i - 1][2]) as f64;
                self.total_dist += (dx * dx + dz * dz).sqrt();
            }
        }
        let start_x = coords[0][0] as f64;
        let start_z = coords[0][2] as f64;
        let end_x = coords[count - 1][0] as f64;
        let end_z = coords[count - 1][2] as f64;
        self.displacement = ((end_x - start_x).powi(2) + (end_z - start_z).powi(2)).sqrt();
        self.straightness = if self.total_dist > 0.0 { self.displacement / self.total_dist } else { 0.0 };

        let sector_count = 4usize.min(count);
        let sector_size = count / sector_count;
        self.sectors.clear();
        if sector_size > 1 {
            for s in 0..sector_count {
                let start = s * sector_size;
                let end = if s == sector_count - 1 { count } else { (s + 1) * sector_size };
                let mut sector_dist: f64 = 0.0;
                for i in (start + 1)..end {
                    let dx = (coords[i][0] - coords[i - 1][0]) as f64;
                    let dz = (coords[i][2] - coords[i - 1][2]) as f64;
                    sector_dist += (dx * dx + dz * dz).sqrt();
                }
                self.sectors.push((start, end, sector_dist));
            }
        }
        true
    }
}

/// Input analysis panel: histogram, transition density, symmetry check.
pub fn show(ui: &mut egui::Ui, state: &TasSharedState, cache: &mut AnalysisCache) {
    let count = state.recorded_count as usize;
    if count == 0 {
        ui.label("No recording data — record or load a .tasrec first.");
        return;
    }

    cache.refresh(state);

    egui::CollapsingHeader::new("Input Histogram")
        .default_open(true)
        .show(ui, |ui| {
            show_histogram(ui, cache);
        });

    egui::CollapsingHeader::new("Transition Density")
        .default_open(true)
        .show(ui, |ui| {
            show_transition_density(ui, cache);
        });

    egui::CollapsingHeader::new("Symmetry Check")
        .default_open(true)
        .show(ui, |ui| {
            show_symmetry(ui, cache);
        });

    egui::CollapsingHeader::new("Trajectory Stats")
        .default_open(false)
        .show(ui, |ui| {
            show_trajectory_stats(ui, cache, count);
        });
}

fn show_histogram(ui: &mut egui::Ui, cache: &AnalysisCache) {
    let chart = BarChart::new(cache.histogram_bars.clone())
        .color(egui::Color32::from_rgb(100, 160, 255));

    Plot::new("input_histogram")
        .height(120.0)
        .allow_drag(false)
        .allow_zoom(false)
        .allow_scroll(false)
        .show_axes([false, true])
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(chart);
        });

    ui.horizontal_wrapped(|ui| {
        for (i, &(_, short, _)) in input_bits::ALL.iter().enumerate() {
            let pct = if cache.total > 0.0 {
                cache.counts[i] as f32 / cache.total * 100.0
            } else {
                0.0
            };
            ui.label(format!("{}: {} ({:.1}%)", short, cache.counts[i], pct));
        }
    });
}

fn show_transition_density(ui: &mut egui::Ui, cache: &AnalysisCache) {
    ui.label(format!(
        "Total transitions: {} across {} ticks",
        cache.transitions.len(),
        cache.total as usize
    ));

    if cache.transitions.is_empty() {
        ui.label("No input transitions detected.");
        return;
    }

    let chart = BarChart::new(cache.density_bars.clone())
        .color(egui::Color32::from_rgb(255, 180, 80));

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

    if cache.transitions.len() >= 2 {
        ui.label(format!(
            "Gap stats: min={} max={} avg={:.1} ticks",
            cache.gap_min, cache.gap_max, cache.gap_avg
        ));
    }
}

fn show_symmetry(ui: &mut egui::Ui, cache: &AnalysisCache) {
    let balance_color = if cache.lr_balance < 10.0 {
        egui::Color32::from_rgb(80, 200, 80)
    } else if cache.lr_balance < 30.0 {
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
                cache.left_ticks, cache.right_ticks, cache.lr_balance
            ),
        );
    });

    let ud_total = cache.up_ticks + cache.down_ticks;
    if ud_total > 0 {
        let ud_ratio = cache.up_ticks as f32 / ud_total as f32;
        ui.label(format!(
            "U/D balance: U={} D={} ({:.0}% U)",
            cache.up_ticks, cache.down_ticks, ud_ratio * 100.0
        ));
    }

    if !cache.left_segs.is_empty() || !cache.right_segs.is_empty() {
        ui.label(format!(
            "Segments: {} L (avg {:.1} ticks), {} R (avg {:.1} ticks)",
            cache.left_segs.len(),
            if cache.left_segs.is_empty() { 0.0 } else { cache.left_segs.iter().sum::<u32>() as f64 / cache.left_segs.len() as f64 },
            cache.right_segs.len(),
            if cache.right_segs.is_empty() { 0.0 } else { cache.right_segs.iter().sum::<u32>() as f64 / cache.right_segs.len() as f64 },
        ));
    }
}

fn show_trajectory_stats(ui: &mut egui::Ui, cache: &AnalysisCache, count: usize) {
    if count < 2 {
        ui.label("Need at least 2 ticks for trajectory stats.");
        return;
    }

    ui.label(format!("Path length: {:.1} units", cache.total_dist));
    ui.label(format!("Displacement: {:.1} units", cache.displacement));
    ui.label(format!("Straightness: {:.3} (1.0 = straight line)", cache.straightness));

    if !cache.sectors.is_empty() {
        ui.label("Sector splits:");
        for (i, &(start, end, dist)) in cache.sectors.iter().enumerate() {
            ui.label(format!("  S{}: ticks {}-{}, dist {:.1}", i + 1, start, end - 1, dist));
        }
    }
}
