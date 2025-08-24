use crate::ai::{EntropyPulse, FeedbackSignal, ProbabilisticSearch};
use rand_distr::{Distribution, Normal};

// A simple Gaussian distribution to model our belief about a parameter.
pub struct Gaussian {
    pub mean: f64,
    pub std_dev: f64,
}

// This struct holds the AI's entire belief system about the target pulse.

pub struct FrequencyBeliefSpace {
    pub frequency: Gaussian,
    pub amplitude: Gaussian,
    // Add these two fields for memory
    pub best_guess: EntropyPulse,
    pub best_feedback: FeedbackSignal,
}

impl FrequencyBeliefSpace {
    pub fn new(initial_freq: f64, initial_amp: f64) -> Self {
        let initial_guess = EntropyPulse {
            frequency: initial_freq,
            amplitude: initial_amp,
            waveform: "sine".to_string(),
        };

        Self {
            frequency: Gaussian {
                mean: initial_freq,
                std_dev: 50.0,
            }, // Widen initial search
            amplitude: Gaussian {
                mean: initial_amp,
                std_dev: 1.0,
            },
            // Initialize memory with a "worst-case" scenario
            best_guess: initial_guess,
           // Initialize memory with the largest possible error.
            best_feedback: FeedbackSignal {
                correlation_strength: f64::MAX
            },
        }
    }
}

impl ProbabilisticSearch for FrequencyBeliefSpace {
    // propose_best_guess remains the same...
    fn propose_best_guess(&self) -> EntropyPulse {
        let freq_dist = Normal::new(self.frequency.mean, self.frequency.std_dev).unwrap();
        let amp_dist = Normal::new(self.amplitude.mean, self.amplitude.std_dev).unwrap();
        let mut rng = rand::rng();
        EntropyPulse {
            frequency: freq_dist.sample(&mut rng),
            amplitude: amp_dist.sample(&mut rng),
            waveform: "sine".to_string(),
        }
    }

    fn update(&mut self, feedback: &FeedbackSignal, last_guess: &EntropyPulse) {
    // The core logic now checks for a SMALLER error.
    if feedback.correlation_strength < self.best_feedback.correlation_strength {
        // We found a better guess! Update our memory.
        self.best_feedback = feedback.clone();
        self.best_guess = last_guess.clone();
    }


        // Step 2: Use a more aggressive learning rate to move the belief faster.
        let learning_rate = 0.15; // Increased from 0.1
        self.frequency.mean =
            (1.0 - learning_rate) * self.frequency.mean + learning_rate * self.best_guess.frequency;

        // Step 3: Use a more aggressive and consistent reduction in exploration.
        // This forces the AI to "zoom in" and commit to a peak.
        self.frequency.std_dev *= 0.9; // Shrinks by 0.5% each step

        // Ensure std_dev doesn't become too small.
        if self.frequency.std_dev < 0.01 {
            self.frequency.std_dev = 0.01;
        }
    }
}
