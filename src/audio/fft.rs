use phastft::{r2c_fft_f32};
use crate::audio::resample::SAMPLING_RATE;

pub const VOICE_ERROR_THRESHOLD: u32 = 52;

/// Transform time domain signal into frequency domain
pub fn fft(signal: &[f32]) -> Option<Vec<f32>> {
    
    if signal.is_empty() {
        return None;
    }

    // Square signal increases the desired signal strength and weakens noise.
    let sq_signal: Vec<f32> = signal.iter().map(|&sample| sample * sample).collect();

    // Real and imaginary bins.
    let mut spec_re = vec![0.0; (SAMPLING_RATE / 2 + 1) as usize];
    let mut spec_img = vec![0.0; (SAMPLING_RATE / 2 + 1) as usize];

    r2c_fft_f32(&sq_signal, &mut spec_re, &mut spec_img);

    Some(spec_re)

}