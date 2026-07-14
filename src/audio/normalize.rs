/// Normalize audio amplitude to range [-1.0, 1.0]
pub fn normalize_audio(signal: &mut [f32]) {
    
    if signal.is_empty() {
        return;
    }

    // Find maximum absolute amplitude.
    let max_amplitude = signal
        .iter()
        .max_by(|bin1, bin2| bin1.abs()partial_cmp(&bin2.abs()).unwrap())
        .unwrap();

    // let max_amplitude = signal
    //     .iter()
    //     .map(|sample| sample.abs())
    //     .fold(0.0, f32::max);

    // Avoid division by zero for silent audio.
    if max_amplitude == 0.0 {
        return;
    }

    // Scale samples.
    signal.iter_mut().map(|&sample| sample / max_amplitude).collect();
    // for sample in signal.iter_mut() {
    //     *sample /= max_amplitude;
    // }
}