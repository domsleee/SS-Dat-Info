//! Headless whole-frame tests: the real `TasApp`, one `update_frame` per
//! egui pass, against the fake game on a private mapping, with a fake clock,
//! keyboard, game process and file pickers. Input goes through real widgets
//! (found by their probe names after a frame) and real key events.
//!
//! Time only moves when the harness says so: each frame advances the world
//! by `FRAME` first, and the app's own bounded waits advance it from
//! inside the frame, so a STOP can be acknowledged during the wait for it.

use std::cell::{RefCell, RefMut};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

use eframe::egui::{self, Event, Key, Modifiers, PointerButton, Pos2, RawInput, Rect};
use tas_shared::input_bits::{LEFT, RIGHT, UP};
use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient, TasSharedState};

use super::fake_game::{self, FakeGame, FE, VE};
use crate::{host, probe, settings, TasApp};

mod buffer_ops;
mod cont;
mod editing;
mod refusal;
mod scheduling;

/// The fake Supreme.exe.
const GAME_PID: u32 = 4242;
/// Game time each frame runs first.
const FRAME: Duration = Duration::from_millis(16);

/// The fake world the app's host reads: clock, game, keyboard, process,
/// and the next answers of the file pickers.
pub struct World {
    base: Instant,
    elapsed: Duration,
    pub game: FakeGame,
    /// Virtual keys held right now (global shortcut polling).
    pub keys_down: Vec<i32>,
    pub foreground: Option<u32>,
    pub load_path: Option<PathBuf>,
    pub save_path: Option<PathBuf>,
    dir: PathBuf,
}

impl World {
    fn advance(&mut self, d: Duration) {
        self.elapsed += d;
        self.game.advance(d);
    }
}

struct FakeHost(Rc<RefCell<World>>);

impl host::Host for FakeHost {
    fn now(&self) -> Instant {
        let world = self.0.borrow();
        world.base + world.elapsed
    }
    fn wait(&self, d: Duration) {
        self.0.borrow_mut().advance(d);
    }
    fn key_is_down(&self, vk: i32) -> bool {
        self.0.borrow().keys_down.contains(&vk)
    }
    fn game_pid(&self) -> Option<u32> {
        Some(GAME_PID)
    }
    fn foreground_pid(&self) -> Option<u32> {
        self.0.borrow().foreground
    }
    fn set_dark_title_bar(&self, _title: &str) {}
    fn pick_load_path(&self, _level_id: u32) -> Option<PathBuf> {
        self.0.borrow_mut().load_path.take()
    }
    fn pick_save_path(&self, _level: Option<&str>, _default_name: &str) -> Option<PathBuf> {
        self.0.borrow_mut().save_path.take()
    }
    fn open_file(&self, path: &std::path::Path) {
        panic!("unexpected launch of {}", path.display());
    }
    fn open_folder(&self, path: &std::path::Path) -> Result<(), String> {
        panic!("unexpected launch of {}", path.display());
    }
    fn diagnostics_dir(&self) -> PathBuf {
        self.0.borrow().dir.join("diagnostics")
    }
}

/// The standard take's keys, by REC index: R 80-100, L 110-120, R 130-170,
/// U 180-200.
pub fn standard_keys(i: u32) -> u8 {
    match i {
        80..100 | 130..170 => RIGHT,
        110..120 => LEFT,
        180..200 => UP,
        _ => 0,
    }
}

pub struct Harness {
    pub app: TasApp,
    pub world: Rc<RefCell<World>>,
    ctx: egui::Context,
}

impl Drop for Harness {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.world.borrow().dir);
    }
}

impl Harness {
    /// The app connected to a fake FE race, a few frames in: the race is
    /// seen ticking, so the transport is enabled. (egui hit-tests against
    /// the previous frame, so a button enabled only this frame cannot be
    /// clicked yet.)
    pub fn new() -> Self {
        static RUN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let dir = std::env::temp_dir().join(format!(
            "tas_ui_e2e_{}_{}",
            std::process::id(),
            RUN.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let mut shared = TasSharedMemoryClient::new_test_mapping();
        let state: *mut TasSharedState = shared.state_mut();
        let world = Rc::new(RefCell::new(World {
            base: Instant::now(),
            elapsed: Duration::ZERO,
            game: FakeGame::new(state),
            keys_down: Vec::new(),
            foreground: None,
            load_path: None,
            save_path: None,
            dir,
        }));
        let settings = settings::Settings {
            show_history: true,
            cont_catchup_speed: 4.0,
            history_cap: 64,
            ..Default::default()
        };
        let mut app = TasApp::blank_with_host(settings, Box::new(FakeHost(world.clone())));
        app.conn.shared = Some(shared);
        let mut h = Self {
            app,
            world,
            ctx: egui::Context::default(),
        };
        h.frames(3);
        assert!(h.app.arming_allowed());
        h
    }

    /// A harness holding the standard take, recorded through REC.
    pub fn with_standard_take() -> Self {
        let mut h = Self::new();
        h.record(standard_keys, 230);
        h
    }

    pub fn state(&self) -> &TasSharedState {
        self.app.conn.shared.as_ref().unwrap().state()
    }

    pub fn game(&self) -> RefMut<'_, FakeGame> {
        RefMut::map(self.world.borrow_mut(), |w| &mut w.game)
    }

    pub fn world(&self) -> RefMut<'_, World> {
        self.world.borrow_mut()
    }

