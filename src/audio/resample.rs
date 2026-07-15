use crate::audio::decode::DecodedAudio;

pub const SAMPLING_RATE: u32 = 1u32 << 14;

pub fn resample_audio(audio: &DecodedAudio) -> Vec<f32> {
    let signal = &audio.signal;
    let original_rate = audio.sample_rate;

    // No resampling needed.
    if original_rate == SAMPLING_RATE {
        return signal.to_vec();
    }

    let ratio = SAMPLING_RATE as f64 / original_rate as f64;

    let new_length = (signal.len() as f64 * ratio).round() as usize;

    let mut resampled = Vec::with_capacity(new_length);

    for i in 0..new_length {
        // Find corresponding position in original signal.
        let original_position = i as f64 / ratio;

        let left_index = original_position.floor() as usize;
        let right_index = left_index + 1;

        // Stop if at the end.
        if right_index >= signal.len() {
            resampled.push(signal[left_index]);
            continue;
        }

        // Distance between the two original samples.
        let fraction = original_position - left_index as f64;

        // Linear interpolation.
        let sample =
            signal[left_index] * (1.0 - fraction as f32)
            + signal[right_index] * fraction as f32;

        resampled.push(sample);
    }

    resampled
}