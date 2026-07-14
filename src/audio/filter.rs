use std::f32::consts::PI;

/// Apply a simple low-pass filter in place.
fn low_pass(
    signal: &mut [f32],
    sample_rate: u32,
    cutoff: f32,
) {
    if signal.is_empty() {
        return;
    }

    let dt = 1.0 / sample_rate as f32;
    let rc = 1.0 / (2.0 * PI * cutoff);
    let alpha = dt / (rc + dt);

    let mut previous = signal[0];

    for sample in signal.iter_mut() {
        previous += alpha * (*sample - previous);
        *sample = previous;
    }
}

/// Apply a simple high-pass filter in place.
fn high_pass(
    signal: &mut [f32],
    sample_rate: u32,
    cutoff: f32,
) {
    if signal.is_empty() {
        return;
    }

    let dt = 1.0 / sample_rate as f32;
    let rc = 1.0 / (2.0 * PI * cutoff);
    let alpha = rc / (rc + dt);

    let mut previous_input = signal[0];
    let mut previous_output = signal[0];

    for sample in signal.iter_mut() {
        let current_input = *sample;

        let current_output =
            alpha * (previous_output + current_input - previous_input);

        *sample = current_output;

        previous_input = current_input;
        previous_output = current_output;
    }
}

pub fn filter_audio(
    signal: &mut [f32],
    sample_rate: u32,
) {
    // Remove very low-frequency rumble.
    high_pass(signal, sample_rate, 80.0);

    // Remove high-frequency content/noise.
    low_pass(signal, sample_rate, 4_000.0);
}