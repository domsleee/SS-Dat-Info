//! Same-speed and cross-speed transparency share the reliability procedure.
pub fn run() -> bool {
    let cases = [(2.0, 2.0), (1.0, 2.0)].map(|(rec_speed, play_speed)| crate::cycles::Case {
        label: "Drift at speed",
        rec_speed,
        play_speed,
        tail_ticks: 100,
        iterations: 1,
        require_movement_gates: false,
    });
    let passed = crate::cycles::run(&cases);
    println!(
        "*** DRIFT-AT-SPEED TEST {} ***",
        if passed { "PASSED" } else { "FAILED" }
    );
    passed
}
