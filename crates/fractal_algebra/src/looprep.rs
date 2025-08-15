use crate::field::FractalField;

#[derive(Debug)]
pub struct LoopReport {
    pub best_field: FractalField,
    pub best_score: f32,
    pub history: Vec<(FractalField, f32)>,
}
