/// Voice match by calculating mean absolute error between the reference and the unknown signal
///  and compare to the threshold.
pub fn mean_absolute_error(
    reference: &[f32],
    input: &[f32],
) -> f32 {

    if reference.is_empty() || input.is_empty() {
        return 0.0;
    }
    
    let mae: f32 = reference.iter().zip(input).map(|(refer, inp)| inp - refer).sum();

    mae / reference.len() as f32

}