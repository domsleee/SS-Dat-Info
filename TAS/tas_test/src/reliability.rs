//! Same-speed repeated REC/PLAY coverage.
pub fn run(iterations: u32, speed: f32) -> bool {
    let passed = crate::cycles::run(&[crate::cycles::Case {
        label: "Reliability",
        rec_speed: speed,
        play_speed: speed,
        tail_ticks: 50,
        iterations,
        require_movement_gates: true,
    }]);
    println!(
        "*** RELIABILITY TEST {} ***",
        if passed { "PASSED" } else { "FAILED" }
    );
    passed
}
