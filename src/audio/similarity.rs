/// Calculate the cosine similarity
/// between two audio signals.

pub const SPEECH_ERROR_THRESHOLD: u32 = 52;

/// Returns a value between -1.0 and 1.0.
/// 1.0 = identical match
pub fn cosine_similarity(
    reference: &[f32],
    input: &[f32],
) -> f32 {

    if reference.is_empty() || input.is_empty() {
        return 0.0;
    }

    let dot_product: f32 = reference.iter().zip(input).map(|(refer, inp)| refer * inp).sum();
    let norm_reference: f32 = reference.iter().map(|r| r * r).sum::<f32>().sqrt();
    let norm_input: f32 = input.iter().map(|i| i * i).sum::<f32>().sqrt();

    if norm_reference == 0.0 || norm_input == 0.0 {
        return 0.0;
    }

    dot_product / (norm_reference * norm_input)

}