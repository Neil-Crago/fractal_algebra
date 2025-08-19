#[derive(Clone, Debug, PartialEq)]
pub struct FractalSignature {
    pub total_amplitude: f32,
    pub average_phase: f32,
    pub entropy: f32,
    pub edge_count: usize,
    pub depth_range: (u32, u32),
}

impl FractalSignature {
    pub fn distance(&self, other: &Self) -> f32 {
        let amp_diff = (self.total_amplitude - other.total_amplitude).abs();
        let phase_diff = (self.average_phase - other.average_phase).abs();
        let entropy_diff = (self.entropy - other.entropy).abs();
        let count_diff = (self.edge_count as i32 - other.edge_count as i32).abs() as f32;

        amp_diff + phase_diff + entropy_diff + count_diff
    }
}

impl FractalSignature {
    pub fn is_symmetric(&self) -> bool {
        self.average_phase.abs() < 1e-3 || (self.average_phase - std::f32::consts::PI).abs() < 1e-3
    }
}

impl FractalSignature {
    pub fn normalized(&self) -> Self {
        let amp = self.total_amplitude.max(1e-6);
        FractalSignature {
            total_amplitude: 1.0,
            average_phase: self.average_phase,
            entropy: self.entropy / amp,
            edge_count: self.edge_count,
            depth_range: self.depth_range,
        }
    }
}

impl FractalSignature {
    pub fn from_units(units: &[crate::resonance::SemanticUnit]) -> Self {
        FractalSignature {
            total_amplitude: units.iter().map(|u| u.depth as f32).sum::<f32>() as f32,
            average_phase: units.iter().map(|u| u.phase as f32).sum::<f32>() / units.len() as f32,
            entropy: units.iter().map(|u| u.depth as f32).sum::<f32>() / units.len() as f32,
            edge_count: units.len(),
            depth_range: (
                units
                    .iter()
                    .map(|u| u.depth)
                    .min()
                    .map(|v| v as u32)
                    .unwrap_or(0),
                units
                    .iter()
                    .map(|u| u.depth)
                    .max()
                    .map(|v| v as u32)
                    .unwrap_or(0),
            ),
        }
    }
}
