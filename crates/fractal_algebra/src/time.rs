/// Represents the state of a point in our fractal universe.
/// The dimensionality is encoded directly in the type of the coordinate.
pub enum SpacetimeCoordinate {
    /// A 1-dimensional reality, defined by a single sequence point.
    Sequential(f64),

    /// A 4-dimensional reality, defined by a point in Minkowski spacetime.
    /// We use a tuple: (time, x, y, z).
    Minkowski(f64, f64, f64, f64),
    // We could even add others later, like a 2D complex plane.
    // ComplexPlane(Complex<f64>),
}

/// The core struct now simply holds the current coordinate state.
pub struct FractalSpacetime {
    pub coordinate: SpacetimeCoordinate,
}

pub trait Evolvable {
    type EnergyOutput;
    fn transition_to_4d(&mut self) -> Self::EnergyOutput;
    fn tick(&mut self);
}

impl Evolvable for FractalSpacetime {
    type EnergyOutput = Vec<f64>; // Represents the primordial waveform

    fn transition_to_4d(&mut self) -> Self::EnergyOutput {
        // The transition only happens if we are in a 1D state.
        if let SpacetimeCoordinate::Sequential(initial_state) = self.coordinate {
            // The new 4D state starts at time 0, at the origin.
            self.coordinate = SpacetimeCoordinate::Minkowski(0.0, 0.0, 0.0, 0.0);

            // The energy released is a function of the 1D state we came from.
            // This is where the primordial waveform is generated.
            let energy = vec![initial_state.sin(), initial_state.cos()]; // Placeholder
            return energy;
        }
        // If we're already in 4D, do nothing and release no energy.
        vec![]
    }

    fn tick(&mut self) {
        match &mut self.coordinate {
            SpacetimeCoordinate::Sequential(time) => *time += 1.0,
            SpacetimeCoordinate::Minkowski(time, _, _, _) => *time += 1.0,
        }
    }
}
