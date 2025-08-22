//! The main binary for the Slow AI experiment.
//! This crate orchestrates the learning loop, combining the Bayesian search
//! with the quantum simulation defined in the EntangledSystem.

mod entangled_system;

// Use shared types from the core API crate
use fa_core::state::{EntropyPulse, FeedbackSignal};
use fa_core::traits::ProbabilisticSearch;

// Use the concrete implementation of the learning engine
use fa_bayes::FrequencyBeliefSpace;

// Use the simulation engine
use entangled_system::EntangledSystem;

fn run_short_experiment(pulse: &EntropyPulse) -> FeedbackSignal {
    // 1. Create a fresh, entangled system.
    let mut system = EntangledSystem::new(10, 10);

    // 2. Apply the pulse.
    system.apply_pulse(pulse);

    // 3. Measure the result and return it directly.
    system.measure_correlation()
}

// The AI's learning loop is implemented in the `main` function below.
// It sets up the belief space, runs multiple iterations of guessing,
// and updates its beliefs based on feedback from the real experiment.
fn main() {
    println!("\n--- Slow AI Entanglement Recoherence Experiment ---");

    for _k in 1..=5 {
        // 1. Setup the Bayesian learning agent.
        //    We'll start with a wide search range for the frequency.
        let mut belief_space = FrequencyBeliefSpace::new(0.0, 1.0); // Start guess at 0 Hz, amplitude 1.0

        // We need to define what the "correct" frequency is for our simulation.
        // In a real experiment, this would be unknown. Here, we define it so we can
        // see if the AI can discover it. Let's choose a non-obvious number.
        const TARGET_RESONANT_FREQUENCY: f64 = std::f64::consts::PI / 2.0; // approx 1.57

        // 2. The main learning loop.
        for _i in 1..=10_000 {
            // Run for 100 iterations to give the AI time to learn
            // a. The AI proposes its best guess based on its current beliefs.
            let guess_pulse = belief_space.propose_best_guess();

            // b. We run the physics simulation with this guess.
            //    To make the simulation work, we need to compare the AI's guess
            //    to our hidden "resonant frequency". A pulse at the resonant frequency
            //    should cause minimal decoherence.
            //    We simulate this by adjusting the pulse's frequency relative to the target.
            //    A perfect guess (guess_pulse.frequency == TARGET_RESONANT_FREQUENCY)
            //    results in a frequency of 0, causing no phase rotation.
            let effective_pulse = EntropyPulse {
                frequency: guess_pulse.frequency - TARGET_RESONANT_FREQUENCY,
                amplitude: guess_pulse.amplitude,
                waveform: guess_pulse.waveform.clone(),
            };

            let feedback = run_short_experiment(&effective_pulse);

            // c. The AI updates its beliefs based on the feedback from the real experiment.
            belief_space.update(&feedback, &guess_pulse);
        }

        let final_guess = belief_space.propose_best_guess();

        let metric_fract = (final_guess.frequency / (2.0 * std::f64::consts::PI))
            .fract()
            .abs();

        let mut category = "Unknown";
        let tolerance = 0.05; // How close it needs to be

        if (metric_fract - 0.25).abs() < tolerance {
            category = "Peak (Coherent Resonance)";
        } else if (metric_fract - 0.75).abs() < tolerance {
            category = "Saddle Point (Stable Decoherence)";
        }

        println!(
            "guess: {:.4} rad/s, \tmetric = {:.4}, \tcategory = {}",
            final_guess.frequency, metric_fract, category
        );
    }
}