    pub fn mode(&self) -> TasMode {
        self.state().mode_enum()
    }

    /// The take in the buffer: its input.
    pub fn input(&self) -> Vec<u8> {
        let s = self.state();
        s.input_log[..s.recorded_count as usize].to_vec()
    }

    pub fn taken(&self) -> Vec<TasCommand> {
        self.game().taken.clone()
    }

    pub fn frame(&mut self) {
        self.frame_with(Vec::new(), Modifiers::NONE);
    }

    pub fn frames(&mut self, n: usize) {
        for _ in 0..n {
            self.frame();
        }
    }

    /// One frame: the world runs `FRAME`, then the app runs on `events`.
    pub fn frame_with(&mut self, events: Vec<Event>, modifiers: Modifiers) {
        self.world.borrow_mut().advance(FRAME);
        probe::clear();
        let now = self.world.borrow().elapsed.as_secs_f64();
        let mut input = RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(1100.0, 680.0))),
            time: Some(now),
            modifiers,
            events,
            ..Default::default()
        };
        input
            .viewports
            .entry(egui::ViewportId::ROOT)
            .or_default()
            .native_pixels_per_point = Some(1.0);
        let app = &mut self.app;
        let _ = self.ctx.run(input, |ctx| app.update_frame(ctx));
    }

    /// Frames until `done`, at most `max`.
    pub fn run_until(&mut self, what: &str, max: usize, done: impl Fn(&Self) -> bool) {
        for _ in 0..max {
            if done(self) {
                return;
            }
            self.frame();
        }
        assert!(done(self), "{what}: not after {max} frames\n{}", self.log());
    }

    /// Game time with no UI frame at all, until `done`.
    pub fn run_game_until(&mut self, what: &str, done: impl Fn(&TasSharedState) -> bool) {
        for _ in 0..100_000 {
            if done(self.state()) {
                return;
            }
            self.world.borrow_mut().advance(Duration::from_millis(1));
        }
        panic!("{what}: never happened");
    }

    /// Where `name` was drawn last frame.
    pub fn rect(&self, name: &str) -> Rect {
        probe::rect(name).unwrap_or_else(|| panic!("no widget {name} in the last frame"))
    }

    /// Press on the widget `name` in one frame, release in the next.
    pub fn click(&mut self, name: &str) {
        let at = self.rect(name).center();
        self.click_at(at);
    }

    pub fn click_at(&mut self, at: Pos2) {
        let button = |pressed| Event::PointerButton {
            pos: at,
            button: PointerButton::Primary,
            pressed,
            modifiers: Modifiers::NONE,
        };
        self.frame_with(vec![Event::PointerMoved(at), button(true)], Modifiers::NONE);
        self.frame_with(vec![button(false)], Modifiers::NONE);
    }

    /// Press and release `key` in one frame.
    pub fn key(&mut self, key: Key, modifiers: Modifiers) {
        self.frame_with(key_events(key, modifiers), modifiers);
    }

    /// Type `text`, one character per frame.
    pub fn type_text(&mut self, text: &str) {
        for c in text.chars() {
            self.frame_with(vec![Event::Text(c.to_string())], Modifiers::NONE);
        }
    }

    /// Clear a focused text field.
    pub fn clear_field(&mut self) {
        self.key(Key::End, Modifiers::NONE);
        for _ in 0..8 {
            self.key(Key::Backspace, Modifiers::NONE);
        }
    }

    pub fn log(&self) -> String {
        self.app.log_lines.lines().join("\n")
    }

    pub fn log_count(&self, needle: &str) -> usize {
        self.app
            .log_lines
            .lines()
            .iter()
            .filter(|l| l.contains(needle))
            .count()
    }

    /// Record through the REC button with the fake keyboard on `keys`, and
    /// STOP once `ticks` are recorded.
    pub fn record(&mut self, keys: fn(u32) -> u8, ticks: u32) {
        self.game().keyboard = Box::new(keys);
        self.click("transport.rec");
        self.run_until("REC records", 1000, |h| {
            h.mode() == TasMode::Rec && h.state().recorded_count >= ticks
        });
        self.click("transport.stop");
        self.run_until("REC stops", 100, |h| {
            h.mode() == TasMode::Off && h.app.session.active.is_none()
        });
    }

    /// Click PLAY and run until the replay is over and its cycle is done.
    pub fn play(&mut self) {
        self.click("transport.play");
        self.run_until("PLAY runs", 2000, |h| {
            !h.app.transport.is_running() && h.mode() == TasMode::Off
        });
    }

    /// The take's gate (first moving tick).
    pub fn rec_gate(&self) -> usize {
        let s = self.state();
        tas_shared::align::detect_first_moving(&s.rec_coords, s.recorded_count).unwrap() as usize
    }

    /// Whether the last replay reproduced the take's whole trajectory, gate
    /// to gate, bit for bit.
    pub fn replay_matches_take(&self) -> bool {
        let s = self.state();
        let rec_gate = self.rec_gate();
        let live_gate = *self.game().live_gates.last().unwrap() as usize;
        let n = s.recorded_count as usize - rec_gate;
        s.rec_coords[rec_gate..rec_gate + n] == s.play_coords[live_gate..live_gate + n]
    }

    /// Whether any cycle aborted (the watcher's verdict included).
    pub fn aborted(&self) -> bool {
        self.log_count("aborted") > 0 || self.log_count("gave up") > 0
    }
}

