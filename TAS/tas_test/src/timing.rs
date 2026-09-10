//! Timing gates require every planned trial, not a median of surviving runs.
pub fn complete_median(samples: &mut [f64], expected: usize) -> Option<f64> {
    if expected == 0
        || samples.len() != expected
        || samples.iter().any(|s| !s.is_finite() || *s <= 0.0)
    {
        return None;
    }
    samples.sort_by(f64::total_cmp);
    let mid = samples.len() / 2;
    Some(if samples.len().is_multiple_of(2) {
        samples[mid - 1] / 2.0 + samples[mid] / 2.0
    } else {
        samples[mid]
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_a_complete_finite_trial_set_can_pass() {
        for count in 0..3 {
            assert_eq!(complete_median(&mut vec![1.0; count], 3), None);
        }
        assert_eq!(complete_median(&mut [3.0, 1.0, 2.0], 3), Some(2.0));
        assert_eq!(complete_median(&mut [1.0, 3.0], 2), Some(2.0));
        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert_eq!(complete_median(&mut [1.0, invalid, 2.0], 3), None);
        }
    }
}
