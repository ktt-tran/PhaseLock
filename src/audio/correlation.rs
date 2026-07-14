/// Calculate the maximum normalized cross-correlation
/// between two audio signals.
///
/// Returns a value between -1.0 and 1.0.
/// 1.0 = identical match
pub fn cross_correlation(
    reference: &[f32],
    input: &[f32],
) -> f32 {

    if reference.is_empty() || input.is_empty() {
        return 0.0;
    }

    let (shorter, longer) = if reference.len() <= input.len() {
        (reference, input)
    } else {
        (input, reference)
    };

    let mut best_score = -1.0;

    // Slide shorter signal across longer signal.
    for offset in 0..=(longer.len() - shorter.len()) {

        let window = &longer[offset..offset + shorter.len()];

        let score = normalized_correlation(
            shorter,
            window,
        );

        if score > best_score {
            best_score = score;
        }
    }

    best_score
}


/// Pearson-style normalized correlation.
fn normalized_correlation(
    a: &[f32],
    b: &[f32],
) -> f32 {

    let mut numerator = 0.0;
    let mut a_energy = 0.0;
    let mut b_energy = 0.0;

    for i in 0..a.len() {
        numerator += a[i] * b[i];

        a_energy += a[i] * a[i];
        b_energy += b[i] * b[i];
    }

    let denominator =
        (a_energy * b_energy).sqrt();

    if denominator == 0.0 {
        return 0.0;
    }

    numerator / denominator
}