pub fn key_events(key: Key, modifiers: Modifiers) -> Vec<Event> {
    [true, false]
        .map(|pressed| Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers,
        })
        .to_vec()
}

/// A take recorded through the UI replays exactly through the UI: the fake
/// and the harness agree with the real controller.
#[test]
fn a_recorded_take_replays_exactly() {
    let mut h = Harness::with_standard_take();
    let take = h.input();
    assert!(take.len() >= 230);
    assert_eq!(take[85], RIGHT);
    assert_eq!(
        h.app.history.list.entries().last().unwrap().end_tick,
        take.len() as u32
    );
    h.play();
    assert!(!h.aborted(), "{}", h.log());
    assert!(h.replay_matches_take());
    assert_eq!(h.input(), take);
    assert_eq!(
        h.taken(),
        [
            // REC.
            TasCommand::Stop,
            TasCommand::Restart,
            TasCommand::ArmRec,
            TasCommand::Stop,
            // PLAY.
            TasCommand::StopForRestart,
            TasCommand::Restart,
            TasCommand::ArmPlay,
        ]
    );
    assert_eq!(h.state().owner_pid, 0, "the cycle released its ownership");
}

/// A replay that differs from the take aborts the cycle and leaves a report:
/// the watcher is live in this harness.
#[test]
fn a_replay_that_differs_aborts() {
    let mut h = Harness::with_standard_take();
    h.game().perturb_at = Some(50);
    h.play();
    assert_eq!(h.log_count("PLAY aborted: replay diverged at gate+50"), 1);
    let reports = std::fs::read_dir(h.world().dir.join("diagnostics")).unwrap();
    assert_eq!(reports.count(), 1);
}

/// The UI reads the identity the fake publishes, through the same
/// seqlocked readers as the real DLL's.
#[test]
fn the_ui_reads_the_identity_the_fake_publishes() {
    let mut h = Harness::new();
    let live = |h: &Harness| {
        let history = &h.app.history.list;
        (
            history.live_level().map(str::to_string),
            history.live_rider().map(str::to_string),
            history.live_physics().map(str::to_string),
        )
    };
    let stamps = |level: &str, rider: &str, physics: &str| {
        (Some(level.into()), Some(rider.into()), Some(physics.into()))
    };
    assert_eq!(live(&h), stamps("FE", "Vincent · regular", "OpenGL/53-bit"));
    let mut game = h.game();
    game.set_level(VE);
    game.set_rider(
        tas_shared::TAS_CHARACTER_KEITH,
        tas_shared::TAS_STANCE_GOOFY,
    );
    game.set_fpu_control_word(0x007F);
    drop(game);
    h.frame();
    assert_eq!(live(&h), stamps("VE", "Keith · goofy", "OpenGL/24-bit"));
    h.game().unresolve_level();
    h.frame();
    assert!(h.app.history.list.level_is_resolving());
}
