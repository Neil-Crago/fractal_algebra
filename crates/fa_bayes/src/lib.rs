use fa_core::state::{EntropyPulse, FeedbackSignal};
use fa_core::traits::ProbabilisticSearch;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

// A simple Gaussian distribution to model our belief about a parameter.
pub struct Gaussian {
    pub mean: f64,
    pub std_dev: f64,
}

// This struct holds the AI's entire belief system about the target pulse.
// In fa_bayes/src/lib.rs

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
            best_feedback: FeedbackSignal {
                correlation_strength: -1.0,
                phase_alignment_error: f64::MAX,
            },
        }
    }
}

impl ProbabilisticSearch for FrequencyBeliefSpace {
    // propose_best_guess remains the same...
    fn propose_best_guess(&self) -> EntropyPulse {
        let freq_dist = Normal::new(self.frequency.mean, self.frequency.std_dev).unwrap();
        let amp_dist = Normal::new(self.amplitude.mean, self.amplitude.std_dev).unwrap();
        let mut rng = thread_rng();
        EntropyPulse {
            frequency: freq_dist.sample(&mut rng),
            amplitude: amp_dist.sample(&mut rng),
            waveform: "sine".to_string(),
        }
    }

    // This is the new, smarter update logic
    fn update(&mut self, feedback: &FeedbackSignal, last_guess: &EntropyPulse) {
        // Step 1: Check if the new guess is better than our best-remembered guess.
        if feedback.correlation_strength > self.best_feedback.correlation_strength {
            // If it is, update our memory.
            self.best_feedback = feedback.clone(); // You may need to derive(Clone) on FeedbackSignal
            self.best_guess = last_guess.clone(); // You may need to derive(Clone) on EntropyPulse
        }

        // Step 2: Always nudge the mean towards the best-known position.
        let learning_rate = 0.1;
        let error = self.best_guess.frequency - self.frequency.mean;
        self.frequency.mean += learning_rate * error;

        // Step 3: Gradually reduce uncertainty (exploitation) as we find better solutions.
        // Shrink std_dev faster if correlation is high.
        //self.frequency.std_dev *= 1.0 - (learning_rate * self.best_feedback.correlation_strength * 0.1);
        self.frequency.std_dev *=
            1.0 - (learning_rate * self.best_feedback.correlation_strength * 0.5); // Use 0.5
        // Ensure std_dev doesn't become too small, to prevent getting stuck.
        if self.frequency.std_dev < 1.0 {
            self.frequency.std_dev = 1.0;
        }
    }
}
